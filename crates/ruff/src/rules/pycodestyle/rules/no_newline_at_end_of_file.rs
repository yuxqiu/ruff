use rustpython_parser::ast::Location;

use ruff_diagnostics::{AlwaysAutofixableViolation, Diagnostic, Fix};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::source_code::Stylist;
use ruff_python_ast::types::Range;

#[violation]
pub struct NoNewLineAtEndOfFile;

impl AlwaysAutofixableViolation for NoNewLineAtEndOfFile {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("No newline at end of file")
    }

    fn autofix_title(&self) -> String {
        "Add trailing newline".to_string()
    }
}

/// W292
pub fn no_newline_at_end_of_file(
    stylist: &Stylist,
    contents: &str,
    autofix: bool,
) -> Option<Diagnostic> {
    if !contents.ends_with('\n') {
        // Note: if `lines.last()` is `None`, then `contents` is empty (and so we don't
        // want to raise W292 anyway).
        if let Some(line) = contents.lines().last() {
            // Both locations are at the end of the file (and thus the same).
            let location = Location::new(contents.lines().count(), line.len());
            let mut diagnostic =
                Diagnostic::new(NoNewLineAtEndOfFile, Range::new(location, location));
            if autofix {
                diagnostic.amend(Fix::insertion(stylist.line_ending().to_string(), location));
            }
            return Some(diagnostic);
        }
    }
    None
}
