use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("room already exists")]
    RoomAlreadyExists,
    #[error("generic io error")]
    Io(#[from] io::Error),
}
