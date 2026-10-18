# Repository instructions

- Keep generated evidence and committed certification inputs byte-for-byte stable unless the task explicitly owns them.
- Put extra working copies in `.agent/worktrees/<lane>` and build in each lane's own `target/` directory.
- Before editing, inspect the current worktree status and avoid overwriting unrelated changes.
- Use Rust for backend, CLI, CI, and compute code. Do not add Python files; existing Python exceptions are listed in `PYTHON_EXCEPTIONS.md`.
- Run the narrowest relevant tests for a change, then `git diff --check`. Before pushing, run the repository's required gate and verify the PR head.
- Keep security-sensitive evidence, scoring, and certification gates fail-closed. Do not weaken thresholds or substitute synthetic evidence for certified inputs.
