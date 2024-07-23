# Changelog

# Version 0.0.14

- Added `LangaugeModelProvider::is_local` method

# Version 0.0.13

No functional changes.

# Version 0.0.12

- Fixed the proc macro for the variants of the response options ([PR #9](https://github.com/guywaldman/orch/pull/9))
- Added support for "alignment", improved documentation, added examples ([PR #11](https://github.com/guywaldman/orch/pull/11))

# Version 0.0.11

No functional changes.

# Version 0.0.10

No functional changes.

# Version 0.0.9

No functional changes.

# Version 0.0.8

- Added support for boolean fields in the response options

# Version 0.0.7

- Fixed issue where the `orch` crate was not used for types in the proc macros
- Fixed issue where multiple fields in a response option would fail the proc macro

# Version 0.0.6

No functional changes.

# Version 0.0.5

- Fixed an issue where the proc macros were not exposed directly from `orch`

# Version 0.0.4

No functional changes.

# Version 0.0.3

- Added support for streaming responses
- Added support for structured data generation
- Added a convenience proc macro (`#[derive(OrchResponseOptions)]`) for generating structured data generation options
- Added support for Open AI

## Version 0.0.2

No functional changes.

## Version 0.0.1

Initial release.
