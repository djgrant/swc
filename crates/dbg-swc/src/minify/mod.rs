use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use swc_common::SourceMap;

use self::{compare::CompareCommand, ensure_size::EnsureSize, reduce::ReduceCommand};

mod compare;
mod ensure_size;
mod reduce;

/// Minify a javascript file.
#[derive(Debug, Subcommand)]
pub enum MinifyCommand {
    Reduce(ReduceCommand),
    Compare(CompareCommand),
    EnsureSize(EnsureSize),
}

impl MinifyCommand {
    pub fn run(self, cm: Arc<SourceMap>) -> Result<()> {
        match self {
            MinifyCommand::Reduce(cmd) => cmd.run(cm),
            MinifyCommand::EnsureSize(cmd) => cmd.run(cm),
            MinifyCommand::Compare(cmd) => cmd.run(cm),
        }
    }
}
