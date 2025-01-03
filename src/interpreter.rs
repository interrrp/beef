use std::io::{stdin, stdout, Read, StdinLock, StdoutLock, Write};

use anyhow::{anyhow, Context, Result};

const TAPE_SIZE: usize = 30_000;

/// A Brainfuck interpreter.
///
/// To get started, instantiate an interpreter with [`Interpreter::new`] or
/// [`Interpreter::from_program_str`], then run the program with [`Interpreter::run`].
///
/// # Example
///
/// ```
/// let mut interpreter = Interpreter::from_program_str(">><");
/// interpreter.run().unwrap();
/// ```
pub struct Interpreter {
    tape: [u8; TAPE_SIZE],
    tape_pointer: usize,

    program: Vec<char>,
    program_pointer: usize,

    /// Precomputed bracket map, calculated on every call to `run`.
    ///
    /// This maps starting brackets to their corresponding ending brackets, and vice versa. For
    /// example: if a starting bracket at index 1 has its closing bracket at index 3, the index of
    /// the closing bracket can be retrieved by `bracket_map[1]`, and the index of the starting
    /// bracket at `bracket_map[3]`.
    ///
    /// This exists for both convenience and performance, as it allows for O(1) loop indexing during
    /// runtime while only requiring one line.
    bracket_map: Vec<usize>,
}

impl Interpreter {
    /// Return a new, empty interpreter.
    pub fn new() -> Interpreter {
        Interpreter {
            tape: [0; TAPE_SIZE],
            tape_pointer: 0,

            program: Vec::new(),
            program_pointer: 0,

            bracket_map: Vec::new(),
        }
    }

    /// Return an empty interpreter with a program preloaded.
    pub fn from_program_str(program: &str) -> Interpreter {
        let mut interpreter = Interpreter::new();
        interpreter.program = program.chars().collect();
        interpreter
    }

    /// Run the program.
    ///
    /// This locks stdin and stdout until execution finishes.
    ///
    /// An error is returned:
    ///
    /// - Immediately, if there is an unmatched loop bracket
    /// - At runtime, if unable to read from stdin or write to stdout
    pub fn run(&mut self) -> Result<()> {
        self.compute_bracket_map()?;

        let mut stdin = stdin().lock();
        let mut stdout = stdout().lock();

        while self.program_pointer < self.program.len() {
            let instruction = self.program[self.program_pointer];
            self.execute_instruction(instruction, &mut stdin, &mut stdout)?;
            self.program_pointer += 1;
        }

        Ok(())
    }

    /// Execute a single instruction.
    ///
    /// An error is returned if:
    ///
    /// - The instruction is `.`, and writing to stdout fails
    /// - The instruction is `,`, and reading from stdin fails
    fn execute_instruction(
        &mut self,
        instruction: char,
        stdin: &mut StdinLock,
        stdout: &mut StdoutLock,
    ) -> Result<()> {
        let tape_val = &mut self.tape[self.tape_pointer];

        match instruction {
            '>' => self.tape_pointer = (self.tape_pointer + 1) % TAPE_SIZE,
            '<' => self.tape_pointer = (self.tape_pointer + TAPE_SIZE - 1) % TAPE_SIZE,

            '+' => *tape_val = tape_val.wrapping_add(1),
            '-' => *tape_val = tape_val.wrapping_sub(1),

            '[' if *tape_val == 0 => self.program_pointer = self.bracket_map[self.program_pointer],
            ']' if *tape_val != 0 => self.program_pointer = self.bracket_map[self.program_pointer],

            '.' => {
                write!(stdout, "{}", *tape_val as char)?;
                stdout.flush()?;
            }
            ',' => {
                *tape_val = stdin
                    .bytes()
                    .next()
                    .context("Failed to read character from stdin")??;
            }

            _ => {}
        }

        Ok(())
    }

    /// Compute the loop bracket map.
    ///
    /// An error is returned if there is an unmatched bracket.
    fn compute_bracket_map(&mut self) -> Result<()> {
        self.bracket_map = vec![0; self.program.len()];
        let mut stack = Vec::new();

        for (i, &ch) in self.program.iter().enumerate() {
            match ch {
                '[' => stack.push(i),
                ']' => {
                    if let Some(open_index) = stack.pop() {
                        self.bracket_map[open_index] = i;
                        self.bracket_map[i] = open_index;
                    } else {
                        return Err(anyhow!("Unmatched ] at {i}"));
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(program: &str) -> Interpreter {
        let mut interpreter = Interpreter::from_program_str(program);
        interpreter.run().unwrap();
        interpreter
    }

    #[test]
    fn move_tape_pointer() {
        let interpreter = run(">><");
        assert_eq!(interpreter.tape_pointer, 1);
    }

    #[test]
    fn move_tape_pointer_wrap() {
        let interpreter = run("<");
        assert_eq!(interpreter.tape_pointer, TAPE_SIZE - 1);
    }

    #[test]
    fn increment_decrement() {
        let interpreter = run("+++--");
        assert_eq!(interpreter.tape[0], 1);
    }

    #[test]
    fn wrap_increment_decrement() {
        let interpreter = run("->[+]");
        assert_eq!(interpreter.tape[0], 255);
        assert_eq!(interpreter.tape[1], 0);
    }

    #[test]
    fn loops() {
        let interpreter = run("+++++[->+<]++");
        assert_eq!(interpreter.tape[0], 2);
        assert_eq!(interpreter.tape[1], 5);
    }

    #[test]
    fn skip_loop_if_zero() {
        let interpreter = run("[+++]");
        assert_eq!(interpreter.tape[0], 0);
    }

    #[test]
    fn unmatched_loop_error() {
        let mut interpreter = Interpreter::from_program_str("]");
        assert!(interpreter.run().is_err());
    }

    #[test]
    fn nested_loops() {
        let interpreter = run("++[->+[-++[->+[-]++[->+[-]]]]]");
        assert_eq!(interpreter.tape[0], 1);
        assert_eq!(interpreter.tape[1], 1);
        assert_eq!(interpreter.tape[2], 1);
        assert_eq!(interpreter.tape[3], 0);
        assert_eq!(interpreter.tape_pointer, 3);
    }
}
