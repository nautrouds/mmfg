use std::io::Read;
use std::os::unix::net::UnixStream;
use crate::net::receive_fds;
use std::os::unix::io::RawFd;
use crate::error::{Result, MmfgError};

pub const CONN_HEADER: &[u8; 4] = b"MMFG";
pub const CONN_VERSION: u8 = 1;

pub struct HandshakeResult {
    pub node_id: usize,
    pub fds: Vec<RawFd>,
}

pub fn perform_handshake(stream: &mut UnixStream) -> Result<HandshakeResult> {
    let mut header = [0u8; 7];
    stream.read_exact(&mut header).map_err(MmfgError::from)?;
    
    if &header[0..4] != CONN_HEADER {
        return Err(MmfgError::Protocol("Invalid handshake header".to_string()));
    }

    if header[4] != CONN_VERSION {
        return Err(MmfgError::Protocol(format!("Unsupported protocol version: {}", header[4])));
    }
    
    // header[5] is MsgType (Handshake = 1)
    let node_id = header[6] as usize;

    let fds = receive_fds(stream, 3).map_err(|e| MmfgError::Protocol(format!("FD receive failed: {}", e)))?;

    Ok(HandshakeResult {
        node_id,
        fds,
    })
}
