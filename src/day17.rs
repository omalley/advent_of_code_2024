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
            self.operand.evaluate(state).min(63);
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

pub fn generator(input: &str) -> (State, Program) {
  let (registers, program) = input.split_once("\n\n")
      .expect("Can't find program");
  let values = registers.lines()
      .map(|s| read_register(s))
      .collect::<Result<Vec<DataValue>, String>>()
      .expect("Can't parse values")
      .try_into().unwrap();
  let state = State{registers: values, pc: 0, output: Vec::new()};
  let (_, program) = program.trim().split_once(": ").expect("Can't find program");
  let bytes: Vec<u8> = program.split(',').map(|s| s.parse::<u8>()
      .map_err(|_| format!("int parse error '{s}'"))).try_collect()
      .expect("Can't parse program");
  let mut program = Vec::new();
  for bytes in &bytes.into_iter().chunks(2) {
    program.push(Instruction::from_bytes(&bytes.collect::<Vec<u8>>())
        .expect("Can't parse instruction"));
  }
  (state, program)
}

pub fn part1((state, program): &(State, Program)) -> String {
  let mut state = state.clone();
  while state.pc < program.len() {
    program[state.pc].exuecute(&mut state);
  }
  state.output.iter().join(",")
}

pub fn part2((_state, program): &(State, Program)) -> String {
  for (pc, stmt) in program.iter().enumerate() {
    println!("{pc}: {stmt:?}");
  }
  String::new()
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

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!("", part2(&data));
  }
}
