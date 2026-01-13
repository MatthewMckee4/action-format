pub use clap::Parser;
pub use clap::builder::Styles;
pub use clap::builder::styling::{AnsiColor, Effects, Style};

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser)]
#[command(name = "action-format", author, version)]
#[command(about = "A fast GitHub Actions workflow formatter")]
#[command(styles = STYLES)]
pub struct Cli {
    /// Check if files are formatted without modifying them
    #[arg(long, short)]
    pub check: bool,

    /// Print the diff of formatting changes
    #[arg(long)]
    pub diff: bool,

    #[command(flatten)]
    pub global: GlobalArgs,
}

#[derive(Parser, Debug, Clone)]
#[command(next_help_heading = "Global options")]
pub struct GlobalArgs {
    /// Use quiet output (only show errors)
    #[arg(long, short)]
    pub quiet: bool,

    /// Control the use of color in output
    #[arg(long, value_enum, value_name = "WHEN")]
    pub color: Option<ColorChoice>,
}

#[derive(Debug, Copy, Clone, clap::ValueEnum)]
pub enum ColorChoice {
    /// Enable colors when output is a terminal
    Auto,
    /// Always enable colors
    Always,
    /// Disable colors
    Never,
}

impl ColorChoice {
    #[must_use]
    pub fn and_colorchoice(self, next: anstream::ColorChoice) -> Self {
        match self {
            Self::Auto => match next {
                anstream::ColorChoice::Auto => Self::Auto,
                anstream::ColorChoice::Always | anstream::ColorChoice::AlwaysAnsi => Self::Always,
                anstream::ColorChoice::Never => Self::Never,
            },
            Self::Always | Self::Never => self,
        }
    }
}
