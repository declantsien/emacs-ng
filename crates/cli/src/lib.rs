#![feature(lazy_cell)]
#![feature(stmt_expr_attributes)]

mod dump_method;

use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::LazyLock;

use clap::arg;
use clap::builder::EnumValueParser;
use clap::builder::PossibleValuesParser;
use clap::value_parser;
use clap::Arg;
use clap::ArgAction;
use clap::ArgGroup;
use clap::ColorChoice;
use clap::Command;
pub use dump_method::DumpMethod;

static INITIALIZATION_OPTIONS: LazyLock<Vec<Arg>> = LazyLock::new(|| {
    vec![
        Arg::new("batch")
            .long("batch")
            .action(ArgAction::SetTrue)
            .help("do not do interactive display; implies -q")
            .group("initialization"),
        Arg::new("chdir")
            .long("chdir")
            .value_name("DIR")
            .action(ArgAction::Append)
            .help("change to directory DIR")
            .group("initialization"),
        Arg::new("daemon")
            .long("daemon")
            .visible_alias("bg-daemon")
            .value_name("NAME")
            .action(ArgAction::Append)
            .help("start a (named) server in the background")
            .group("initialization"),
        Arg::new("fg-daemon")
            .long("fg-daemon")
            .value_name("NAME")
            .action(ArgAction::Append)
            .help("start a (named) server in the foreground")
            .group("initialization"),
        Arg::new("debug-init")
            .long("debug-init")
            .action(ArgAction::SetTrue)
            .help("enable Emacs Lisp debugger for init file")
            .group("initialization"),
        Arg::new("display")
            .short('d')
            .long("display")
            .action(ArgAction::SetTrue)
            .help("use display server DISPLAY")
            .group("initialization"),
        #[cfg(have_modules)]
        Arg::new("module-assertions")
            .long("module-assertions")
            .action(ArgAction::SetTrue)
            .help("assert behavior of dynamic modules")
            .group("initialization"),
        #[cfg(have_pdumper)]
        Arg::new("dump-file")
            .long("dump-file")
            .value_name("FILE")
            .help("read dumped state from FILE")
            .group("initialization"),
        #[cfg(have_pdumper)]
        Arg::new("fingerprint")
            .long("fingerprint")
            .action(ArgAction::SetTrue)
            .help("output fingerprint and exit")
            .group("initialization"),
        #[cfg(all(
            have_linux_seccomp_h,
            have_linux_filter_h,
            have_decl_seccomp_set_mode_filter,
            have_decl_seccomp_filter_flag_tsync
        ))]
        Arg::new("seccomp")
            .long("seccomp")
            .value_name("FILE")
            .action(ArgAction::Set)
            .help("read Seccomp BPF filter from FILE")
            .group("initialization"),
        Arg::new("no-build-details")
            .long("no-build-details")
            .action(ArgAction::SetTrue)
            .help("do not add build details such as time stamps")
            .group("initialization"),
        Arg::new("no-desktop")
            .long("no-desktop")
            .action(ArgAction::SetTrue)
            .help("do not load a saved desktop")
            .group("initialization"),
        Arg::new("no-init-file")
            .long("no-init-file")
            .short('q')
            .action(ArgAction::SetTrue)
            .help("load neither ~/.emacs nor default.el")
            .group("initialization"),
        Arg::new("no-loadup")
            .long("no-loadup")
            .visible_alias("nl")
            .action(ArgAction::SetTrue)
            .help("do not load loadup.el into bare Emacs")
            .group("initialization"),
        Arg::new("no-site-file")
            .long("no-site-file")
            .action(ArgAction::SetTrue)
            .help("do not load site-start.el")
            .group("initialization"),
        Arg::new("no-x-resources")
            .long("no-x-resources")
            .action(ArgAction::SetTrue)
            .help("do not load X resources")
            .group("initialization"),
        Arg::new("no-site-lisp")
            .long("no-site-lisp")
            .visible_alias("nsl")
            .action(ArgAction::SetTrue)
            .help("do not add site-lisp directories to load-path")
            .group("initialization"),
        Arg::new("no-splash")
            .long("no-splash")
            .action(ArgAction::SetTrue)
            .help("do not display a splash screen on startup")
            .group("initialization"),
        Arg::new("no-window-system")
            .long("no-window-system")
            .alias("no-windows")
            .visible_alias("nw")
            .action(ArgAction::SetTrue)
            .help("do not communicate with display server, ignoring $DISPLAY")
            .group("initialization"),
        Arg::new("init-directory")
            .long("init-directory")
            .value_name("DIR")
            .action(ArgAction::Append)
            .help("use DIR when looking for the Emacs init files.")
            .group("initialization"),
        Arg::new("quick")
            .long("quick")
            .short('Q')
            .action(ArgAction::SetTrue)
            .help(
                "equivalent to:
-q --no-site-file --no-site-lisp --no-splash
--no-x-resources",
            )
            .group("initialization"),
        Arg::new("script")
            .long("script")
            .value_name("FILE")
            .action(ArgAction::Append)
            .help("run FILE as an Emacs Lisp script\n")
            .group("initialization"),
        Arg::new("x")
            .short('x')
            .action(ArgAction::SetTrue)
            .help(
                "to be used in #!/usr/bin/emacs -x
and has approximately the same meaning
as -Q --script\n",
            )
            .group("initialization"),
        Arg::new("terminal")
            .long("terminal")
            .short('t')
            .action(ArgAction::SetTrue)
            .help("use DEVICE for terminal I/O")
            .group("initialization"),
        Arg::new("user")
            .long("user")
            .short('u')
            .value_name("USER")
            .action(ArgAction::Append)
            .help("load ~USER/.emacs instead of your own")
            .group("initialization"),
    ]
});
static ACTION_OPTIONS: LazyLock<Vec<Arg>> = LazyLock::new(|| {
    vec![
        Arg::new("temacs")
            .long("temacs")
            .value_name("METHOD")
            .value_parser(EnumValueParser::<DumpMethod>::new())
            .action(ArgAction::Set)
            .hide(true),
        Arg::new("file0").value_name("FILE").help("visit FILE"),
        Arg::new("line")
            .value_name("+LINE")
            .help("go to line LINE in next FILE"),
        Arg::new("column")
            .value_name("+LINE:COLUMN")
            .help("go to line LINE, column COLUMN, in next FILE"),
        Arg::new("directory")
            .long("directory")
            .short('L')
            .value_name("DIR")
            .action(ArgAction::Append)
            .help("DIR to load-path (with :DIR, append DIR)")
            .group("actions"),
        Arg::new("eval")
            .long("eval")
            .visible_alias("execute")
            .value_name("EXPR")
            .action(ArgAction::Append)
            .help("Emacs Lisp expression EXPR)")
            .group("actions"),
        Arg::new("file")
            .long("file")
            .visible_alias("find-file")
            .visible_alias("visit")
            .value_name("FILE")
            .action(ArgAction::Append)
            .help("visit FILE")
            .group("actions"),
        Arg::new("funcall")
            .long("funcall")
            .short('f')
            .value_name("FUNC")
            .action(ArgAction::Append)
            .help("call Emacs Lisp function FUNC with no arguments")
            .group("actions"),
        Arg::new("insert")
            .long("insert")
            .value_name("FILE")
            .action(ArgAction::Append)
            .help("insert contents of FILE into current buffer")
            .group("actions"),
        Arg::new("kill")
            .long("kill")
            .action(ArgAction::SetTrue)
            .help("exit without asking for confirmation")
            .group("actions"),
        Arg::new("load")
            .long("load")
            .short('l')
            .visible_alias("scriptload")
            .value_name("FILE")
            .action(ArgAction::Append)
            .help("load Emacs Lisp FILE using the load function")
            .group("actions"),
        Arg::new("NXOpen")
            .long("NXOpen")
            .value_name("FILE")
            .action(ArgAction::Append)
            .help("NXOpen")
            .group("actions"),
        Arg::new("NXOpenTemp")
            .long("NXOpenTemp")
            .value_name("FILE")
            .action(ArgAction::Append)
            .help("NXOpenTemp")
            .group("actions"),
        Arg::new("NSOpen")
            .long("NSOpen")
            .value_name("FILE")
            .action(ArgAction::Append)
            .help("NSOpen")
            .group("actions"),
        Arg::new("NSOpenTemp")
            .long("NSOpenTemp")
            .value_name("FILE")
            .action(ArgAction::Append)
            .help("NSOpenTemp")
            .group("actions"),
        Arg::new("GSFilePath")
            .long("GSFilePath")
            .value_name("FILE")
            .action(ArgAction::Append)
            .help("GSFilePath")
            .group("actions"),
    ]
});
static DISPLAY_OPTIONS: LazyLock<Vec<Arg>> = LazyLock::new(|| {
    vec![
        Arg::new("background-color")
            .long("background-color")
            .visible_alias("bg")
            .value_name("COLOR")
            .action(ArgAction::Append)
            .help("window background color")
            .group("display-options"),
        Arg::new("basic-display")
            .long("basic-display")
            .short('D')
            .action(ArgAction::SetTrue)
            .help(
                "disable many display features;
used for debugging Emacs",
            )
            .group("display-options"),
        Arg::new("border-color")
            .long("border-color")
            .visible_alias("bd")
            .value_name("COLOR")
            .action(ArgAction::Append)
            .help("main border color")
            .group("display-options"),
        Arg::new("border-width")
            .long("border-width")
            .visible_alias("bw")
            .value_name("WIDTH")
            .action(ArgAction::Append)
            .help("width of main border")
            .group("display-options"),
        Arg::new("color")
            .long("color")
            .value_name("MODE")
            .value_parser(EnumValueParser::<ColorChoice>::new())
            .help(
                "override color mode for character terminals;
MODE defaults to `auto', and
can also be `never', `always',
or a mode name like `ansi8'",
            )
            .group("display-options"),
        Arg::new("cursor-color")
            .long("cursor-color")
            .visible_alias("cr")
            .value_name("COLOR")
            .action(ArgAction::Append)
            .help("color of the Emacs cursor indicating point")
            .group("display-options"),
        Arg::new("font")
            .long("font")
            .visible_alias("fn")
            .value_name("FONT")
            .action(ArgAction::Append)
            .help("default font; must be fixed-width")
            .group("display-options"),
        Arg::new("foreground-color")
            .long("foreground-color")
            .visible_alias("fg")
            .value_name("COLOR")
            .action(ArgAction::Append)
            .help("window foreground color")
            .group("display-options"),
        Arg::new("fullheight")
            .long("fullheight")
            .visible_alias("fh")
            .action(ArgAction::SetTrue)
            .help("make the first frame high as the screen")
            .group("display-options"),
        Arg::new("fullscreen")
            .long("fullscreen")
            .visible_alias("fs")
            .action(ArgAction::SetTrue)
            .help("make the first frame fullscreen")
            .group("display-options"),
        Arg::new("fullwidth")
            .long("fullwidth")
            .visible_alias("fw")
            .action(ArgAction::SetTrue)
            .help("make the first frame wide as the screen")
            .group("display-options"),
        Arg::new("maximized")
            .long("maximized")
            .visible_alias("mm")
            .action(ArgAction::SetTrue)
            .help("make the first frame maximized")
            .group("display-options"),
        Arg::new("geometry")
            .long("geometry")
            .short('g')
            .value_name("GEOMETRY")
            .action(ArgAction::Append)
            .help("window geometry")
            .group("actions"),
        Arg::new("no-bitmap-icon")
            .long("no-bitmap-icon")
            .visible_alias("nbi")
            .action(ArgAction::SetTrue)
            .help("do not use picture of gnu for Emacs icon")
            .group("display-options"),
        Arg::new("iconic")
            .long("iconic")
            .action(ArgAction::SetTrue)
            .help("start Emacs in iconified state")
            .group("display-options"),
        Arg::new("internal-border")
            .long("internal-border")
            .visible_alias("ib")
            .value_name("WIDTH")
            .action(ArgAction::Append)
            .help("width between text and main border")
            .group("display-options"),
        Arg::new("line-spacing")
            .long("line-spacing")
            .visible_alias("lsp")
            .value_name("PIXELS")
            .action(ArgAction::Append)
            .help("additional space to put between lines")
            .group("display-options"),
        Arg::new("mouse-color")
            .long("mouse-color")
            .visible_alias("ms")
            .value_name("COLOR")
            .action(ArgAction::Append)
            .help("mouse cursor color in Emacs window")
            .group("display-options"),
        Arg::new("title")
            .long("title")
            .short('T')
            .visible_alias("name")
            .value_name("NAME")
            .action(ArgAction::Append)
            .help("title for initial Emacs frame")
            .group("display-options"),
        Arg::new("no-blinking-cursor")
            .long("no-blinking-cursor")
            .visible_alias("nbc")
            .action(ArgAction::SetTrue)
            .help("disable blinking cursor")
            .group("display-options"),
        Arg::new("reverse-video")
            .long("reverse-video")
            .short('r')
            .visible_alias("rv")
            .visible_alias("reverse")
            .action(ArgAction::SetTrue)
            .help("switch foreground and background")
            .group("display-options"),
        Arg::new("vertical-scroll-bars")
            .long("vertical-scroll-bars")
            .visible_alias("vb")
            .action(ArgAction::SetTrue)
            .help("enable vertical scroll bars")
            .group("display-options"),
        // TODO hide this, this is no in gnu Emacs
        Arg::new("horizontal-scroll-bars")
            .long("horizontal-scroll-bars")
            .visible_alias("hb")
            .action(ArgAction::SetTrue)
            .help("enable vertical scroll bars")
            .group("display-options"),
        Arg::new("xrm")
            .long("xrm")
            .value_name("XRESOURCES")
            .action(ArgAction::Append)
            .help("set additional X resources")
            .group("display-options"),
        Arg::new("parent-id")
            .long("parent-id")
            .value_name("XID")
            .action(ArgAction::Append)
            .help("set parent window")
            .group("display-options"),
        #[cfg(have_ns)]
        Arg::new("NSAutoLaunch")
            .long("NSAutoLaunch")
            .action(ArgAction::SetTrue)
            .help("NSAutoLaunch")
            .group("display-options"),
        #[cfg(have_ns)]
        Arg::new("NXAutoLaunch")
            .long("NXAutoLaunch")
            .action(ArgAction::SetTrue)
            .help("NXAutoLaunch")
            .group("display-options"),
        #[cfg(have_ns)]
        Arg::new("_NSMachLaunch")
            .long("_NSMachLaunch")
            .action(ArgAction::SetTrue)
            .help("_NSMachLaunch")
            .group("display-options"),
        #[cfg(have_ns)]
        Arg::new("MachLaunch")
            .long("MachLaunch")
            .action(ArgAction::SetTrue)
            .help("MachLaunch")
            .group("display-options"),
        #[cfg(have_ns)]
        Arg::new("macosx")
            .long("macosx")
            .action(ArgAction::SetTrue)
            .help("macosx")
            .group("display-options"),
        #[cfg(have_ns)]
        Arg::new("NSHost")
            .long("NSHost")
            .value_name("HOST")
            .action(ArgAction::Append)
            .help("NSHost")
            .group("display-options"),
    ]
});
static LISP_OPTIONS: LazyLock<Vec<Arg>> =
    LazyLock::new(|| vec![arg!(--"bin-dest"[DEST]), arg!(--"eln-dest"[DEST])]);
static KNOWN_OPTION_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    let mut result: Vec<String> = Vec::new();
    for options in [&INITIALIZATION_OPTIONS, &ACTION_OPTIONS, &DISPLAY_OPTIONS] {
        result = options.iter().fold(result, |mut acc, arg| {
            arg.get_long().map(|lname| acc.push(lname.to_string()));
            let acc = arg
                .get_visible_aliases()
                .map_or(Vec::new(), |v| v)
                .into_iter()
                .fold(acc, |mut acc, a| {
                    acc.push(a.to_string());
                    acc
                });
            acc
        });
    }
    result
});

#[derive(Debug)]
pub struct Args {
    pub dump_file: Option<PathBuf>,
    #[cfg(all(
        have_linux_seccomp_h,
        have_linux_filter_h,
        have_decl_seccomp_set_mode_filter,
        have_decl_seccomp_filter_flag_tsync
    ))]
    pub seccomp: Option<PathBuf>,
    pub temacs: Option<DumpMethod>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            dump_file: None,
            #[cfg(all(
                have_linux_seccomp_h,
                have_linux_filter_h,
                have_decl_seccomp_set_mode_filter,
                have_decl_seccomp_filter_flag_tsync
            ))]
            seccomp: None,
            temacs: None,
        }
    }
}

fn cli() -> Command {
    //TODO get the invocation name
    Command::new("emacs")
        .about(
            "Run Emacs, the extensible, customizable, self-documenting real-time
display editor.  The recommended way to start Emacs for normal editing
is with no options at all.

Run M-x info RET m emacs RET m emacs invocation RET inside Emacs to
read the main documentation for these command-line arguments.",
        )
        .group(ArgGroup::new("initialization").multiple(true))
        .next_help_heading("Initialization options")
        .args(INITIALIZATION_OPTIONS.iter())
        .group(ArgGroup::new("actions").multiple(true))
        .next_help_heading("Action options")
        .args(ACTION_OPTIONS.iter())
        .group(ArgGroup::new("display-options").multiple(true))
        .next_help_heading("Display options")
        .args(DISPLAY_OPTIONS.iter())
        .group(ArgGroup::new("lisp-options").multiple(true))
        .hide(true)
        .next_help_heading("Lisp options")
        .args(LISP_OPTIONS.iter())
        .after_help(
            "You can generally also specify long option names with a single -; for
example, -batch as well as --batch.  You can use any unambiguous
abbreviation for a --option.

Various environment variables and window system resources also affect
the operation of Emacs.  See the main documentation.

Report bugs to \" PACKAGE_BUGREPORT \".  First, please see the Bugs
section of the Emacs manual or the file BUGS.",
        )
}

fn normalize<I, T>(itr: I) -> Vec<String>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = itr.into_iter().fold(Vec::new(), |mut acc, arg| {
        let arg: OsString = arg.into();
        let arg = arg.to_string_lossy().to_string();
        let r = KNOWN_OPTION_NAMES
            .iter()
            .find(|n| arg.starts_with(format!("-{}", n).as_str()));
        let arg = match r {
            Some(_) => format!("-{}", arg),
            None => arg,
        };
        acc.push(arg);
        acc
    });
    args
}

pub fn args() -> Args {
    let tmp = normalize(std::env::args());
    let matches = cli().get_matches_from(tmp);
    let dump_file = matches.get_one::<PathBuf>("dump-file").map(|v| v.clone());
    let mut args = Args::default();
    args.dump_file = dump_file;

    if cfg!(all(
        have_linux_seccomp_h,
        have_linux_filter_h,
        have_decl_seccomp_set_mode_filter,
        have_decl_seccomp_filter_flag_tsync
    )) {
        args.seccomp = matches.get_one::<PathBuf>("seccomp").map(|v| v.clone());
    }
    args.temacs = matches.get_one::<DumpMethod>("temacs").copied();
    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matches = cli().get_matches_from([
            "emacs",
            "--temacs",
            DumpMethod::Bootstrap.to_string().as_str(),
        ]);
        let result = matches.get_one::<DumpMethod>("temacs");
        assert_eq!(result, Some(&DumpMethod::Bootstrap));
    }

    #[test]
    fn normalize_works() {
        let args = normalize([
            "emacs",
            "-temacs",
            DumpMethod::Bootstrap.to_string().as_str(),
        ]);
        let matches = cli().get_matches_from(args);
        let result = matches.get_one::<DumpMethod>("temacs");
        assert_eq!(result, Some(&DumpMethod::Bootstrap));
    }

    #[cfg(have_pdumper)]
    #[test]
    fn test1() {
        let args = normalize(["emacs", "-dump-file", "/tmp/sdf"]);
        let matches = cli().get_matches_from(args);
        let result = matches.get_one::<String>("dump-file");
        assert_eq!(result, Some(&"/tmp/sdf".to_string()));
    }

    #[test]
    fn bin_dest() {
        let args = normalize(["emacs", "--bin-dest"]);
        let matches = cli().get_matches_from(args);
        let result = matches.get_one::<String>("bin-dest");
        assert_eq!(result, None);
    }

    #[test]
    fn fg_works() {
        let args = normalize(["emacs", "-fg", "red"]);
        let matches = cli().get_matches_from(args);
        let result = matches.get_one::<String>("foreground-color");
        assert_eq!(result, Some(&"red".to_string()));
    }

    #[test]
    fn fg2_works() {
        let args = normalize(["emacs", "-fg=red"]);
        let matches = cli().get_matches_from(args);
        let result = matches.get_one::<String>("foreground-color");
        assert_eq!(result, Some(&"red".to_string()));
    }

    #[test]
    fn known_option_names_works() {
        let result = KNOWN_OPTION_NAMES.contains(&"bg-daemon".to_string());
        assert_eq!(result, true);
    }
}
