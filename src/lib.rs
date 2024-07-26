//! `pm-bin`: Util for rust CLI program.
//!
//! <https://github.com/fm-elpac/pm-bin>
#![deny(unsafe_code)]

pub use env_logger;
pub use log;

#[cfg(feature = "build")]
mod build;
mod cli;

#[cfg(feature = "build")]
pub use build::build_gen;
pub use cli::{cli_arg, init_env_logger};

#[cfg(test)]
mod test {
    // TODO
}
