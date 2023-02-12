# unnecessary-generator-list (C400)

Derived from the **flake8-comprehensions** linter.

Autofix is always available.

## What it does
Checks for unnecessary generators that can be rewritten as `list`
comprehensions.

## Why is this bad?
It is unnecessary to use `list` around a generator expression, since
there are equivalent comprehensions for these types. Using a
comprehension is clearer and more idiomatic.

## Examples
```python
list(f(x) for x in foo)
```

Use instead:
```python
[f(x) for x in foo]
```