use std::error::Error;
use std::fmt;
use std::io::Error as IOERROR;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum CustomError {
    custom(String),
    IO(IOERROR),
}
