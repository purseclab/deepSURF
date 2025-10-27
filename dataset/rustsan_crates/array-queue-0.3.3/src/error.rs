use std::error::Error;
use std::fmt::{Display, Formatter, Result};

const MESSAGE: &'static str = "queue is full";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CapacityError;

impl Display for CapacityError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", MESSAGE)
    }
}

impl Error for CapacityError {
    fn description(&self) -> &str {
        MESSAGE
    }
}
