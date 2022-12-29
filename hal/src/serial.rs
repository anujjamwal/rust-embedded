use crate::io::AsyncRead;
use crate::io::AsyncWrite;
use crate::io::Error;
use core::pin::Pin;
use core::task::Context;
use core::task::Poll;

type BaudRate = u16;

const BAUD_9600: BaudRate = 9600;

pub struct Config {
    baud_rate: BaudRate,
    async_mode: bool,
}

pub trait Usart {
    fn init(&mut self, c: Config) -> Result<(), Error>;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;

    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
}

pub struct Serial<'a, D: Usart> {
    internal: &'a D, 
}

impl <'a, D: Usart> Serial<'a, D> {
    pub fn init(d: &'a mut D, c: Config) -> Result<Self, Error> {
        d.init(c).map(|()| { Serial {internal: d} })
    }
}

impl<'a, D: Usart> AsyncRead for Serial<'a, D> {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context, buf: &mut [u8]) -> Poll<Result<usize, Error>> {
        Poll::Pending
    }
}

impl<'a, D: Usart> AsyncWrite for Serial<'a, D> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<Result<usize, Error>> {
        Poll::Pending
    }
}
