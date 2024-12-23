use anyhow::{anyhow, Result};

const TAPE_SIZE: usize = 512;

/// A Brainfuck interpreter.
///
/// To get started, instantiate an interpreter with [`Interpreter::new`] or
/// [`Interpreter::from_program`], then run the program with
/// [`Interpreter::run`].
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

    pub program: Vec<char>,
    program_pointer: usize,

    loop_stack: Vec<usize>,
}

impl Interpreter {
    /// Return a new, empty interpreter.
    pub fn new() -> Interpreter {
        Interpreter {
            tape: [0; TAPE_SIZE],
            tape_pointer: 0,

            program: Vec::new(),
            program_pointer: 0,

            loop_stack: Vec::new(),
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
        while self.program_pointer < self.program.len() {
            let instruction = self.program[self.program_pointer];
            self.execute_instruction(instruction)?;
            self.program_pointer += 1;
        }
        Ok(())
    }

    /// Execute a single instruction.
    ///
    /// This does not advance the program pointer.
    fn execute_instruction(&mut self, instruction: char) -> Result<()> {
        match instruction {
            '>' => self.tape_pointer += 1,
            '<' => self.tape_pointer -= 1,

            _ => return Err(anyhow!("Unknown instruction: {instruction}")),
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
    fn increment_decrement() {
        let mut interpreter = Interpreter::from_program("+++--");
        interpreter.run().unwrap();
        assert_eq!(interpreter.tape[0], 1);
    }

    #[test]
    fn wrap_increment_decrement() {
        let mut interpreter = Interpreter::from_program("-");
        interpreter.run().unwrap();
        assert_eq!(interpreter.tape[0], 255);
    }

    #[test]
    fn loops() {
        let mut interpreter = Interpreter::from_program("+++++[->+<]");
        interpreter.run().unwrap();
        assert_eq!(interpreter.tape[1], 5);
    }
}
