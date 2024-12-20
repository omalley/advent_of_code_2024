use std::fmt::{Display, Formatter};
use itertools::Itertools;

type DataValue = u64;

#[derive(Clone,Copy,Debug)]
pub enum RegisterName {
  A,
  B,
  C
}

#[derive(Clone,Debug)]
pub struct State {
  registers: [DataValue; 3],
  pc: usize,
  output: Vec<u8>,
}

impl Display for State {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "A: {:o}, B: {:o}, C: {:o}, PC: {:o}, Outputs: {:?}",
           self.registers[RegisterName::A as usize], self.registers[RegisterName::B as usize],
           self.registers[RegisterName::C as usize], self.pc, self.output)
  }
}

#[derive(Clone,Copy,Debug)]
pub enum Operation {
  Adv(RegisterName),
  Xor(RegisterName),
  Jnz,
  Out,
  St(RegisterName),
}

#[derive(Clone,Copy,Debug)]
pub enum Operand {
  Literal(DataValue),
  Register(RegisterName),
}

impl Operand {
  fn literal_from_byte(b: u8) -> Operand {
    Operand::Literal((b % 8) as DataValue)
  }

  fn combo_from_byte(b: u8) -> Result<Operand, String> {
    match b % 8 {
      0..4 => Ok(Operand::Literal((b % 8) as DataValue)),
      4 => Ok(Operand::Register(RegisterName::A)),
      5 => Ok(Operand::Register(RegisterName::B)),
      6 => Ok(Operand::Register(RegisterName::C)),
      _ => Err(format!("Bad combo number {}", b)),
    }
  }

  fn evaluate(&self, state: &State) -> DataValue {
    match self {
      Operand::Literal(lit) => *lit,
      Operand::Register(reg) => state.registers[*reg as usize],
    }
  }
}

#[derive(Clone,Copy,Debug)]
pub struct Instruction {
  op: Operation,
  operand: Operand,
}

impl Instruction {
  fn from_bytes(bytes: &[u8]) -> Result<Instruction, String> {
    if bytes.len() != 2 {
      return Err(format!("Bad instruction length {}", bytes.len()));
    }
    match bytes[0] % 8 {
      0 => Ok(Instruction{op: Operation::Adv(RegisterName::A),
        operand: Operand::combo_from_byte(bytes[1])?}),
      1 => Ok(Instruction{op: Operation::Xor(RegisterName::B),
        operand: Operand::literal_from_byte(bytes[1])}),
      2 => Ok(Instruction{op: Operation::St(RegisterName::B),
        operand: Operand::combo_from_byte(bytes[1])?}),
      3 => Ok(Instruction{op: Operation::Jnz,
        operand: Operand::literal_from_byte(bytes[1])}),
      4 => Ok(Instruction{op: Operation::Xor(RegisterName::B),
        operand: Operand::Register(RegisterName::C)}),
      5 => Ok(Instruction{op: Operation::Out,
        operand: Operand::combo_from_byte(bytes[1])?}),
      6 => Ok(Instruction{op: Operation::Adv(RegisterName::B),
        operand: Operand::combo_from_byte(bytes[1])?}),
      7 => Ok(Instruction{op: Operation::Adv(RegisterName::C),
        operand: Operand::combo_from_byte(bytes[1])?}),
      _ => Err(format!("Bad instruction number {}", bytes[0])),
    }
  }

  fn exuecute(&self, state: &mut State) {
    state.pc += 1;
    match self.op {
      Operation::Adv(reg) => {
        state.registers[reg as usize] = state.registers[RegisterName::A as usize] >>
            self.operand.evaluate(state);
      }
      Operation::Xor(reg) => {
        state.registers[reg as usize] = state.registers[RegisterName::B as usize] ^
            self.operand.evaluate(state);
      }
      Operation::Jnz => {
        if state.registers[RegisterName::A as usize] != 0 {
          state.pc = self.operand.evaluate(state) as usize;
        }
      }
      Operation::Out => {
        state.output.push((self.operand.evaluate(state) % 8) as u8)
      }
      Operation::St(reg) => {
        state.registers[reg as usize] = self.operand.evaluate(state) % 8;
      }
    }
  }
}

