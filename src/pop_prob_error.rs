use core::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PopProbError {
    Generic,
    SnskKGreaterThanN((u32, u32)),
    UniqueGreaterThanSample((u32, u32)),
    UniqueGreaterThanSize((u32, u32)),
    UniqueZero,
}

impl Display for PopProbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic => write!(f, "There was an error"),
            Self::SnskKGreaterThanN((k, n)) => write!(
                f,
                "`k` cannot be greater than `n` (`k` = {}, `n` = {})",
                k, n
            ),
            Self::UniqueGreaterThanSample((unique, sample)) => write!(
                f,
                "`unique` cannot be greater than `sample` (`unique` = {}, `sample` = {})",
                unique, sample
            ),
            Self::UniqueGreaterThanSize((unique, size)) => write!(
                f,
                "`unique` cannot be greater than `size` (`unique` = {}, `size` = {})",
                unique, size
            ),
            Self::UniqueZero => write!(f, "`unique` cannot be zero"),
        }
    }
}

impl Error for PopProbError {}
