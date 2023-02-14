use rustpython_parser::ast::{Expr, ExprKind};

use ruff_macros::{define_violation, derive_message_formats};

use crate::ast::types::{CallPath, Range};
use crate::registry::Diagnostic;
use crate::violation::Violation;

define_violation!(
    /// ## What it does
    /// Checks for `asyncio.create_task` calls that do not store a reference
    /// to the returned result.
    ///
    /// ## Why is this bad?
    /// Per the `asyncio` documentation, the event loop only retains a weak
    /// reference to tasks. If the task returned by `asyncio.create_task` is
    /// not stored in a variable, or a collection, or otherwise referenced, it
    /// may be garbage collected at any time. This can lead to unexpected and
    /// inconsistent behavior, as your tasks may or may not run to completion.
    ///
    /// ## Example
    /// ```python
    /// import asyncio
    ///
    /// for i in range(10):
    ///    # This creates a weak reference to the task, which may be garbage
    ///    # collected at any time.
    ///    asyncio.create_task(some_coro(param=i))
    /// ```
    ///
    /// Use instead:
    /// ```python
    /// import asyncio
    ///
    /// background_tasks = set()
    ///
    /// for i in range(10):
    ///     task = asyncio.create_task(some_coro(param=i))
    ///
    ///     # Add task to the set. This creates a strong reference.
    ///     background_tasks.add(task)
    ///
    ///     # To prevent keeping references to finished tasks forever,
    ///     # make each task remove its own reference from the set after
    ///     # completion:
    ///     task.add_done_callback(background_tasks.discard)
    /// ```
    ///
    /// ## References
    /// * [_The Heisenbug lurking in your async code_](https://textual.textualize.io/blog/2023/02/11/the-heisenbug-lurking-in-your-async-code/)
    /// * [`asyncio.create_task`](https://docs.python.org/3/library/asyncio-task.html#asyncio.create_task)
    pub struct AsyncioDanglingTask;
);

impl Violation for AsyncioDanglingTask {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Store a reference to the return value of `asyncio.create_task`")
    }
}

/// RUF006
pub fn asyncio_dangling_task<'a, F>(expr: &'a Expr, resolve_call_path: F) -> Option<Diagnostic>
where
    F: FnOnce(&'a Expr) -> Option<CallPath<'a>>,
{
    if let ExprKind::Call { func, .. } = &expr.node {
        if resolve_call_path(func).map_or(false, |call_path| {
            call_path.as_slice() == ["asyncio", "create_task"]
        }) {
            return Some(Diagnostic::new(
                AsyncioDanglingTask,
                Range::from_located(expr),
            ));
        }
    }
    None
}