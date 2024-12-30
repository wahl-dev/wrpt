use crate::commands::Command;
use clap::{Args, ColorChoice as ClapColorChoice, Parser};
use log::LevelFilter;
use simplelog::{
    ColorChoice as SimpleLogColorChoice, CombinedLogger, Config, TermLogger, TerminalMode,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, styles=get_styles())]
pub(crate) struct WrptArgs {
    #[command(flatten)]
    pub global_args: GlobalArgs,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Args)]
pub(crate) struct GlobalArgs {
    /// URL of the Portainer instance
    #[arg(short = 'l', long, global = true)]
    pub url: Option<String>,

    /// Access token of the Portainer instance
    #[arg(short = 'A', long, global = true)]
    pub access_token: Option<String>,

    /// Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
    #[arg(short, global = true, default_value_t = 1, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(..=4))]
    pub verbose: u8,

    /// Do not output any message
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// When to use terminal colours
    #[arg(long, global = true, default_value_t = ClapColorChoice::Auto)]
    pub color: ClapColorChoice,
}

pub(crate) fn init_logger(args: &WrptArgs) {
    let mut level_filter = LevelFilter::Off;

    if !args.global_args.quiet {
        level_filter = match args.global_args.verbose {
            1 => LevelFilter::Error,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            4 => LevelFilter::Trace,
            _ => LevelFilter::Off,
        };
    }

    let color_choice = match args.global_args.color {
        ClapColorChoice::Auto => SimpleLogColorChoice::Auto,
        ClapColorChoice::Always => SimpleLogColorChoice::Always,
        ClapColorChoice::Never => SimpleLogColorChoice::Never,
    };

    CombinedLogger::init(vec![TermLogger::new(
        level_filter,
        Config::default(),
        TerminalMode::Mixed,
        color_choice,
    )])
    .unwrap();
}

fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}
