use std::io;
use std::os::unix::io::{RawFd, AsRawFd};

pub struct Eventfd {
    fd: RawFd,
}

impl AsRawFd for Eventfd {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl Eventfd {
    pub fn new() -> io::Result<Self> {
        let fd = unsafe { libc::eventfd(0, libc::EFD_CLOEXEC) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self { fd })
    }

    pub fn attach(fd: RawFd) -> Self {
        Self { fd }
    }

    pub fn notify(&self) -> io::Result<()> {
        let val: u64 = 1;
        let res = unsafe {
            libc::write(
                self.fd,
                &val as *const u64 as *const libc::c_void,
                std::mem::size_of::<u64>(),
            )
        };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    pub fn wait(&self) -> io::Result<()> {
        let mut val: u64 = 0;
        let res = unsafe {
            libc::read(
                self.fd,
                &mut val as *mut u64 as *mut libc::c_void,
                std::mem::size_of::<u64>(),
            )
        };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    pub fn fd(&self) -> RawFd {
        self.fd
    }
}

impl Drop for Eventfd {
    fn drop(&mut self) {
        if self.fd >= 0 {
            unsafe { libc::close(self.fd) };
            self.fd = -1;
        }
    }
}
