use clap::{ArgMatches, Command};
use seaplane::api::{
    metadata::v1::Key,
    shared::v1::{Directory, RangeQueryContext},
};

use crate::{
    api::MetadataReq,
    cli::{cmds::metadata::common, CliCommand},
    context::{Ctx, MetadataCtx},
    error::{CliError, CliErrorKind, Result},
    ops::metadata::KeyValues,
    printer::{Output, OutputFormat},
};

static LONG_ABOUT: &str = "List one or more metadata key-value pairs

Keys and values will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Using --decode allows one to decode them and display the unencoded
values.";

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneMetadataList;

impl SeaplaneMetadataList {
    pub fn command() -> Command<'static> {
        Command::new("list")
            .visible_alias("ls")
            .override_usage("seaplane metadata list <DIR> [OPTIONS]")
            .about("List one or more metadata key-value pairs")
            .long_about(LONG_ABOUT)
            .arg(
                arg!(dir =["DIR"])
                    .help("The root directory of the metadata key-value pairs to list"),
            )
            .arg(common::base64())
            .args(common::display_args())
            .group(common::keys_or_values())
            .arg(arg!(--from - ('f') =["KEY"]).help("Only print metadata key-value pairs after this key (note: if this key has a value it will be included in the results)"))
    }
}

impl CliCommand for SeaplaneMetadataList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        // Scope releases the mutex on the MetadataCtx so that when we hand off the ctx to print_*
        // we don't have the chance of a deadlock if those functions need to acquire a
        // MetadataCtx
        let kvs = {
            let mdctx = ctx.md_ctx.get_or_init();

            let mut range = RangeQueryContext::new();
            if let Some(dir) = &mdctx.directory {
                range.set_directory(dir.clone());
            }
            if let Some(from) = &mdctx.from {
                range.set_from(from.clone());
            }
            // Using the KeyValues container makes displaying easy
            let mut req = MetadataReq::new(ctx)?;
            req.set_dir(range)?;
            KeyValues::from_model(req.get_all_pages()?)
        };

        match ctx.args.out_format {
            OutputFormat::Json => kvs.print_json(ctx)?,
            OutputFormat::Table => kvs.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.md_ctx.init(MetadataCtx::default());
        ctx.args.out_format = matches.get_one("format").copied().unwrap_or_default();
        let mut mdctx = ctx.md_ctx.get_mut().unwrap();
        mdctx.base64 = matches.contains_id("base64");
        mdctx.decode = matches.contains_id("decode");
        mdctx.decode_safe = matches.contains_id("decode-safe");
        mdctx.no_decode = matches.contains_id("no-decode");
        mdctx.no_keys = matches.contains_id("only-values");
        mdctx.no_values = matches.contains_id("only-keys");
        mdctx.no_header = matches.contains_id("no-header");
        mdctx.keys_width_limit = matches
            .get_one::<usize>("keys-width-limit")
            .copied()
            .unwrap_or_default();
        mdctx.values_width_limit = matches
            .get_one::<usize>("values-width-limit")
            .copied()
            .unwrap_or_default();
        mdctx.from = maybe_base64_arg!(matches, "from", matches.contains_id("base64"))
            .map(Key::from_encoded);
        mdctx.directory = maybe_base64_arg!(matches, "dir", matches.contains_id("base64"))
            .map(Directory::from_encoded);

        // We set the decode_safe flag if there's no `decode` or `no-decode`
        // flags set, because there's no built-in clap method to turn a flag on
        // with default_value_if()
        if matches.contains_id("human-readable") && !(mdctx.decode || mdctx.no_decode) {
            mdctx.decode_safe = true
        };

        if mdctx.decode && ctx.args.out_format != OutputFormat::Table {
            let format_arg = format!("--format {}", ctx.args.out_format);
            return Err(CliError::from(CliErrorKind::ConflictingArguments(
                "--decode".to_owned(),
                format_arg,
            )));
        }

        Ok(())
    }
}
