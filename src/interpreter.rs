use std::io::{stdin, stdout, Read, StdinLock, StdoutLock, Write};

use anyhow::{anyhow, Context, Result};

const TAPE_SIZE: usize = 512;

/// A Brainfuck interpreter.
///
/// To get started, instantiate an interpreter with [`Interpreter::new`] or
/// [`Interpreter::from_program`], then run the program with [`Interpreter::run`].
///
/// # Example
///
/// ```
/// let mut interpreter = Interpreter::from_program(">><");
/// interpreter.run().unwrap();
/// ```
pub struct Interpreter {
    tape: [u8; TAPE_SIZE],
    tape_pointer: usize,

    program: Vec<char>,
    program_pointer: usize,

    loop_stack: Vec<usize>,
}

impl Interpreter {
    /// Return a new, empty interpreter.
    ///
    /// This locks stdin until this interpreter is dropped.
    pub fn new() -> Interpreter {
        Interpreter {
            tape: [0; TAPE_SIZE],
            tape_pointer: 0,

            program: Vec::new(),
            program_pointer: 0,

            loop_stack: Vec::with_capacity(16),
        }
    }

    /// Return an empty interpreter but with a program preloaded.
    pub fn from_program(program: &str) -> Interpreter {
        let mut interpreter = Interpreter::new();
        interpreter.program = program.chars().collect();
        interpreter
    }

    /// Run the program.
    pub fn run(&mut self) -> Result<()> {
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
    /// This does not advance the program pointer.
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

            '[' => self.loop_stack.push(self.program_pointer),
            ']' => {
                if *tape_val == 0 {
                    // Loop ends when the tape value at the pointer is 0
                    if self.loop_stack.pop().is_none() {
                        return Err(anyhow!("Unmatched ]"));
                    }
                } else if let Some(beginning) = self.loop_stack.last() {
                    self.program_pointer = *beginning;
                } else {
                    return Err(anyhow!("Unmatched ]"));
                }
            }

            '.' => write!(stdout, "{}", *tape_val as char)?,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_tape_pointer() {
        let mut interpreter = Interpreter::from_program(">><");
        interpreter.run().unwrap();
        assert_eq!(interpreter.tape_pointer, 1);
    }

    #[test]
    fn move_tape_pointer_wrap() {
        let mut interpreter = Interpreter::from_program("<");
        interpreter.run().unwrap();
        assert_eq!(interpreter.tape_pointer, TAPE_SIZE - 1);
    }

    #[test]
    fn increment_decrement() {
        let mut interpreter = Interpreter::from_program("+++--");
        interpreter.run().unwrap();
        assert_eq!(interpreter.tape[0], 1);
    }

    #[test]
    fn wrap_increment_decrement() {
        let mut interpreter = Interpreter::from_program("->[+]");
        interpreter.run().unwrap();
        assert_eq!(interpreter.tape[0], 255);
        assert_eq!(interpreter.tape[1], 0);
    }

    #[test]
    fn loops() {
        let mut interpreter = Interpreter::from_program("+++++[->+<]++");
        interpreter.run().unwrap();
        assert_eq!(interpreter.tape[0], 2);
        assert_eq!(interpreter.tape[1], 5);
    }

    #[test]
    fn unmatched_loop_error() {
        let mut interpreter = Interpreter::from_program("]");
        assert!(interpreter.run().is_err());
    }

    #[test]
    fn nested_loops() {
        let mut interpreter = Interpreter::from_program("++[->+[-++[->+[-]++[->+[-]]]]]");
        interpreter.run().unwrap();
        assert_eq!(interpreter.tape[0], 1);
        assert_eq!(interpreter.tape[1], 1);
        assert_eq!(interpreter.tape[2], 1);
        assert_eq!(interpreter.tape[3], 0);
        assert_eq!(interpreter.tape_pointer, 3);
    }
}
