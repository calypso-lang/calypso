#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/odysseus/index.html")]
#![warn(clippy::pedantic)]

use mimalloc::MiMalloc;

pub mod arena;
pub mod bc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
