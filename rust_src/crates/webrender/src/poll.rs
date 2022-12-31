use nix::unistd::close;

use std::io;
use std::io::Error;
use std::os::unix::io::{AsRawFd, RawFd};
use std::time::Duration;

use easy_parallel::Parallel;
use polling::{Event, Poller};

use crate::event_loop::{FdSet, Timespec};

#[derive(Debug)]
pub struct FileDescriptor {
    fd: RawFd,
}

impl AsRawFd for FileDescriptor {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl Drop for FileDescriptor {
    fn drop(&mut self) {
        let _ = close(self.fd);
    }
}

// There are two rust poll crates, we are trying "polling" here
// https://github.com/smol-rs/polling
// https://docs.rs/mio/latest/mio/guide/index.html

pub fn tokio_select_fds(
    nfds: i32,
    readfds: &FdSet,
    writefds: &FdSet,
    timespec: &Timespec,
) -> std::io::Result<i32> {
    let poller = Poller::new()?;

    let mut events = Vec::new();

    let timeout =
        unsafe { Duration::new((*timespec.0).tv_sec as u64, (*timespec.0).tv_nsec as u32) };
    let mut descriptors = None;

    Parallel::new()
        .add(|| {
            poller.wait(&mut events, Some(timeout))?;
            // poller.wait(&mut events, None)?;
            Ok(())
        })
        .add(|| {
            descriptors = Some(upload_fds(nfds, &readfds, &writefds, &poller));
            poller.notify()?;
            Ok(())
        })
        .run()
        .into_iter()
        .collect::<io::Result<()>>()?;

    if events.len() > 0 {
        log::debug!("events {:?}", events);
    }

    match descriptors.unwrap() {
        Ok(ret) => descriptors_to_fd_set(&ret, readfds, writefds, &events),
        Err(err) => {
	    // log::error!("poller add err: {err:?}");
	},
    }

    Ok(events.len() as i32)
}

pub fn upload_fds(
    nfds: i32,
    readfds: &FdSet,
    writefds: &FdSet,
    poller: &polling::Poller,
) -> Result<(Vec<FileDescriptor>, Vec<FileDescriptor>), Error> {
    let rfds = fd_set_to_descriptors(nfds, readfds);
    let wfds = fd_set_to_descriptors(nfds, writefds);

    for i in 0..rfds.len() {
        if let Some(fd) = rfds.get(i) {
            poller.add(&*fd, polling::Event::readable(i))?
        }
    }

    for i in 0..wfds.len() {
        if let Some(fd) = wfds.get(i) {
            poller.add(&*fd, polling::Event::writable(i))?
        }
    }

    Ok((rfds, wfds))
}

fn fd_set_to_descriptors(nfds: i32, fd_set: &FdSet) -> Vec<FileDescriptor> {
    if fd_set.0 == std::ptr::null_mut() {
        return Vec::new();
    }

    let mut result = Vec::new();

    for fd in 0..nfds {
        unsafe {
            if libc::FD_ISSET(fd, fd_set.0) {
                // set_nonblocking(fd);
                result.push(FileDescriptor { fd });
            }
        }
    }

    result
}

pub fn descriptors_to_fd_set(
    descriptors: &(Vec<FileDescriptor>, Vec<FileDescriptor>),
    rfd_set: &FdSet,
    wfd_set: &FdSet,
    events: &Vec<Event>,
) {
    let (r_descriptors, w_descriptors) = descriptors;
    for Event {
        key,
        readable,
        writable,
    } in events
    {
        if *readable {
            if let Some(descriptor) = r_descriptors.get(*key) {
                let fd = descriptor.as_raw_fd();
                unsafe { libc::FD_SET(fd, rfd_set.0) };
            }
        }
        if *writable {
            if let Some(descriptor) = w_descriptors.get(*key) {
                let fd = descriptor.as_raw_fd();
                unsafe { libc::FD_SET(fd, wfd_set.0) };
            }
        }
    }
}

// Is this useful?
fn set_nonblocking(fd: std::os::unix::prelude::RawFd) {
    use nix::fcntl::{OFlag, F_GETFL, F_SETFL};

    let flags = nix::fcntl::fcntl(fd, F_GETFL);
    if flags.is_err() {
        log::debug!("fcntl(F_GETFD) {:?}", flags.err());
        return;
    }
    let flags = flags.unwrap();

    if flags < 0 {
        log::debug!(
            "bad return value from fcntl(F_GETFL): {} ({:?})",
            flags,
            nix::Error::last()
        );
    }

    let flags = OFlag::from_bits_truncate(flags) | OFlag::O_NONBLOCK;

    let result = nix::fcntl::fcntl(fd, F_SETFL(flags));

    if result.is_err() {
        log::debug!("fcntl(F_SETFD) {:?}", result.err());
    }
}
