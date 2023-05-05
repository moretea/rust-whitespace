use super::{Instruction, Label};
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;

pub struct Interpreter {
    ip: usize,
    program: Vec<Instruction>,
    labels: HashMap<Label, usize>,
    stack: Vec<i64>,
    call_stack: Vec<usize>,
    output_writer: Option<Box<dyn Write>>,
    halted: bool,
}

impl Interpreter {
    pub fn new(program: Vec<Instruction>, output_writer: Option<Box<dyn Write>>) -> Interpreter {
        Interpreter {
            ip: 0,
            program: program,
            labels: HashMap::new(),
            stack: Vec::new(),
            call_stack: Vec::new(),
            output_writer:  output_writer,
            halted: false,
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn step(&mut self) {
        use Instruction::*;

        let instruction = self.program[self.ip]; // TODO
        println!("Executing {:?}", instruction);

        self.ip += 1;
        match instruction {
            PushNrOnStack(nr) => { self.stack.push(nr)}
            OutputChar => {
                let nr = self.stack.pop().unwrap(); // TODO
                if nr >= 0 && nr <= 256 {
                    let c = nr as u8 as char;
                    print!("{:?}", c);
                }

            }
            _ => panic!("Unsupported instruction!")
        }

    }
}

#[cfg(test)]
mod test {
    use super::super::test_helper::prepare_example;
    use crate::{parser::parse};
    use super::Interpreter;

    #[test]
    fn test_run_hello_world() {
        use super::Instruction::*;

        let hello_world = prepare_example(include_str!("example_programs/hello_world.txt"));
        let program = parse(&hello_world).unwrap();
        let mut interpreter =  Interpreter::new(program, None);
        while ! interpreter.is_halted() {
            interpreter.step();
        }

    }

}

