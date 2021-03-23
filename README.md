<img src="https://raw.githubusercontent.com/calypso-lang/assets/main/logo/logo.png" alt="Calypso logo" width="250" align="right"/>

# Calypso

[![CI Status][b3]][l3] [![License][b4]][l4] [![Discord](https://img.shields.io/discord/822290196057948171)](https://discord.gg/26X6ChQQcG) ![Lines of Code][b2] <!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-2-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->
> # :warning: WARNING :warning:
> This is currently just a hodgepodge of parts that work and others that don't. DO NOT ASSUME **ANY** OF THIS CODE IS RELIABLE OR READY FOR PRODUCTION IN ANY WAY UNLESS OTHERWISE SPECIFIED!

Calypso is a mostly imperative language with some functional influences that is focused on flexibility and simplicity.

## Example

The following example is an implementation of [FizzBuzz](https://en.wikipedia.org/wiki/Fizz_buzz) that goes until a number specified in the CLI arguments of the program, or 100 if that is not present. Note that this is currently psuedocode and may change.


```
import atlas.process

@spec main(process.Args) -> :ok | (:error, Error)
fn main(args) ->
    args[0]
    |> Nullable.get_or!("100")
    |> u16.parse!
    |> fizzbuzz
    |> Iter.each(&println/1)
end

@spec fizzbuzz(u16) -> [string]
fn fizzbuzz(max) ->
  (1..=max).map(fn n -> 
    case do
      15.divides(n) -> "FizzBuzz",
      3.divides(n)  -> "Fizz",
      5.divides(n)  -> "Buzz",
      _             -> n.to_string
    end
  end)
end
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

This project is licensed under the MIT license ([LICENSE](LICENSE) or http://opensource.com/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you (i.e. submitted as a patch or pull request), shall be licensed as above, without any additional terms or conditions.

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

For the status of Odysseus crates, see [its README](libs/odysseus/README.md).

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

## Odysseus

Odysseus is the VM I'm writing for Calypso. It's meant to be mostly standalone but at the moment there may be some connections or dependencies between it and Calypso. Odysseus's top-level crate can be found in `libs/odysseus` and some of it's subcrates will be found in `libs/` under the name `odysseus_*`. The `calypso_vm` crate will probably be a Calypso-specific interface to Odyssey.

For more information, see [its README](libs/odysseus/README.md).

## Logo Credits

The logo was modified from an image made by [OpenClipart-Vectors](https://pixabay.com/users/openclipart-vectors-30363/) from [Pixabay](https://pixabay.com/)

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

[b3]: https://github.com/calypso-lang/calypso/actions/workflows/main.yml/badge.svg?branch=main&event=push
[l3]: https://github.com/calypso-lang/calypso/actions

[b4]: https://img.shields.io/badge/license-MIT-blue.svg
[l4]: ./LICENSE

[crates.io]: https://crates.io/
[docs.rs]: https://docs.rs/
