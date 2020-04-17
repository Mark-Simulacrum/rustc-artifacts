# Rustc CI Artifact listing

This crate is primarily written for use in tooling that intends to enumerate all
available rustc CI artifacts (such as cargo-bisect-rustc, perf.rust-lang.org,
among others).

We currently provide just master commits via a semi-private API; this crate
should continue to work, but may need to be updated to do so (including across
breaking "major" versions).
