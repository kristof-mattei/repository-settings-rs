use std::env;
use std::ffi::OsString;
use std::num::NonZeroU16;

use clap::parser::ValueSource;
use clap::{command, value_parser, Arg, ArgAction, Command};
use color_eyre::eyre::{self, WrapErr};
use once_cell::sync::Lazy;

use crate::state::config::{Config, DEFAULT_PORT};

static DEFAULT_PORT_VALUE: Lazy<String> = Lazy::new(|| DEFAULT_PORT.to_string());

fn build_clap_matcher() -> Command {
    command!()
        .disable_help_flag(true)
        .color(clap::ColorChoice::Always)
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Listening port")
                .display_order(6)
                .default_value(DEFAULT_PORT_VALUE.as_str())
                .value_parser(value_parser!(u64).range(u64::from(1u16)..=u64::from(u16::MAX))),
        )
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help("Print this help message and exit")
                .display_order(9)
                .action(ArgAction::Help),
        )
}

pub(crate) fn parse_cli() -> Result<Config, eyre::Error> {
    parse_cli_from(env::args_os())
}

fn parse_cli_from<I, T>(from: I) -> Result<Config, eyre::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = build_clap_matcher().try_get_matches_from(from)?;

    let mut config = Config::new();

    if let Some(&p) = get_user_cli_value::<u64>(&matches, "port") {
        let arg_u16 =
            u16::try_from(p).wrap_err_with(|| format!("Couldn't convert '{}' to u16", p))?;

        let non_zero_arg = NonZeroU16::try_from(arg_u16)
            .wrap_err_with(|| format!("{} is not a valid value for port", arg_u16))?;

        config.set_port(non_zero_arg);
    }

    Ok(config)
}

fn get_user_cli_value<'a, T>(matches: &'a clap::ArgMatches, key: &str) -> Option<&'a T>
where
    T: Clone + Send + Sync + 'static,
{
    // our CLI has defaults, so we check if the user has provided a value
    let Some(ValueSource::CommandLine) = matches.value_source(key) else {
        return None;
    };

    // NOTE: we might change this later to always use the user's input, as we might want this module
    // to drive the config's defaults.
    // I am always confused as to who should do what. Who provides defaults? Who provides upper and lower limits?
    // Because not everything comes through a CLI. I would love to share this with something like
    // a yaml file. But then we run into issues with valid values for a type (say 1 for max-line-length) but
    // that's an invalid number in our logic.
    // on the other hand there are port 100000 which doesn't even fit into our data type

    // return the value provided by the user
    matches.get_one::<T>(key)
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;

    use color_eyre::eyre;

    use crate::state::config::Config;

    use super::parse_cli_from;

    fn parse_factory(input: &'static str) -> Result<Config, eyre::Report> {
        // fake input
        let command_line = input.split_whitespace().collect::<Vec<&str>>();

        parse_cli_from(command_line)
    }

    #[test]
    fn bad_cli_options_1() {
        let result = parse_factory("foo bar");

        assert!(result.is_err());
    }

    #[test]
    fn bad_cli_options_2() {
        let result = parse_factory("endless-ssh-rs bar");

        assert!(result.is_err());
    }

    #[test]
    fn parses_port() {
        let result = parse_factory("endless-ssh-rs --port 2000");

        #[allow(clippy::needless_update)]
        let expected_config = Config {
            port: NonZeroU16::new(2000).unwrap(),
            ..Default::default()
        };

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_config);
    }

    #[test]
    fn ensures_minimum_line_length() {
        let result = parse_factory("endless-ssh-rs --max-line-length 2");

        assert!(result.is_err());
    }

    #[test]
    fn specifying_ipv4_and_ipv6_throw_error() {
        let result = parse_factory("endless-ssh-rs -4 -6");

        assert!(result.is_err());
    }
}
