use std::fmt::{Display, Formatter};
use std::process::Command;

#[derive(Default, Debug)]
pub enum Verbosity {
    #[default]
    None,
    One,
    Two,
    Three
}

impl Display for Verbosity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Verbosity::None => { "".fmt(f) }
            Verbosity::One => { "1".fmt(f) }
            Verbosity::Two => { "2".fmt(f) }
            Verbosity::Three => { "3".fmt(f) }
        }
    }
}