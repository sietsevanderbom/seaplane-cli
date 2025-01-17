mod common;
mod delete;
mod get;
mod list;
mod set;

use clap::{value_parser, ArgMatches, Command};

pub use self::{
    common::SeaplaneMetadataCommonArgMatches,
    delete::SeaplaneMetadataDelete,
    get::SeaplaneMetadataGet,
    list::SeaplaneMetadataList,
    set::{SeaplaneMetadataSet, SeaplaneMetadataSetArgMatches},
};
use crate::{cli::CliCommand, printer::OutputFormat};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneMetadata;

impl SeaplaneMetadata {
    pub fn command() -> Command<'static> {
        Command::new("metadata")
            .about("Operate on metadata key-value pairs using the Global Data Coordination API")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .visible_aliases(&["meta", "md"])
            .arg(
                arg!(--format =["FORMAT"=>"table"] global)
                    .help("Change the output format")
                    .value_parser(value_parser!(OutputFormat)),
            )
            .subcommand(SeaplaneMetadataGet::command())
            .subcommand(SeaplaneMetadataSet::command())
            .subcommand(SeaplaneMetadataDelete::command())
            .subcommand(SeaplaneMetadataList::command())
    }
}

impl CliCommand for SeaplaneMetadata {
    fn next_subcmd<'a>(
        &self,
        matches: &'a ArgMatches,
    ) -> Option<(Box<dyn CliCommand>, &'a ArgMatches)> {
        match &matches.subcommand() {
            Some(("get", m)) => Some((Box::new(SeaplaneMetadataGet), m)),
            Some(("set", m)) => Some((Box::new(SeaplaneMetadataSet), m)),
            Some(("delete", m)) => Some((Box::new(SeaplaneMetadataDelete), m)),
            Some(("list", m)) => Some((Box::new(SeaplaneMetadataList), m)),
            _ => None,
        }
    }
}
