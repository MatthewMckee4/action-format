use anstream::{eprint, print};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Printer {
    Quiet,
    Default,
}

impl Printer {
    pub fn stdout(self) -> Stdout {
        match self {
            Self::Quiet => Stdout::Disabled,
            Self::Default => Stdout::Enabled,
        }
    }

    #[allow(clippy::unused_self)]
    pub fn stderr(self) -> Stderr {
        Stderr
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stdout {
    Enabled,
    Disabled,
}

impl std::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self {
            Self::Enabled => {
                #[allow(clippy::print_stdout, clippy::ignored_unit_patterns)]
                {
                    print!("{s}");
                }
            }
            Self::Disabled => {}
        }
        Ok(())
    }
}

pub struct Stderr;

impl std::fmt::Write for Stderr {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        #[allow(clippy::print_stderr, clippy::ignored_unit_patterns)]
        {
            eprint!("{s}");
        }
        Ok(())
    }
}
