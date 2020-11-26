# Contributing to Calypso

Unfortunately, we do not have a guide that describes the internals of Calypso. However, there are some people that are willing to mentor new users on easy issues. Issues that have a mentor are tagged with `E-mentor`. If you'd like to be a mentor, look for issues that are marked as `E-needs-mentor`, and ping `ThePuzzlemaker` to claim the mentorship for that issue.

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
  - `infra`: infrastructure code (e.g. `calypso_base` and `calypso_util`)
  - `diag`: diagnostics
  - `filety`: binary file types