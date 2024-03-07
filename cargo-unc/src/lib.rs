#![allow(clippy::large_enum_variant)]

pub use unc_cli_rs::CliResult;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

pub mod commands;
mod common;
pub mod types;
mod util;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = unc_cli_rs::GlobalContext)]
pub struct Cmd {
    #[interactive_clap(subcommand)]
    pub opts: Opts,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = unc_cli_rs::GlobalContext)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(disable_back)]
/// Near
pub enum Opts {
    #[strum_discriminants(strum(message = "unc"))]
    /// Which cargo extension do you want to use?
    Near(NearArgs),
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = unc_cli_rs::GlobalContext)]
pub struct NearArgs {
    #[interactive_clap(subcommand)]
    pub cmd: self::commands::UncCommand,
}
