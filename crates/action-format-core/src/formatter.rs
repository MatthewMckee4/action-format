use std::path::Path;

use crate::config::FormatterConfig;
use crate::parser::FormatError;

/// Format a YAML string according to the configuration.
pub fn format_string(content: &str, config: &FormatterConfig) -> Result<String, FormatError> {
    // First pass: detect the source indent size
    let source_indent = detect_indent_size(content);

    let mut output = String::with_capacity(content.len());
    let mut in_steps_section = false;
    let mut steps_indent: Option<usize> = None;
    let mut seen_first_step = false;
    let mut first_line = true;
    let mut prev_line_blank = false;

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim_start();
        let is_blank = trimmed.is_empty();

        // Calculate original indentation
        let original_indent = line.len() - trimmed.len();

        // Check for mixed tabs and spaces
        let indent_chars = &line[..original_indent];
        if indent_chars.contains('\t') && indent_chars.contains(' ') {
            return Err(FormatError::MixedIndentation { line: line_num + 1 });
        }

        // Convert tabs to spaces for calculation
        let indent_spaces = if indent_chars.contains('\t') {
            indent_chars
                .chars()
                .map(|c| if c == '\t' { 2 } else { 1 })
                .sum()
        } else {
            original_indent
        };

        // Detect steps: key at any level
        if trimmed.starts_with("steps:") {
            in_steps_section = true;
            steps_indent = None;
            seen_first_step = false;
        }

        // Detect if we're a step item (- at the steps level)
        let is_step_item = if in_steps_section && trimmed.starts_with("- ") {
            match steps_indent {
                None => {
                    steps_indent = Some(indent_spaces);
                    true
                }
                Some(expected) => indent_spaces == expected,
            }
        } else {
            false
        };

        // Check if we've exited the steps section
        if in_steps_section && !is_blank && !trimmed.starts_with('#') {
            if let Some(si) = steps_indent {
                if indent_spaces < si && !is_step_item {
                    if trimmed.contains(':') && !trimmed.starts_with('-') {
                        in_steps_section = false;
                        steps_indent = None;
                        seen_first_step = false;
                    }
                }
            }
        }

        // Add blank line before step items (except the first one, and only if not already blank)
        if config.separate_steps && is_step_item && seen_first_step && !prev_line_blank {
            output.push('\n');
        }

        // Update seen_first_step after we've used it for the blank line decision
        if is_step_item {
            seen_first_step = true;
        }

        // Normalize indentation
        let normalized_indent = normalize_indent(indent_spaces, source_indent, config.indent_size);

        if !first_line {
            output.push('\n');
        }
        first_line = false;

        // Write the line with normalized indentation (skip indent for blank lines)
        if !is_blank {
            for _ in 0..normalized_indent {
                output.push(' ');
            }
        }
        output.push_str(trimmed);

        prev_line_blank = is_blank;
    }

    // Preserve trailing newline if present
    if content.ends_with('\n') {
        output.push('\n');
    }

    Ok(output)
}

/// Detect the indent size used in the source file.
fn detect_indent_size(content: &str) -> usize {
    let mut min_indent: Option<usize> = None;

    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }

        let indent = line.len() - trimmed.len();
        if indent > 0 {
            min_indent = Some(match min_indent {
                None => indent,
                Some(current) => gcd(current, indent),
            });
        }
    }

    min_indent.unwrap_or(2)
}

/// Greatest common divisor.
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Normalize indentation from source indent size to target indent size.
fn normalize_indent(current: usize, source_size: usize, target_size: usize) -> usize {
    if current == 0 || source_size == 0 {
        return 0;
    }

    // Calculate the indent level based on source size, then apply target size
    let level = current / source_size;
    level * target_size
}

/// Format a YAML file in place.
pub fn format_file(path: &Path, config: &FormatterConfig) -> Result<bool, FormatError> {
    let content = std::fs::read_to_string(path)?;
    let formatted = format_string(&content, config)?;

    if content == formatted {
        return Ok(false);
    }

    std::fs::write(path, &formatted)?;
    Ok(true)
}
