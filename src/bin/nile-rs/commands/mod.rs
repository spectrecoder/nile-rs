mod compile;
mod init;
mod run;

pub use compile::{
    cairo_to_sierra::CompileCairoToSierra, sierra_to_casm::CompileSierraToCasm, Compile,
};
pub use init::Init;
pub use run::Run;

use anyhow::Result;

/// Common trait for Cli commands
pub trait CliCommand {
    type Output;
    fn run(self) -> Result<Self::Output>;
}