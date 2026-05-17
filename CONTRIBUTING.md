# Contributing to Calypso

Unfortunately, we do not (yet) have a guide that describes the
internals of Calypso. However, there are some people that are willing
to mentor new users on easy issues. Issues that have a mentor are
tagged with `E-mentor`. If you'd like to be a mentor, look for issues
that are marked as `E-needs-mentor`, and ping `ThePuzzlemaker` to
claim the mentorship for that issue.

## Bug Reports / Feature Requests

To submit a bug report or feature request, please submit an issue using the applicable template.

## Writing "todo", "bug", "fixme", etc. comments

When writing code and you have a bug that needs further fixing or code that needs further work, please format your comment like this:
```
TYPE(@USER): MESSAGE
```
- Replace `TYPE` with the type of the comment, in all lowercase:
  - `hack`: hacky solutions that might be made better
  - `todo`: code that needs further work
  - `bug`/`fixme`: a bug that needs fixing
- Replace `USER` with your Codeberg username, e.g. `@ThePuzzlemaker`.
