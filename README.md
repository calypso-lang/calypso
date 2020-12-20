<img src="https://raw.githubusercontent.com/calypso-lang/assets/main/logo/logo.png" alt="Calypso logo" width="250" align="right"/>

# Calypso

[![CI Status][b3]][l3] [![License][b4]][l4] [![Discord][b1]][l1] ![Lines of Code][b2] <!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->
> # :warning: WARNING :warning:
> This is currently just a hodgepodge of parts that work and others that don't. DO NOT ASSUME **ANY** OF THISE CODE IS RELIABLE OR READY FOR PRODUCTION IN ANY WAY UNLESS OTHERWISE SPECIFIED!

Calypso is a mostly imperative language with some functional influences that is focused on flexibility and simplicity.

## Example

The following example is an implementation of [FizzBuzz](https://en.wikipedia.org/wiki/Fizz_buzz) that goes until a number specified in the CLI arguments of the program, or 100 if that is not present.

```
import atlas.env

fn fizzbuzz(max) {
    (1..=max).map(n -> {
        case: 15.divides(n) -> "FizzBuzz",
              3.divides(n)  -> "Fizz",
              5.divides(n)  -> "Buzz",
              _              -> n.to_string
    })
}

fn main() {
    env.args()
    |> Iter.get?(1)
    |> Option.unwrap_or!("100")
    |> uint.parse!
    |> fizzbuzz
    |> Iter.each(&println)
}
```

## Compatibility

The MSRV (Minimum Supported Rust Version) is currently Rust 1.48.0.

Calypso is developed and tested on these platforms:
- Arch Linux
- I'd hope there's more in the future...but I don't currently have the infra to test those :(

## License

This project is licensed under the MIT license ([LICENSE](LICENSE) or http://opensource.com/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you (i.e. submitted as a patch or pull request), shall be licensed as above, without any additional terms or conditions.

## [crates.io][crates.io] statuses

> Note: A version number of `0.0.0` indicates an unreleased crate.

| Crate Name             | Version                                | [docs.rs][docs.rs] Status                  |
|:----------------------:|:--------------------------------------:|:------------------------------------------:|
| `calypso`              | [![calypso][bcio1]][lcio1]             | [![calypso][bdrs1]][ldrs1]                 |
| `calypso_base`         | [![calypso_base][bcio2]][lcio2]        | [![calypso_base][bdrs2]][ldrs2]            |
| `calypso_diagnostic`   | [![calypso_diagnostic][bcio3]][lcio3]  | [![calypso_diagnostic][bdrs3]][ldrs3]      |
| `calypso_filety`       | [![calypso_filety][bcio7]][lcio7]      | [![calypso_filety][bdrs7]][ldrs7]          |
| `calypso_opt`          | [![calypso_opt][bcio8]][lcio8]         | [![calypso_opt][bdrs8]][ldrs8]             |
| `calypso_parsing`      | [![calypso_parsing][bcio4]][lcio4]     | [![calypso_parsing][bdrs4]][ldrs4]         |
| `calypso_repl`         | [![calypso_repl][bcio5]][lcio5]        | [![calypso_repl][bdrs5]][ldrs5]            |
| `calypso_util`         | [![calypso_util][bcio6]][lcio6]        | [![calypso_util][bdrs6]][ldrs6]            |

Docs for the `main` branch can be found [here](https://calypso-lang.github.io/rustdoc/calypso/index.html).

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
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!

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

[l1]: https://discord.gg/jRaYGgeeJF
[b1]: https://img.shields.io/badge/discord-%23calypso-informational

[b2]: https://tokei.rs/b1/github/calypso-lang/calypso?category=lines

[b3]: https://img.shields.io/travis/com/calypso-lang/calypso
[l3]: https://travis-ci.com/github/calypso-lang/calypso

[b4]: https://img.shields.io/badge/license-MIT-blue.svg
[l4]: ./LICENSE

[crates.io]: https://crates.io/
[docs.rs]: https://docs.rs/
