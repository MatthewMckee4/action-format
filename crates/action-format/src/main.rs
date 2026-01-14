use std::fmt::Write;
use std::path::Path;
use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use owo_colors::{OwoColorize, Style};
use similar::{Algorithm, ChangeTag, TextDiff};

use action_format_cli::Cli;
use action_format_core::{FormatError, FormatterConfig, format_string};

mod printer;
use printer::Printer;

const WORKFLOWS_DIR: &str = ".github/workflows";

#[derive(Copy, Clone)]
pub enum ExitStatus {
    Success,
    Failure,
    Error,
}

impl From<ExitStatus> for ExitCode {
    fn from(status: ExitStatus) -> Self {
        match status {
            ExitStatus::Success => Self::from(0),
            ExitStatus::Failure => Self::from(1),
            ExitStatus::Error => Self::from(2),
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let printer = if cli.global.quiet {
        Printer::Quiet
    } else {
        Printer::Default
    };

    match run(&cli, printer) {
        Ok(status) => status.into(),
        Err(err) => {
            #[allow(clippy::print_stderr)]
            {
                let mut causes = err.chain();
                eprintln!(
                    "{}: {}",
                    "error".red().bold(),
                    causes.next().unwrap().to_string().trim()
                );
                for cause in causes {
                    eprintln!(
                        "  {}: {}",
                        "Caused by".red().bold(),
                        cause.to_string().trim()
                    );
                }
            }
            ExitStatus::Error.into()
        }
    }
}

fn run(cli: &Cli, printer: Printer) -> Result<ExitStatus> {
    let workflows_path = Path::new(WORKFLOWS_DIR);

    if !workflows_path.exists() {
        anyhow::bail!("No {WORKFLOWS_DIR} directory found");
    }

    let config = FormatterConfig::default();
    let mut any_changed = false;
    let mut any_error = false;

    let walker = walkdir::WalkDir::new(workflows_path)
        .sort_by_file_name()
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| is_workflow_file(e.path()));

    for entry in walker {
        match process_file(entry.path(), &config, cli, printer) {
            Ok(changed) => any_changed |= changed,
            Err(e) => {
                let _ = writeln!(
                    printer.stderr(),
                    "{}: {}: {}",
                    "error".red().bold(),
                    entry.path().display(),
                    e
                );
                any_error = true;
            }
        }
    }

    if any_error {
        Ok(ExitStatus::Error)
    } else if cli.check && any_changed {
        Ok(ExitStatus::Failure)
    } else {
        Ok(ExitStatus::Success)
    }
}

fn is_workflow_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .is_some_and(|ext| ext == "yml" || ext == "yaml")
}

fn process_file(
    path: &Path,
    config: &FormatterConfig,
    cli: &Cli,
    printer: Printer,
) -> Result<bool, FormatError> {
    let content = fs_err::read_to_string(path)?;
    let formatted = format_string(&content, config)?;

    if content == formatted {
        return Ok(false);
    }

    if cli.check {
        let _ = writeln!(
            printer.stdout(),
            "{}: {}",
            "Would reformat".yellow(),
            path.display()
        );
        return Ok(true);
    }

    if cli.diff {
        print_diff(path, &content, &formatted, printer);
        return Ok(true);
    }

    fs_err::write(path, &formatted)?;

    let _ = writeln!(
        printer.stdout(),
        "{}: {}",
        "Reformatted".green(),
        path.display()
    );

    Ok(true)
}

fn terminal_width() -> usize {
    terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80)
}

fn print_diff(path: &Path, old: &str, new: &str, printer: Printer) {
    let width = terminal_width();
    let mut stdout = printer.stdout();

    let _ = writeln!(stdout, "Source: {}", path.display());

    let diff = TextDiff::configure()
        .algorithm(Algorithm::Patience)
        .diff_lines(old, new);

    let _ = writeln!(stdout, "────────────┬{:─^1$}", "", width.saturating_sub(13));

    for (idx, group) in diff.grouped_ops(4).iter().enumerate() {
        if idx > 0 {
            let _ = writeln!(stdout, "┈┈┈┈┈┈┈┈┈┈┈┈┼{:┈^1$}", "", width.saturating_sub(13));
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                match change.tag() {
                    ChangeTag::Insert => {
                        let _ = write!(
                            stdout,
                            "{:>5} {:>5} │{}",
                            "",
                            (change.new_index().unwrap() + 1)
                                .style(Style::new().cyan().dimmed().bold()),
                            "+".green(),
                        );
                        for &(emphasized, value) in change.values() {
                            if emphasized {
                                let _ = write!(stdout, "{}", value.green().underline());
                            } else {
                                let _ = write!(stdout, "{}", value.green());
                            }
                        }
                    }
                    ChangeTag::Delete => {
                        let _ = write!(
                            stdout,
                            "{:>5} {:>5} │{}",
                            (change.old_index().unwrap() + 1).style(Style::new().cyan().dimmed()),
                            "",
                            "-".red(),
                        );
                        for &(emphasized, value) in change.values() {
                            if emphasized {
                                let _ = write!(stdout, "{}", value.red().underline());
                            } else {
                                let _ = write!(stdout, "{}", value.red());
                            }
                        }
                    }
                    ChangeTag::Equal => {
                        let _ = write!(
                            stdout,
                            "{:>5} {:>5} │ ",
                            (change.old_index().unwrap() + 1).style(Style::new().cyan().dimmed()),
                            (change.new_index().unwrap() + 1)
                                .style(Style::new().cyan().dimmed().bold()),
                        );
                        for &(_, value) in change.values() {
                            let _ = write!(stdout, "{}", value.dimmed());
                        }
                    }
                }
                if change.missing_newline() {
                    let _ = writeln!(stdout);
                }
            }
        }
    }

    let _ = writeln!(stdout, "────────────┴{:─^1$}", "", width.saturating_sub(13));
}
