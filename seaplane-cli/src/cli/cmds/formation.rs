mod configuration;
mod container_stats;
mod create;
mod delete;
mod list;
mod stop;
mod template;
mod traffic_configuration;

use clap::{Parser, Subcommand};


use crate::{error::Result, Ctx};

/// Operate on Seaplane Formations
#[derive(Parser)]
pub struct SeaplaneFormationArgs {
    #[clap(subcommand)]
    cmd: SeaplaneFormationCmds,
}

impl SeaplaneFormationArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        

        self.update_ctx(ctx)?;

        match &self.cmd {
            _ => unimplemented!(),
        }
    }

    fn update_ctx(&self, _ctx: &mut Ctx) -> Result<()> {
        Ok(())
    }
}

#[derive(Subcommand)]
pub enum SeaplaneFormationCmds {}
