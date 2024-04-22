// #[cfg(debug_assertions)]
#![feature(lazy_cell)]

use cli::DumpMethod;
use emacs_sys::bindings::build_string;
use emacs_sys::bindings::gflags;
use emacs_sys::bindings::globals;
use emacs_sys::bindings::load_seccomp_filters_from_file;
use emacs_sys::bindings::will_dump_with_pdumper_p;
use emacs_sys::bindings::will_dump_with_unexec_p;
use emacs_sys::bindings::COPYRIGHT;
use emacs_sys::bindings::PACKAGE_BUGREPORT;
use emacs_sys::bindings::PACKAGE_VERSION;
use emacs_sys::emacs::initialized;
use emacs_sys::lisp::LispObject;
use std::sync::Mutex;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

use emacs_sys::bindings::main1;
use emacs_sys::bindings::terminate_due_to_signal;
use emacs_sys::bindings::will_dump_p;
use std::os::unix::ffi::OsStringExt;
use std::sync::LazyLock;

// Include the main c_exports file that holds the main rust_init_syms.
// This function calls the other crates init_syms functions which contain
// the generated bindings.
#[cfg(not(test))]
include!(concat!(env!("OUT_DIR"), "/c_exports.rs"));

#[allow(unused_doc_comments)]
#[no_mangle]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* TODO: maybe more thoroughly scrub process environment in order to
    make this use case (loading a dump file in an unexeced emacs)
    possible?  Right now, we assume that things we don't touch are zero-initialized, and in an unexeced Emacs, this assumption
    doesn't hold.  */
    if initialized() {
        panic!("cannot load dump file in unexeced Emacs");
    }

    use libc::c_char;
    use libc::c_int;
    use std::ffi::CString;

    let cli::Args {
        dump_file,
        seccomp,
        temacs,
    } = cli::args();

    /* First, check whether we should apply a seccomp filter.  This
    should come at the very beginning to allow the filter to protect
    the initialization phase.  */
    #[cfg(all(
        have_linux_seccomp_h,
        have_linux_filter_h,
        have_decl_seccomp_set_mode_filter,
        have_decl_seccomp_filter_flag_tsync
    ))]
    if let Some(seccomp_file) = seccomp {
        let file = seccomp_file.into_os_string().into_vec();
        let file = unsafe { CString::from_vec_unchecked(file) };
        let file: *mut c_char = file.into_raw();
        unsafe { load_seccomp_filters_from_file(file) };
    }

    #[cfg(have_pdumper)]
    let mut attempt_load_pdump = false;
    let mut dump_mode: Option<DumpMethod> = None;
    if !initialized() && temacs.is_some() {
        let mode = temacs.unwrap();
        #[cfg(have_unexec)]
        unsafe {
            gflags.set_will_dump_with_unexec_(true);
        };

        #[cfg(have_pdumper)]
        unsafe {
            gflags.set_will_dump_with_pdumper_(true)
        };

        if cfg!(have_unexec) || cfg!(have_pdumper) {
            match mode {
                cli::DumpMethod::Bootstrap => unsafe { gflags.set_will_bootstrap_(true) },
                _ => {}
            }
            unsafe { gflags.set_will_dump_(true) };
            dump_mode = Some(mode);
        }
    } else if temacs.is_some() {
        panic!("--temacs not supported for unexeced emacs");
    } else {
        assert!(temacs.is_none());
        if cfg!(have_unexec) {
            assert!(!initialized());
        }
        #[cfg(have_pdumper)]
        if !initialized() {
            attempt_load_pdump = true;
        }
    }

    #[cfg(have_unexec)]
    if !will_dump_with_unexec_p() {
        gflags.set_will_not_unexec_(true);
    }

    // #ifdef WINDOWSNT
    //   /* Grab our malloc arena space now, before anything important
    //      happens.  This relies on the static heap being needed only in
    //      temacs and only if we are going to dump with unexec.  */
    //   bool use_dynamic_heap = true;
    //   if (temacs)
    //     {
    //       char *temacs_str = NULL, *p;
    //       for (p = argv[0]; (p = strstr (p, "temacs")) != NULL; p++)
    // 	temacs_str = p;
    //       if (temacs_str != NULL
    // 	  && (temacs_str == argv[0] || IS_DIRECTORY_SEP (temacs_str[-1])))
    // 	{
    // 	  /* Note that gflags are set at this point only if we have been
    // 	     called with the --temacs=METHOD option.  We assume here that
    // 	     temacs is always called that way, otherwise the functions
    // 	     that rely on gflags, like will_dump_with_pdumper_p below,
    // 	     will not do their job.  */
    // 	  use_dynamic_heap = will_dump_with_pdumper_p ();
    // 	}
    //     }
    //   init_heap (use_dynamic_heap);
    //   initial_cmdline = GetCommandLine ();
    // #endif

    /* Set global variables used to detect Windows version.  Do this as
    early as possible.  (w32proc.c calls this function as well, but
    the additional call here is harmless.) */
    #[cfg(any(windowsnt, have_ntgui))]
    {
        cache_system_info();
        w32_init_main_thread();
    }
    #[cfg(windowsnt)]
    {
        /* On Windows 9X, we have to load UNICOWS.DLL as early as possible,
        to have non-stub implementations of APIs we need to convert file
        names between UTF-8 and the system's ANSI codepage.  */
        maybe_load_unicows_dll();
        /* Initialize the codepage for file names, needed to decode
        non-ASCII file names during startup.  */
        w32_init_file_name_codepage();
        /* Initialize the startup directory, needed for emacs_wd below.  */
        w32_init_current_directory();
    }

    /* Initialize the Obj C autorelease pool.  */
    #[cfg(have_ns)]
    ns_init_pool();

    // https://stackoverflow.com/questions/34379641/how-do-i-convert-rust-args-into-the-argc-and-argv-c-equivalents/34379937#34379937
    // create a vector of zero terminated strings
    let args = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let mut c_args = args
        .iter()
        .map(|arg| arg.clone().into_raw())
        .collect::<Vec<*mut c_char>>();
    // pass the pointer of the vector's internal buffer to a C function
    let argc = c_args.len() as c_int;
    let argv = c_args.as_mut_ptr();

    fn handle_exit_code(_code: c_int) -> Result<(), Box<dyn std::error::Error>> {
        // TODO
        Ok(())
    }

    let temacs = match temacs {
        Some(temacs) => {
            let temacs = temacs.to_string().into_bytes();
            let temacs = unsafe { CString::from_vec_unchecked(temacs) };
            let temacs: *mut c_char = temacs.into_raw();
            temacs
        }
        None => std::ptr::null_mut(),
    };

    let dump_mode = match dump_mode {
        Some(mode) => {
            let mode = mode.to_string().into_bytes();
            let mode = unsafe { CString::from_vec_unchecked(mode) };
            let mode: *const c_char = mode.into_raw();
            mode
        }
        None => std::ptr::null(),
    };

    unsafe {
        if will_dump_p() {
            #[cfg(have_pdumper)]
            let exit_code = main1(argc, argv, temacs, dump_mode, attempt_load_pdump);
            #[cfg(not(have_pdumper))]
            let exit_code = main1(argc, argv, temacs, dump_mode);
            return handle_exit_code(exit_code);
        }
    }

    // install global collector configured based on EMACSNG_LOG env var.
    // #[cfg(debug_assertions)]
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("EMACSNG_LOG"))
        .init();

    log::trace!("Emacs NG");

    #[cfg(have_pdumper)]
    let exit_code = unsafe { main1(argc, argv, temacs, dump_mode, attempt_load_pdump) };
    #[cfg(not(have_pdumper))]
    let exit_code = unsafe { main1(argc, argv, temacs, dump_mode) };


    // emacs abort
    unsafe { terminate_due_to_signal(libc::SIGABRT, 40) };
    return handle_exit_code(exit_code);
}
