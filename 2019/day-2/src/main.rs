/*!
Link: https://adventofcode.com/2019/day/2

 --- Day 2: 1202 Program Alarm ---
On the way to your gravity assist around the Moon, your ship computer beeps angrily about a "1202 program alarm".
On the radio, an Elf is already explaining how to handle the situation: "Don't worry, that's perfectly norma--"
The ship computer bursts into flames.

You notify the Elves that the computer's magic smoke seems to have escaped.
"That computer ran Intcode programs like the gravity assist program it was working on;
surely there are enough spare parts up there to build a new Intcode computer!"

An Intcode program is a list of integers separated by commas (like 1,0,0,3,99). 
To run one, start by looking at the first integer (called position 0).
Here, you will find an opcode - either 1, 2, or 99.
The opcode indicates what to do; for example, 99 means that the program is finished and should immediately halt.
Encountering an unknown opcode means something went wrong.

Opcode 1 adds together numbers read from two positions and stores the result in a third position.
The three integers immediately after the opcode tell you these three positions - the first two indicate
the positions from which you should read the input values, and the third indicates the position at which
the output should be stored.

For example, if your Intcode computer encounters 1,10,20,30, it should read the values at positions 10 and 20,
add those values, and then overwrite the value at position 30 with their sum.

Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them.
Again, the three integers after the opcode indicate where the inputs and outputs are, not their values.

Once you're done processing an opcode, move to the next one by stepping forward 4 positions.

For example, suppose you have the following program:

    1,9,10,3,2,3,11,0,99,30,40,50

For the purposes of illustration, here is the same program split into multiple lines:

    1,9,10,3,
    2,3,11,0,
    99,
    30,40,50

The first four integers, 1,9,10,3, are at positions 0, 1, 2, and 3.
Together, they represent the first opcode (1, addition), the positions of the two inputs (9 and 10),
and the position of the output (3). To handle this opcode, you first need to get the values at the
input positions: position 9 contains 30, and position 10 contains 40.
Add these numbers together to get 70.
Then, store this value at the output position; here, the output position (3) is at position 3, so it overwrites itself.
Afterward, the program looks like this:

    1,9,10,70,
    2,3,11,0,
    99,
    30,40,50

Step forward 4 positions to reach the next opcode, 2.
This opcode works just like the previous, but it multiplies instead of adding.
The inputs are at positions 3 and 11; these positions contain 70 and 50 respectively.
Multiplying these produces 3500; this is stored at position 0:

    3500,9,10,70,
    2,3,11,0,
    99,
    30,40,50

Stepping forward 4 more positions arrives at opcode 99, halting the program.

Here are the initial and final states of a few more small programs:

    - 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
    - 2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
    - 2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
    - 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.

Once you have a working computer, the first step is to restore the gravity assist program (your puzzle input)
to the "1202 program alarm" state it had just before the last computer caught fire. To do this, before running
the program, replace position 1 with the value 12 and replace position 2 with the value 2. What value is left at
position 0 after the program halts?
*/

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
enum Outcome {

    // Continue executing.
    //
    Continue,

    // Halt execution
    //
    Halt, 
}

trait OpCode {
    fn execute(&self, state: &mut ProgramState) -> Outcome;
}

/// Implementation of a halt op code.
#[derive(Default)]
struct HaltOpCode;
impl OpCode for HaltOpCode {
    fn execute(&self, _state: &mut ProgramState) -> Outcome {
        return Outcome::Halt;
    }
}

/// Implementation of an addition op code.
#[derive(Default)]
struct AddOpCode;
impl OpCode for AddOpCode {
    fn execute(&self, state: &mut ProgramState) -> Outcome {

        let arg_one = state.get_arg_one();
        let arg_two = state.get_arg_two();

        state.store_result(arg_one + arg_two);

        return Outcome::Continue;
    }
}

/// Implementation of a multiplication op code.
#[derive(Default)]
struct MultOpCode;
impl OpCode for MultOpCode {
    fn execute(&self, state: &mut ProgramState) -> Outcome {

        let arg_one = state.get_arg_one();
        let arg_two = state.get_arg_two();

        state.store_result(arg_one * arg_two);

        return Outcome::Continue;
    }
}

