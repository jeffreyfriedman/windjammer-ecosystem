# Contributing to Windjammer Ecosystem

Thank you for helping make Windjammer Ecosystem better. The ecosystem is a set of idiomatic Windjammer apps and packages that others can copy and learn from.

## Ways to contribute

- Report bugs with a minimal reproduction.
- Suggest features and describe the problem they solve.
- Improve documentation and examples.
- Submit fixes, features and tests.
- Help others in issues and discussions.

## Before you start

1. Search the [issue tracker](https://github.com/jeffreyfriedman/windjammer-ecosystem/issues) for related reports.
2. For anything larger than a small fix, open an issue first so we can agree on the approach.
3. Read the README and `AGENTS.md` for the project's conventions.

## Reporting bugs

Open an issue with a clear title, what you expected, what happened, a minimal reproduction, your `wj --version` output and operating system, and the complete error output.

## Development setup

```bash
git clone https://github.com/YOUR_USERNAME/windjammer-ecosystem.git
cd windjammer-ecosystem
git checkout -b feature/your-change
```

You need the `wj` compiler at the commit recorded in the `[toolchain]` table of `wj.toml` (`wj doctor` prints the install command if yours differs).

## Testing

- Build and test one package: `cd packages/<name> && wj build src && wj test` (apps work the same way from `apps/<name>`).
- Every package must define at least one test: `scripts/assert-package-tests.sh`.
- No orphaned test files: `scripts/check-orphan-tests.sh`.
- Sources stay pure Windjammer: no `extern fn`, no `ffi/` directories under `apps/` or `packages/`.

See `docs/TESTING.md` for how tests are discovered.

Every behavior change comes with a test that fails without the change.

## Public hygiene gate

This repository is public. A repo-local gate fails when a tracked file name or file content matches a forbidden-term pattern (private project names, personal paths). Run it with `wj run --interpret --allow process --allow fs.read --allow env.read --allow console scripts/public_hygiene.wj`, and add `--self-test` to confirm the matcher fires on planted terms. The patterns live in `scripts/public-hygiene-patterns.txt`; CI runs both commands.

## Pull requests

1. Rebase on the latest default branch.
2. Keep commits small and focused; use conventional prefixes (`feat`, `fix`, `docs`, `refactor`, `test`, `chore`).
3. Run the checks above and update documentation for user-facing changes.
4. Open the pull request, describe what changed and why, and link the related issue.
5. Respond to review comments; questions are always welcome.

## Code of Conduct and security

Participation is covered by our [Code of Conduct](CODE_OF_CONDUCT.md). Report vulnerabilities privately as described in [SECURITY.md](SECURITY.md).

## License

Contributions are licensed under the same terms as the project: MIT or Apache-2.0, at your option (see `LICENSE-MIT` and `LICENSE-APACHE`).
