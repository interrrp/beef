#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use interpreter::Interpreter;

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

    let program = fs::read_to_string(args.program_path)?;

    let mut interpreter = Interpreter::from_program(&program);
    interpreter.run()?;

    Ok(())
}
