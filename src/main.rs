#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use interpreter::Interpreter;

mod interpreter;

/// A tiny Brainfuck interpreter.
#[derive(Parser, Debug)]
struct Args {
    /// Path of the Brainfuck program to execute.
    ///
    /// The filename typically ends in `.b` or `.bf`, but `.b` is preferred as `.bf` often gets
    /// confused with Befunge.
    program_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let program = fs::read_to_string(&args.program_path)
        .context(format!("Failed to read {}", &args.program_path.display()))?;

    let mut interpreter = Interpreter::from_program(&program);
    interpreter.run()?;

    Ok(())
}
