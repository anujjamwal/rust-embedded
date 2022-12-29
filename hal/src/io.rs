use core::pin::Pin;
use core::task::Context;
use core::task::Poll;

pub enum Error {
    WouldBlock,
    Other(&'static str),
}

pub trait AsyncRead {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context, buf: &mut [u8]) -> Poll<Result<usize, Error>>;
}

pub trait AsyncWrite {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<Result<usize, Error>>;
}
