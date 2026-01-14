#![allow(dead_code, unreachable_pub)]

use assert_cmd::Command;
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::*;
use regex::Regex;
use std::path::{Path, PathBuf};

/// Test context for running action-format commands.
pub struct TestContext {
    pub root: ChildPath,
    filters: Vec<(String, String)>,
    pub _root: tempfile::TempDir,
}

impl TestContext {
    pub fn new() -> Self {
        let root = tempfile::TempDir::with_prefix("action-format-test")
            .expect("Failed to create test root directory");

        let mut filters = Vec::new();

        filters.extend(
            Self::path_patterns(root.path())
                .into_iter()
                .map(|pattern| (pattern, "[TEMP]/".to_string())),
        );

        // Create .github/workflows directory
        let workflows_dir = root.path().join(".github/workflows");
        std::fs::create_dir_all(&workflows_dir).expect("Failed to create workflows directory");

        Self {
            root: ChildPath::new(root.path()),
            _root: root,
            filters,
        }
    }

    pub fn path_patterns(path: impl AsRef<Path>) -> Vec<String> {
        let mut patterns = Vec::new();

        if path.as_ref().exists() {
            patterns.push(Self::path_pattern(
                path.as_ref()
                    .canonicalize()
                    .expect("Failed to create canonical path"),
            ));
        }

        patterns.push(Self::path_pattern(path));
        patterns
    }

    fn path_pattern(path: impl AsRef<Path>) -> String {
        format!(
            r"{}(\\|\/)*",
            regex::escape(&path.as_ref().display().to_string()).replace(r"\\", r"(\\|\/)+")
        )
    }

    pub fn filters(&self) -> Vec<(&str, &str)> {
        self.filters
            .iter()
            .map(|(p, r)| (p.as_str(), r.as_str()))
            .chain(INSTA_FILTERS.iter().copied())
            .collect()
    }

    /// Create a workflow file in .github/workflows with the given content.
    pub fn workflow(&self, name: &str, content: &str) -> &Self {
        self.root
            .child(format!(".github/workflows/{name}"))
            .write_str(content.strip_prefix('\n').unwrap_or(content))
            .expect("Failed to write workflow file");
        self
    }

    /// Read a file from .github/workflows and return its contents.
    pub fn read_workflow(&self, name: &str) -> String {
        std::fs::read_to_string(self.root.join(format!(".github/workflows/{name}")))
            .unwrap_or_else(|_| panic!("Failed to read workflow file: {name}"))
    }

    /// Create an action-format command for testing.
    pub fn command(&self) -> Command {
        let mut command = Command::new(get_bin());
        command.current_dir(self.root.path());
        command.env("NO_COLOR", "1");
        command
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_action-format"))
}

/// Common filters for snapshot testing.
pub static INSTA_FILTERS: &[(&str, &str)] = &[
    // Normalize Windows line endings
    (r"\r\n", "\n"),
    // Normalize Windows paths
    (r"\\", "/"),
    // Rewrite Windows output to Unix output
    (r"\\([\w\d]|\.)", "/$1"),
    (r"action-format\.exe", "action-format"),
    // Strip ANSI color codes (match ESC character using character class)
    (r"[\x1b]\[[0-9;]*m", ""),
    (r"\d+(?:;\d+)*m", ""),
];

pub fn apply_filters<T: AsRef<str>>(mut snapshot: String, filters: impl AsRef<[(T, T)]>) -> String {
    for (matcher, replacement) in filters.as_ref() {
        let re = Regex::new(matcher.as_ref()).expect("Invalid regex filter");
        if re.is_match(&snapshot) {
            snapshot = re.replace_all(&snapshot, replacement.as_ref()).to_string();
        }
    }
    snapshot
}

#[allow(clippy::print_stderr)]
pub fn run_and_format(
    cmd: &mut Command,
    filters: &[(&str, &str)],
    _test_name: &str,
) -> (String, std::process::Output) {
    let program = cmd.get_program().to_string_lossy().to_string();

    let output = cmd
        .output()
        .unwrap_or_else(|err| panic!("Failed to spawn {program}: {err}"));

    let snapshot = apply_filters(
        format!(
            "success: {:?}\nexit_code: {}\n----- stdout -----\n{}\n----- stderr -----\n{}",
            output.status.success(),
            output.status.code().unwrap_or(!0),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        ),
        filters,
    );

    (snapshot, output)
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

#[macro_export]
macro_rules! action_format_snapshot {
    ($cmd:expr, @$snapshot:literal) => {{
        action_format_snapshot!($crate::common::INSTA_FILTERS.to_vec(), $cmd, @$snapshot)
    }};
    ($filters:expr, $cmd:expr, @$snapshot:literal) => {{
        let (snapshot, output) = $crate::common::run_and_format(
            &mut $cmd,
            &$filters,
            $crate::function_name!(),
        );
        ::insta::assert_snapshot!(snapshot, @$snapshot);
        output
    }};
}

#[allow(unused_imports)]
pub(crate) use action_format_snapshot;
