# Contributing to Calypso

Unfortunately, we do not have a guide that describes the internals of Calypso. However, there are some people that are willing to mentor new users on easy issues. Issues that have a mentor are tagged with `E-mentor`. If you'd like to be a mentor, look for issues that are marked as `E-needs-mentor`, and ping `ThePuzzlemaker` to claim the mentorship for that issue.

## Commits

You may use any style of commit message that you wish to use, as long as your commit messages follow these basic rules:
- Separate subject from body with a blank line
- Limit the subject line to 50 characters
- Capitalize the subject line
- Do not end the subject line with a period
- Use the imperative mood in the subject line
- Wrap the body at 72 characters
- Use the body to explain *what* and *why* vs. *how*

See [this article](https://chris.beams.io/posts/git-commit/) for an explanation of these rules.

A few of these rules can be checked using the `commit-msg` hook that you can install via `.etc/install-hooks.sh` or by manually copying `.etc/commit-msg` into your `.git/hooks`. Note that these will require Python 3.8.

There is also a `pre-commit` git hook in `.etc` that will run `cargo check`, `cargo test`, and check your code for formatting errors before committing. This is mostly for committing to the main branch, but it can be helpful to have well-tested commits in a PR.

## Bug Reports / Feature Requests

To submit a bug report or feature request, please submit an issue using the applicable template.

## Writing "todo", "bug", "fixme", etc. comments

When writing code and you have a bug that needs further fixing or code that needs further work, please format your comment like this:
```
TYPE(@USER: CATEGORY): MESSAGE
```
- Replace `TYPE` with the type of the comment, in all lowercase:
  - `hack`:  hacky solutions that might be made better
  - `todo`:  code that needs further work
  - `bug`:   a bug that needs fixing
  - `fixme`: something (not a bug, e.g. edge case) requires fixing
- Replace `USER` with your GitHub username, e.g. `@ThePuzzlemaker`.
- Replace `CATEGORY` with the category of the comment, currently these:
  - `doc`: documentation, whether in the book or just rustdoc
  - `repl`: REPL, whether in its implementation or usage
  - `parse`: parsing and lexing
  - `frame`: framework code (e.g. `calypso_base` and `calypso_util`)
  - `diag`: diagnostics
  - `filety`: binary file types

You may also find "PRODUCTIVITY MARKER" comments which are basically just notes to myself that say:
```
// NOTE: PRODUCTIVITY MARKER: This is purposefully incomplete. I'll come back to it later.
```

Basically, they mark things that are work-in-progress and don't have to be complete now. Productivity!

## Import Order

Imports should go in this order (in separate blocks):
1. Standard library imports (`std`, `core`, `alloc`, etc.)
2. External crate imports, excluding Calypso subcrates (e.g. `regex::Regex`). These may be in separate blocks.
3. Calypso subcrates (e.g. `calypso_parsing`)
4. Local imports (e.g. `super::Foo`, `crate::foo::Bar`)
5. All re-exports (e.g. `pub use foo::bar`, `pub use super::Bar`)

## Versioning Policy

Subcrates (crates in `libs/`) may publish versions independently of all other crates.
No version of any crate published to crates.io may contain build issues. The `main`
branch, however, may contain **temporary** build issues due to breaking changes in a
subcrate. When bumping a subcrate's version, please bump the dependents' version of
that subcrate. If it is possible for you, please fix any compatibility issues. If
not, please ping a relevant team member for that subcrate if your PR is accepted
and they will fix it before the PR is merged.

## Notes to myself

If anyone joins as a major contributor, this will no longer be notes to myself
but notes to the core team.

- Please do not assign milestones to non-tracking issues.
