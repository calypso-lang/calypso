<img src="https://raw.githubusercontent.com/calypso-lang/assets/main/logo/logo.png" alt="Calypso logo" width="250" align="right"/>

# Calypso

[![CI Status][b3]][l3] [![License][b4]][l4] [![Discord](https://img.shields.io/discord/822290196057948171)](https://discord.gg/26X6ChQQcG) ![Lines of Code][b2] <!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-2-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

Calypso is a mostly imperative language with some functional influences that is focused on flexibility and simplicity.

Note that this code is very work-in-progress. Contributions are welcome (and encouraged!), but it's not recommended to use this in production unless you're ready for some serious pain. Or code that just doesn't work.

## Example

The following example is an implementation of [FizzBuzz](https://en.wikipedia.org/wiki/Fizz_buzz) that goes until a number specified in the CLI arguments of the program, or 100 if that is not present. Note that this is currently psuedocode and may change.


```zig
import standard.process.Args

fn main(args: Args) ->
    args
    |> .get(0)
    |> .unwrap_or("100")
    |> uint.from_string
    |> .unwrap_or(100)
    |> fizzbuzz
    |> .each(&println("{}", &1))

fn fizzbuzz(max: uint): [string] ->
  (1..=max).map(fn n ->
    case do
      15.divides(n) -> "FizzBuzz",
      3.divides(n)  -> "Fizz",
      5.divides(n)  -> "Buzz",
      _             -> n.to_string
    end)
```

## Compatibility

The MSRV (Minimum Supported Rust Version) is currently Rust 1.48.0.

Calypso is automatically tested on:
- x86_64 linux, Rust 1.48.0
- x86_64 linux, latest stable
- x86_64 linux, latest nightly

as well as manually tested on x86_64 linux with the latest stable build of Rust

Calypso is developed and tested on these platforms:
- Arch Linux, latest stable
- I'd hope there's more in the future...but I don't currently have the infra to test those :(

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [crates.io][crates.io] statuses

I'm currently holding various crates related to Calypso. These will actually be used but they are being held so that no one uses them for malicious purposes or confuses anyone.

> Note: A version number of `0.0.0` indicates an unreleased crate.

| Crate Name             | Version                                | [docs.rs][docs.rs] Status                  |
|:----------------------:|:--------------------------------------:|:------------------------------------------:|
| `calypso`              | [![calypso][bcio1]][lcio1]             | [![calypso][bdrs1]][ldrs1]                 |
| `calypso_base`         | [![calypso_base][bcio2]][lcio2]        | [![calypso_base][bdrs2]][ldrs2]            |
| `calypso_error`        | [![calypso_error][bcio9]][lcio9]       | [![calypso_error][bdrs9]][ldrs9]           |
| `calypso_diagnostic`   | [![calypso_diagnostic][bcio3]][lcio3]  | [![calypso_diagnostic][bdrs3]][ldrs3]      |
| `calypso_filety`       | [![calypso_filety][bcio7]][lcio7]      | [![calypso_filety][bdrs7]][ldrs7]          |
| `calypso_opt`          | [![calypso_opt][bcio8]][lcio8]         | [![calypso_opt][bdrs8]][ldrs8]             |
| `calypso_parsing`      | [![calypso_parsing][bcio4]][lcio4]     | [![calypso_parsing][bdrs4]][ldrs4]         |
| `calypso_repl`         | [![calypso_repl][bcio5]][lcio5]        | [![calypso_repl][bdrs5]][ldrs5]            |
| `calypso_util`         | [![calypso_util][bcio6]][lcio6]        | [![calypso_util][bdrs6]][ldrs6]            |

For the status of SaturnVM crates, see [its README](libs/saturnvm/README.md).

[bcio1]: https://img.shields.io/crates/v/calypso
[lcio1]: https://crates.io/crates/calypso
[bdrs1]: https://docs.rs/calypso/badge.svg
[ldrs1]: https://docs.rs/calypso/*

[bcio2]: https://img.shields.io/crates/v/calypso_base
[lcio2]: https://crates.io/crates/calypso_base
[bdrs2]: https://docs.rs/calypso_base/badge.svg
[ldrs2]: https://docs.rs/calypso_base/*

[bcio3]: https://img.shields.io/crates/v/calypso_diagnostic
[lcio3]: https://crates.io/crates/calypso_diagnostic
[bdrs3]: https://docs.rs/calypso_diagnostic/badge.svg
[ldrs3]: https://docs.rs/calypso_diagnostic/*

[bcio4]: https://img.shields.io/crates/v/calypso_parsing
[lcio4]: https://crates.io/crates/calypso_parsing
[bdrs4]: https://docs.rs/calypso_parsing/badge.svg
[ldrs4]: https://docs.rs/calypso_parsing/*

[bcio5]: https://img.shields.io/crates/v/calypso_repl
[lcio5]: https://crates.io/crates/calypso_repl
[bdrs5]: https://docs.rs/calypso_repl/badge.svg
[ldrs5]: https://docs.rs/calypso_repl/*calypso_util

[bcio6]: https://img.shields.io/crates/v/calypso_util
[lcio6]: https://crates.io/crates/calypso_util
[bdrs6]: https://docs.rs/calypso_util/badge.svg
[ldrs6]: https://docs.rs/calypso_util/*

[bcio7]: https://img.shields.io/crates/v/calypso_filety
[lcio7]: https://crates.io/crates/calypso_filety
[bdrs7]: https://docs.rs/calypso_filety/badge.svg
[ldrs7]: https://docs.rs/calypso_filety/*

[bcio8]: https://img.shields.io/crates/v/calypso_opt
[lcio8]: https://crates.io/crates/calypso_opt
[bdrs8]: https://docs.rs/calypso_opt/badge.svg
[ldrs8]: https://docs.rs/calypso_opt/*

[bcio9]: https://img.shields.io/crates/v/calypso_error
[lcio9]: https://crates.io/crates/calypso_error
[bdrs9]: https://docs.rs/calypso_error/badge.svg
[ldrs9]: https://docs.rs/calypso_error/*

Docs for the `main` branch can be found [here](https://calypso-lang.github.io/rustdoc/calypso/index.html).

## SaturnVM

SaturnVM is the VM I'm writing for Calypso. It's meant to be mostly standalone but at the moment there may be some connections or dependencies between it and Calypso. SaturnVM's top-level crate can be found in `libs/saturnvm` and some of it's subcrates will be found in `libs/` under the name `saturnvm_*`. The `calypso_vm` crate will probably be a Calypso-specific interface to SaturnVM.

For more information, see [its README](libs/saturnvm/README.md).

## Logo Credits

The logo was modified from an image by [Freepik](https://www.freepik.com) on [Flaticon](https://www.flaticon.com). The original image can be found [here](https://www.flaticon.com/free-icon/saturn_124559) or in [`logo-base.svg`](https://raw.githubusercontent.com/calypso-lang/assets/main/logo/logo-base.svg) in the [assets](https://github.com/calypso-lang/assets) repository.

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://thepuzzlemaker.info/"><img src="https://avatars3.githubusercontent.com/u/12666617?v=4?s=100" width="100px;" alt=""/><br /><sub><b>James [Undefined]</b></sub></a><br /><a href="https://github.com/calypso-lang/calypso/commits?author=ThePuzzlemaker" title="Code">💻</a> <a href="#design-ThePuzzlemaker" title="Design">🎨</a> <a href="https://github.com/calypso-lang/calypso/commits?author=ThePuzzlemaker" title="Documentation">📖</a> <a href="#example-ThePuzzlemaker" title="Examples">💡</a> <a href="#ideas-ThePuzzlemaker" title="Ideas, Planning, & Feedback">🤔</a> <a href="#infra-ThePuzzlemaker" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#maintenance-ThePuzzlemaker" title="Maintenance">🚧</a> <a href="#projectManagement-ThePuzzlemaker" title="Project Management">📆</a> <a href="https://github.com/calypso-lang/calypso/commits?author=ThePuzzlemaker" title="Tests">⚠️</a> <a href="#tool-ThePuzzlemaker" title="Tools">🔧</a> <a href="#tutorial-ThePuzzlemaker" title="Tutorials">✅</a></td>
    <td align="center"><a href="https://github.com/HTG-YT"><img src="https://avatars.githubusercontent.com/u/39023054?v=4?s=100" width="100px;" alt=""/><br /><sub><b>HTG-YT</b></sub></a><br /><a href="https://github.com/calypso-lang/calypso/commits?author=HTG-YT" title="Documentation">📖</a> <a href="#tutorial-HTG-YT" title="Tutorials">✅</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!

[b2]: https://tokei.rs/b1/github/calypso-lang/calypso?category=lines

[b3]: https://github.com/calypso-lang/calypso/actions/workflows/ci.yml/badge.svg?branch=main&event=push
[l3]: https://github.com/calypso-lang/calypso/actions

[b4]: https://img.shields.io/badge/license-MIT-blue.svg
[l4]: ./LICENSE

[crates.io]: https://crates.io/
[docs.rs]: https://docs.rs/
