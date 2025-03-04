//! [![github]](https://github.com/dtolnay/linkme)&ensp;[![crates-io]](https://crates.io/crates/linkme)&ensp;[![docs-rs]](https://docs.rs/linkme)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! **A library for safe cross-platform linker shenanigans.**
//!
//! <br>
//!
//! # Platform support
//!
//! | Component | Linux | macOS | Windows | Other...<sup>†</sup> |
//! |:---|:---:|:---:|:---:|:---:|
//! | Distributed slice | ✅ | ✅ | ✅ | |
//!
//! <br>***<sup>†</sup>*** We welcome PRs adding support for any platforms not
//! listed here.
//!
//! <br>
//!
//! # Distributed slice
//!
//! A distributed slice is a collection of static elements that are gathered
//! into a contiguous section of the binary by the linker. Slice elements may be
//! defined individually from anywhere in the dependency graph of the final
//! binary.
//!
//! Refer to [`linkme::DistributedSlice`][DistributedSlice] for complete details
//! of the API. The basic idea is as follows.
//!
//! A static distributed slice is declared by writing `#[distributed_slice]` on
//! a static item whose type is `[T]` for some type `T`. The initializer
//! expression must be `[..]` to indicate that elements come from elsewhere.
//!
//! ```
//! # struct Bencher;
//! #
//! use linkme::distributed_slice;
//!
//! #[distributed_slice]
//! pub static BENCHMARKS: [fn(&mut Bencher)] = [..];
//! ```
//!
//! Slice elements may be registered into a distributed slice by a
//! `#[distributed_slice(...)]` attribute in which the path to the distributed
//! slice is given in the parentheses. The initializer is required to be a const
//! expression.
//!
//! ```
//! # mod other_crate {
//! #     use linkme::distributed_slice;
//! #
//! #     pub struct Bencher;
//! #
//! #     #[distributed_slice]
//! #     pub static BENCHMARKS: [fn(&mut Bencher)] = [..];
//! # }
//! #
//! # use other_crate::Bencher;
//! #
//! use linkme::distributed_slice;
//! use other_crate::BENCHMARKS;
//!
//! #[distributed_slice(BENCHMARKS)]
//! static BENCH_DESERIALIZE: fn(&mut Bencher) = bench_deserialize;
//!
//! fn bench_deserialize(b: &mut Bencher) {
//!     /* ... */
//! }
//! ```
//!
//! The distributed slice behaves in all ways like `&'static [T]`.
//!
//! ```no_run
//! # use linkme::distributed_slice;
//! #
//! # struct Bencher;
//! #
//! # #[distributed_slice]
//! # static BENCHMARKS: [fn(&mut Bencher)] = [..];
//! #
//! fn main() {
//!     // Iterate the elements.
//!     for bench in BENCHMARKS {
//!         /* ... */
//!     }
//!
//!     // Index into the elements.
//!     let first = BENCHMARKS[0];
//!
//!     // Slice the elements.
//!     let except_first = &BENCHMARKS[1..];
//!
//!     // Invoke methods on the underlying slice.
//!     let len = BENCHMARKS.len();
//! }
//! ```

#![no_std]
#![doc(html_root_url = "https://docs.rs/linkme/0.2.4")]
#![allow(
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::expl_impl_clone_on_copy,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::unused_self
)]

mod distributed_slice;

#[doc(hidden)]
pub mod private;

pub use linkme_impl::*;

pub use crate::distributed_slice::DistributedSlice;
