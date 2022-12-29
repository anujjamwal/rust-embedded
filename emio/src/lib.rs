#![no_std]

const READABLE:u8 = 0b0001;
const WRITABLE:u8 = 0b0010;

#[derive(Clone, PartialEq, Eq)]
pub struct Interest(u8);

impl Interest {
    const READABLE: Interest = Interest(READABLE);
    const WRITABLE: Interest = Interest(WRITABLE);

    pub fn readable() -> Self {
        Self::READABLE
    }

    pub fn writable() -> Self {
        Self::WRITABLE
    }

    pub fn is_readable(&self) -> bool {
        self.0 & READABLE != 0
    }

    pub fn is_writable(&self) -> bool {
        self.0 & WRITABLE != 0
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Source(usize);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token(usize);

#[derive(Clone, PartialEq, Eq)]
pub struct Event {
  token: Token,
}

pub struct Events {}

pub enum Error {
    // Indicates that the requested interests are not supported
    // by the source.
    UnsupportedInterest,
}

pub trait Reactor {
    fn register(&mut self, _src: &mut Source, _interest: Interest, _token: Token) -> Result<(), Error>;

    fn poll(&mut self, _events: &mut Events) -> Result<usize, Error>;
}

