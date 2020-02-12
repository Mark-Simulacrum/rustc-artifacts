# Rustc CI Artifact listing

This crate is primarily written for use in tooling that intends to enumerate all
available rustc CI artifacts (such as cargo-bisect-rustc, perf.rust-lang.org,
among others).

Currently it depends on the GitHub API, but this will likely change in the
future, as without a GitHub API token the current code runs up against rate
limiting quite quickly (roughly ~6 runs per hour). Plus, the current code does
not work well to give a full list of commits, in particular excluding try
commits.
