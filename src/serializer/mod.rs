use std::io;

use crate::allocator;
use crate::utils;

pub trait Reader {
    /// Waits for the socket to become readable. This method is cancel safe.
    fn readable(&mut self) -> impl std::future::Future<Output = io::Result<()>>;

    /// Tries to read data from the stream into the provided buffer, returning how many bytes were read.
    fn try_read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize>;
}

// --------------------------- RPC header ---------------------------
// | xid: i32 | enum type: i32
//              type_tag == CALL | rpcvers: u32 | prog: u32 | vers: u32 | proc: u32 | enum auth_flavor: u32 | body: [u8; <= 400] enum auth_flavor: u32 | body: [u8; <= 400]

pub struct Header {
    xid: i32,
    msg_type: u32,
    rpcvers: u32,
    prog: u32,
    vers: u32,
    proc: u32,
    cred: OpaqueAuth,
    verf: OpaqueAuth,
}

struct OpaqueAuth {
    flavour: u32,
    body: Vec<u8>,
}

pub enum Args {

}

pub enum Error {
    Internal(&'static str),
}

pub struct Message {
    header: Header,
    args: Args,
}

pub async fn read(stream: &mut impl Reader) -> std::result::Result<Args, Error> {
    let mut buf = vec![0; 1024];
    let mut curr_slice = &mut buf[0..];

    loop {
        stream.readable().await.map_err(|e| Error::Internal(utils::io_error_as_str(e)))?;

        match stream.try_read_vectored(&mut [io::IoSliceMut::new(curr_slice)]) {
            Ok(0) => (),
            Ok(n) => curr_slice = &mut curr_slice[n..],
            Err(_) => (),
        }



        if curr_slice.is_empty() {
            return Err(Error::Internal("slice exhousted"));
        }
    }

    Ok(todo!())
}

enum TryParseError {
    MoreDataNeeded,
    Internal(&'static str),
}

struct ReadMessage {
    header: Option<Header>,
    args: Option<Args>,

}

pub fn try_parse(data: &[&[u8]]) -> std::result::Result<Message, TryParseError> {

    todo!()
}
