use rustpython_parser::ast::Location;

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::types::Range;
use ruff_python_ast::whitespace::leading_space;

#[violation]
pub struct MixedSpacesAndTabs;

impl Violation for MixedSpacesAndTabs {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Indentation contains mixed spaces and tabs")
    }
}

/// E101
pub fn mixed_spaces_and_tabs(lineno: usize, line: &str) -> Option<Diagnostic> {
    let indent = leading_space(line);

    if indent.contains(' ') && indent.contains('\t') {
        Some(Diagnostic::new(
            MixedSpacesAndTabs,
            Range::new(
                Location::new(lineno + 1, 0),
                Location::new(lineno + 1, indent.chars().count()),
            ),
        ))
    } else {
        None
    }
}
