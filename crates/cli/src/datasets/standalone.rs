//! Standalone, dependency-free build of the regression dataset engine for CI:
//!
//! ```sh
//! rustc --edition 2021 -O crates/cli/src/datasets/standalone.rs -o jarambit-datasets
//! rustc --edition 2021 --test crates/cli/src/datasets/standalone.rs -o datasets-tests
//! ```
//! It compiles the exact module files that `jarambit dataset` uses (no copies), so the CatBoost CV
//! job never needs the full workspace (faer, solvers) and can never drift from the CLI.
//! This file is never part of the `jarambit` crate.
// Items used only by `jarambit` (recorder, physical import) are dead in this subset by design.
#![allow(dead_code)]
mod csvio;
mod cv;
mod engine;
mod gfeat;
mod gitsync;
mod harvest;
mod json;
mod materialize;
mod recipes;
mod records;
mod sources;
mod stats;

#[cfg(not(test))]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first() else {
        println!("{}", engine::USAGE);
        std::process::exit(2);
    };
    if cmd == "-h" || cmd == "--help" {
        println!("{}", engine::USAGE);
        return;
    }
    match engine::run_pure(cmd, &args[1..]) {
        Some(code) => std::process::exit(code),
        None => {
            eprintln!("jarambit-datasets: `{cmd}` needs the full `jarambit` binary (CAD kernel) or is unknown\n\n{}", engine::USAGE);
            std::process::exit(2);
        }
    }
}
