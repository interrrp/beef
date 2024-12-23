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