struct ProgramState<'a> {
    current_position: usize,
    state: &'a mut Vec<u32>,
}

impl<'a> ProgramState<'a> {

    const INSTRUCTION_INCREMENT: usize = 4;

    pub fn from_vec(instruction_stream: &'a mut Vec<u32>) -> Self {
        Self { 
            current_position: 0,
            state: instruction_stream
        }
    }

    pub fn get_arg_one(&self) -> u32 {
        let position = self.state[self.current_position + 1] as usize;
        self.state[position]
    }

    pub fn get_arg_two(&self) -> u32 {
        let position = self.state[self.current_position + 2] as usize;
        self.state[position]
    }

    pub fn store_result(&mut self, value: u32) {
        let position = self.state[self.current_position + 3] as usize;
        self.state[position] = value;
    }

    /// Restore the gravity assist program to the "1202 program alarm" state it
    /// had just before the last computer caught fire. To do this, before running
    /// the program, replace position 1 with the value 12 and replace position 2 with the value 2. 
    pub fn set_1202_alarm(&mut self) {
        self.state[1] = 12;
        self.state[2] = 02;
    }

    pub fn execute(&mut self) {

        let mut state = Outcome::Continue;

        while state == Outcome::Continue {
            let op_code = self.state[self.current_position];
            let op_action = self.opcode_factory(op_code);

            state = op_action.execute(self);

            // Jump forward the instruction increment.
            //
            self.current_position += ProgramState::INSTRUCTION_INCREMENT;
        }
    }

    fn opcode_factory(&self, code: u32) -> Box<dyn OpCode> {
        match code {
            01 => Box::new(AddOpCode::default()),
            02 => Box::new(MultOpCode::default()),
            99 => Box::new(HaltOpCode::default()),
            _ => panic!("Unknown opcode: {}", code),
        }
    }
}


#[cfg(test)]
fn assert_input_output(input: &mut Vec<u32>, expected: &mut Vec<u32>) {

    let mut program_state = ProgramState::from_vec(input);
    program_state.execute();

    // The input would have been modified.
    //
    assert_eq!(expected, input);
}

#[test]
fn test_program_execution_add() {
    // Execute a program that generates it's own terminates opcode.
    //
    let mut instructions = vec![1,6,5,4,0,98,1];
    let mut program_state = ProgramState::from_vec(&mut instructions);

    program_state.execute();
}

#[test]
fn test_program_execution_mult() {
    // Execute a program that generates it's own terminates opcode.
    //
    let mut instructions = vec![2,6,5,4,0,99,1];
    let mut program_state = ProgramState::from_vec(&mut instructions);

    program_state.execute();
}

#[test]
fn test_program_execution_samples() {
    {
        let mut input = vec![1,0,0,0,99];
        let mut expected = vec![2,0,0,0,99];
        assert_input_output(&mut input, &mut expected);
    }

    {
        let mut input = vec![2,3,0,3,99];
        let mut expected = vec![2,3,0,6,99];
        assert_input_output(&mut input, &mut expected);
    }
    
    {
        let mut input = vec![2,4,4,5,99,0];
        let mut expected = vec![2,4,4,5,99,9801];
        assert_input_output(&mut input, &mut expected);
    }

    {
        let mut input = vec![1,1,1,4,99,5,6,0,99];
        let mut expected = vec![30,1,1,4,2,5,6,0,99];
        assert_input_output(&mut input, &mut expected);
    }
}

fn process_file(filename: &str) -> Vec<u32> {

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut instructions = Vec::new();

    for line in reader.lines() {
        for value in line.unwrap().split(",") {
            instructions.push(value.parse::<u32>().unwrap());
        }
    }

    return instructions;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {

        let filename = &args[1];

        println!("Parsing file: {}", filename);

        let mut instructions = process_file(filename);
        let mut state = ProgramState::from_vec(&mut instructions);

        // Patch state as instructed above.
        state.set_1202_alarm();

        // Execute program
        state.execute();

        println!("End state: {}", instructions[0]);

    } else {
        println!("Missing input file");
    }
}
