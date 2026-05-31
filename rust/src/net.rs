use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::UnixStream;
use crate::error::{Result, MmfgError};

pub fn receive_fds(stream: &UnixStream, count: usize) -> Result<Vec<RawFd>> {
    let mut fds = Vec::with_capacity(count);
    let mut iov_data = [0u8; 1];
    let mut iov = libc::iovec {
        iov_base: iov_data.as_mut_ptr() as *mut libc::c_void,
        iov_len: iov_data.len(),
    };

    let cmsg_size = unsafe { libc::CMSG_SPACE((count * std::mem::size_of::<RawFd>()) as u32) as usize };
    let mut cmsg_buf = vec![0u8; cmsg_size];

    let mut msg = unsafe { std::mem::zeroed::<libc::msghdr>() };
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen = cmsg_size;

    let n = unsafe { libc::recvmsg(stream.as_raw_fd(), &mut msg, 0) };
    if n < 0 {
        return Err(MmfgError::Io(std::io::Error::last_os_error()));
    }
    if n == 0 {
        return Err(MmfgError::Protocol("Socket closed".to_string()));
    }

    let mut cmsg = unsafe { libc::CMSG_FIRSTHDR(&msg) };
    while !cmsg.is_null() {
        if unsafe { (*cmsg).cmsg_level == libc::SOL_SOCKET && (*cmsg).cmsg_type == libc::SCM_RIGHTS } {
            let data_ptr = unsafe { libc::CMSG_DATA(cmsg) as *const RawFd };
            let num_fds = unsafe {
                ((*cmsg).cmsg_len as usize - libc::CMSG_LEN(0) as usize) / std::mem::size_of::<RawFd>()
            };
            for i in 0..num_fds {
                fds.push(unsafe { *data_ptr.add(i) });
            }
        }
        cmsg = unsafe { libc::CMSG_NXTHDR(&msg, cmsg) };
    }

    if fds.len() < count {
        return Err(MmfgError::Protocol(format!("Received fewer FDs than expected: {} < {}", fds.len(), count)));
    }

    Ok(fds)
}
