#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod interpreter;

/// A tiny Brainfuck interpreter.
#[derive(Parser, Debug)]
struct Args {
    /// Path of the Brainfuck program to execute.
    ///
    /// The filename typically ends in `.bf` or `.b`.
    program_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    dbg!(args);

    Ok(())
}
