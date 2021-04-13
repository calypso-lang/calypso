# SaturnVM

[![CI Status][b3]][l3] [![License][b4]][l4] [![Discord](https://img.shields.io/discord/822290196057948171)](https://discord.gg/26X6ChQQcG) ![Lines of Code][b2]

SaturnVM is a bytecode VM written entirely in Rust (although there may eventually be some small parts in C). It's meant to be fast and performant but flexible and idiomatic. It's meant to be standalone but as it's written specifically for [Calypso](https://github.com/calypso-lang/calypso), there may be some connections between the two projects, though they should not be anything significant. The top-level crate for SaturnVM can be found in `/libs/saturnvm` (where `/` is the Calypso repository root). Its subcrates, if any, will be found in `/libs/` under the name `saturnvm_*`.

The crate `calypso_vm` is going to eventually be a more Calypso-specific interface to the VM.

Note that this code is very work-in-progress. Contributions are welcome (and encouraged!), but it's not recommended to use this in production unless you're ready for some serious pain. Or code that just doesn't work.

## Example - Bytecode Builder

The following example creates a module "foo". This is currently the only part of the bytecode builder actually implemented but items and blocks should follow the basic design of this API. Please note that this API is subject to change.

As GATs are not stable (and I'd like to use stable code for this as of now), the `&mut ctx` and `&ctx` references are required due to the nature of the traits behind the bytecode builder. Hopefully GATs will become stable Soon:tm: and then they can be used to make the code simpler and more ergonomic.

```rust
use saturnvm::bc::prelude::*;

fn foo() {
    let mut ctx = Context::new();
    let mod_foo = ctx
        .module("foo")
        .enter(&mut ctx, |b| {
            // Do some building here (not designed yet)
            b
        })
        .finish(&mut ctx);
    // In other code, later
    let mod_foo = ctx.module("foo").get(&ctx).unwrap();
}
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

I'm currently holding various crates related to SaturnVM. These will actually be used but they are being held so that no one uses them for malicious purposes or confuses anyone.

> Note: A version number of `0.0.0` indicates an unreleased crate.

| Crate Name             | Version                                | [docs.rs][docs.rs] Status                  |
|:----------------------:|:--------------------------------------:|:------------------------------------------:|
| `saturnvm`             | [![saturnvm][ody_bc]][ody_lc]          | [![saturnvm][ody_bd]][ody_ld]              |

[ody_bc]: https://img.shields.io/crates/v/saturnvm
[ody_bd]: https://docs.rs/calypso/badge.svg
[ody_lc]: https://crates.io/crates/saturnvm
[ody_ld]: https://docs.rs/saturnvm/*

## Contributors

The list of contributors made using the all-contributors specification can be found on the [main README for Calypso](/README.md).

[b2]: https://tokei.rs/b1/github/calypso-lang/calypso?category=lines

[b3]: https://github.com/calypso-lang/calypso/actions/workflows/ci.yml/badge.svg?branch=main&event=push
[l3]: https://github.com/calypso-lang/calypso/actions

[b4]: https://img.shields.io/badge/license-MIT-blue.svg
[l4]: ./LICENSE

[crates.io]: https://crates.io/
[docs.rs]: https://docs.rs/