type Program = Vec<Instruction>;

fn read_register(s: &str) -> Result<DataValue, String> {
  let (_, value) = s.split_once(':').ok_or("Can't read register value {s}")?;
  value.trim().parse().map_err(|_| format!("Can't parse register value {value}"))
}

pub fn generator(input: &str) -> (State, Program, Vec<u8>) {
  let (registers, program) = input.split_once("\n\n")
      .expect("Can't find program");
  let values = registers.lines()
      .map(read_register)
      .collect::<Result<Vec<DataValue>, String>>()
      .expect("Can't parse values")
      .try_into().unwrap();
  let state = State{registers: values, pc: 0, output: Vec::new()};
  let (_, program) = program.trim().split_once(": ").expect("Can't find program");
  let bytes: Vec<u8> = program.split(',').map(|s| s.parse::<u8>()
      .map_err(|_| format!("int parse error '{s}'"))).try_collect()
      .expect("Can't parse program");
  let mut program = Vec::new();
  for cmd_bytes in &bytes.iter().chunks(2) {
    program.push(Instruction::from_bytes(&cmd_bytes.copied().collect::<Vec<u8>>())
        .expect("Can't parse instruction"));
  }
  (state, program, bytes)
}

pub fn part1((state, program, _): &(State, Program, Vec<u8>)) -> String {
  let mut state = state.clone();
  while state.pc < program.len() {
    program[state.pc].exuecute(&mut state);
  }
  state.output.iter().join(",")
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum TestResult {
  Fail, Partial, Match(u64),
}

fn run_test(orig_state: &State, program: &Program, a: DataValue, required: usize,
            goal: &[u8]) -> TestResult {
  let mut state = orig_state.clone();
  state.registers[RegisterName::A as usize] = a;
  while state.pc < program.len() {
    program[state.pc].exuecute(&mut state);
  }
  if state.output.len() < required || state.output.len() > goal.len() {
    return TestResult::Fail;
  }
  for (i, g) in goal.iter().enumerate().filter(|(i, _)| *i < required) {
    if state.output[i] != *g {
      return TestResult::Fail;
    }
  }
  if required == goal.len() {
    TestResult::Match(a)
  } else {
    TestResult::Partial
  }
}

fn next_digit(orig_state: &State, program: &Program, base: DataValue, required: usize,
              goal: &[u8]) -> TestResult {
  let mut results = Vec::new();
  for digit in 0..8 {
    let a = base + (digit << (3 * (2 + required)));
    match run_test(orig_state, program, a, required, goal) {
      TestResult::Fail => { continue },
      TestResult::Partial => { },
      TestResult::Match(a) => { results.push(a); continue },
    }
    if required == goal.len() {
      return TestResult::Fail;
    }
    if let TestResult::Match(n) = next_digit(orig_state, program, a, required + 1, goal) {
      results.push(n)
    }
  }
  if results.is_empty() {
    TestResult::Fail
  } else {
    TestResult::Match(*results.iter().min().unwrap())
  }
}

pub fn part2((orig_state, program, bytes): &(State, Program, Vec<u8>)) -> DataValue {
  let mut results = Vec::new();
  for a in 0..(8u64.pow(4)) {
    match run_test(orig_state, program, a, 1, bytes) {
      TestResult::Fail => { continue },
      TestResult::Partial => { },
      TestResult::Match(a) => {  results.push(a); continue },
    }
    if let TestResult::Match(n) =  next_digit(orig_state, program, a, 2, bytes) {
      results.push(n)
    }
  }
  *results.iter().min().expect("No results")
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(&data));
  }

  const PART2_INPUT: &str =
  "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

  #[test]
  fn test_part2() {
    let data = generator(PART2_INPUT);
    assert_eq!(117440, part2(&data));
  }
}
