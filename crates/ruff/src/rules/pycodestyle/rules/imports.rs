use rustpython_parser::ast::{Alias, Stmt};

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::types::Range;

use crate::checkers::ast::Checker;

#[violation]
pub struct MultipleImportsOnOneLine;

impl Violation for MultipleImportsOnOneLine {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Multiple imports on one line")
    }
}

#[violation]
pub struct ModuleImportNotAtTopOfFile;

impl Violation for ModuleImportNotAtTopOfFile {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Module level import not at top of file")
    }
}

pub fn multiple_imports_on_one_line(checker: &mut Checker, stmt: &Stmt, names: &[Alias]) {
    if names.len() > 1 {
        checker
            .diagnostics
            .push(Diagnostic::new(MultipleImportsOnOneLine, Range::from(stmt)));
    }
}

pub fn module_import_not_at_top_of_file(checker: &mut Checker, stmt: &Stmt) {
    if checker.ctx.seen_import_boundary && stmt.location.column() == 0 {
        checker.diagnostics.push(Diagnostic::new(
            ModuleImportNotAtTopOfFile,
            Range::from(stmt),
        ));
    }
}
