# SaturnVM

[![CI Status][b3]][l3] [![License][b4]][l4] [![Discord](https://img.shields.io/discord/822290196057948171?style=flat-square&color=blue)](https://discord.gg/26X6ChQQcG) ![Lines of Code][b2]

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

For compatibility information please see the [corresponding section of Calypso's README](https://github.com/calypso-lang/calypso/#compatibility). 

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

I'm currently holding various crates related to SaturnVM. These will actually be used but they are being held so that no one uses them for malicious purposes or confuses anyone.

> Note: A version number of `0.0.0` indicates an unreleased crate.

| Crate Name             | Version                                | [docs.rs][docs.rs] Status                  |
|:----------------------:|:--------------------------------------:|:------------------------------------------:|
| `saturnvm`             | [![saturnvm][svm_bc]][svm_lc]          | [![saturnvm][svm_bd]][svm_ld]              |

[svm_bc]: https://img.shields.io/crates/v/saturnvm
[svm_bd]: https://docs.rs/calypso/badge.svg
[svm_lc]: https://crates.io/crates/saturnvm
[svm_ld]: https://docs.rs/saturnvm/*

## Contributors

The list of contributors made using the all-contributors specification can be found on the [main README for Calypso](https://github.com/calypso-lang/calypso/#compatibility).

[b2]: https://img.shields.io/tokei/lines/github/calypso-lang/calypso?style=flat-square

[b3]: https://img.shields.io/github/workflow/status/calypso-lang/calypso/CI?style=flat-square
[l3]: https://github.com/calypso-lang/calypso/actions

[b4]: https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square
[l4]: ./LICENSE

[crates.io]: https://crates.io/
[docs.rs]: https://docs.rs/
