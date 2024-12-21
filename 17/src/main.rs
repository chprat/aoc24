// Advent of Code 17.12.2024
// - read the debug output from a computer, that consists of
//   - the values of the registers a, b and c
//   - the program to run as comma separated list
//     - even list indices are the instruction
//     - odd list indices are the operand for the instruction before
//   - instruction details can be found on the AoC web site
// - part 1:
//   - get the values in the output register after running the program
// - part 2:
//   - find a value for the a register, so that after running the program the
//     output register contains the same values as the input

use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use z3::ast::{Ast, BV};

fn main() {
    let mut computer = read_data("input");
    computer.run();
    computer.print_result();

    let computer = read_data("input");
    let res = part2(computer.program_to_vec());
    println!("For a={} output and input of the computer are equal", res);
    assert_eq!(res, 164541017976509);
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum OperandType {
    Literal,
    Combo,
    Ignore,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum InstructionType {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
    Illegal,
}
impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match *self {
            InstructionType::Adv => "ADV",
            InstructionType::Bxl => "BXL",
            InstructionType::Bst => "BST",
            InstructionType::Jnz => "JNZ",
            InstructionType::Bxc => "BXC",
            InstructionType::Out => "OUT",
            InstructionType::Bdv => "BDV",
            InstructionType::Cdv => "CDV",
            _ => "ILL",
        };
        write!(f, "{}", text)
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    instruction_type: InstructionType,
    operand_type: OperandType,
    operand: i64,
}

impl Instruction {
    fn new(instruction: i64, operand: i64) -> Self {
        let (instruction_type, operand_type) = match instruction {
            0 => (InstructionType::Adv, OperandType::Combo),
            1 => (InstructionType::Bxl, OperandType::Literal),
            2 => (InstructionType::Bst, OperandType::Combo),
            3 => (InstructionType::Jnz, OperandType::Literal),
            4 => (InstructionType::Bxc, OperandType::Ignore),
            5 => (InstructionType::Out, OperandType::Combo),
            6 => (InstructionType::Bdv, OperandType::Combo),
            7 => (InstructionType::Cdv, OperandType::Combo),
            _ => (InstructionType::Illegal, OperandType::Ignore),
        };
        Self {
            instruction_type,
            operand_type,
            operand,
        }
    }
    fn get_operand(&self, computer: &Computer) -> i64 {
        if self.operand_type == OperandType::Combo {
            match self.operand {
                4 => computer.a,
                5 => computer.b,
                6 => computer.c,
                _ => self.operand,
            }
        } else {
            self.operand
        }
    }
    fn xdv(&self, computer: &Computer) -> i64 {
        let operand = self.get_operand(computer);
        let denominator = 2i64.pow(operand as u32);
        let frac: i64 = computer.a / denominator;
        frac
    }
    fn bxl(&self, computer: &Computer) -> i64 {
        let operand = self.get_operand(computer);
        computer.b ^ operand
    }
    fn bst(&self, computer: &Computer) -> i64 {
        let operand = self.get_operand(computer);
        operand % 8
    }
    fn jnz(&self, computer: &Computer) -> i64 {
        let operand = self.get_operand(computer);
        if computer.a != 0 {
            // instruction pointer is increased by 2 in computer and operates
            // on (opcode, operand), but we work with instructions and have to
            // half the jump
            operand / 2
        } else {
            (computer.ip + 1) as i64
        }
    }
    fn bxc(&self, computer: &Computer) -> i64 {
        computer.b ^ computer.c
    }
    fn out(&self, computer: &Computer) -> i64 {
        let operand = self.get_operand(computer);
        operand % 8
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<i64>> for Instruction {
    fn into(self) -> Vec<i64> {
        let instruction = match self.instruction_type {
            InstructionType::Adv => 0,
            InstructionType::Bxl => 1,
            InstructionType::Bst => 2,
            InstructionType::Jnz => 3,
            InstructionType::Bxc => 4,
            InstructionType::Out => 5,
            InstructionType::Bdv => 6,
            InstructionType::Cdv => 7,
            _ => 8,
        };
        vec![instruction, self.operand]
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.instruction_type, self.operand)
    }
}

#[derive(Clone, Debug)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
    out: Vec<i64>,
    program: Vec<Instruction>,
}

impl Computer {
    fn new(a: i64, b: i64, c: i64, program: Vec<Instruction>) -> Self {
        Self {
            a,
            b,
            c,
            ip: 0,
            out: Vec::new(),
            program,
        }
    }
    fn run(&mut self) {
        while let Some(instruction) = self.program.get(self.ip) {
            match instruction.instruction_type {
                InstructionType::Adv => self.a = instruction.xdv(self),
                InstructionType::Bxl => self.b = instruction.bxl(self),
                InstructionType::Bst => self.b = instruction.bst(self),
                InstructionType::Jnz => self.ip = instruction.jnz(self) as usize,
                InstructionType::Bxc => self.b = instruction.bxc(self),
                InstructionType::Out => self.out.push(instruction.out(self)),
                InstructionType::Bdv => self.b = instruction.xdv(self),
                InstructionType::Cdv => self.c = instruction.xdv(self),
                InstructionType::Illegal => unreachable!(),
            };
            if instruction.instruction_type != InstructionType::Jnz {
                self.ip += 1;
            }
        }
    }
    fn print_result(&self) {
        print!("Computer output: ");
        if !self.out.is_empty() {
            for i in &self.out[0..self.out.len() - 1] {
                print!("{},", i);
            }
            print!("{}", self.out.last().unwrap());
        } else {
            print!("<empty>");
        }
        println!();
    }
    fn program_to_vec(&self) -> Vec<i64> {
        let mut program: Vec<i64> = Vec::new();
        for instruction in &self.program {
            program.append(&mut <Instruction as std::convert::Into<Vec<i64>>>::into(
                *instruction,
            ));
        }
        program
    }
}
impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Register A: {}", self.a)?;
        writeln!(f, "Register B: {}", self.b)?;
        writeln!(f, "Register C: {}", self.c)?;
        write!(f, "Program: ")?;
        if !self.program.is_empty() {
            for i in &self.program[0..self.program.len() - 1] {
                write!(f, "{}, ", i)?;
            }
            write!(f, "{}", self.program.last().unwrap())?;
        }
        writeln!(f,)?;
        write!(f, "Out: ")?;
        if !self.out.is_empty() {
            for i in &self.out[0..self.out.len() - 1] {
                write!(f, "{},", i)?;
            }
            write!(f, "{}", self.out.last().unwrap())?;
        }
        Ok(())
    }
}

// the brute force loop approach for part 2 was oom killed after a few hours
// so we use z3, as brought up in the community
fn part2(program: Vec<i64>) -> i64 {
    let ctx = z3::Context::new(&z3::Config::new());
    let opt = z3::Optimize::new(&ctx);
    let s = BV::new_const(&ctx, "s", 64);
    #[allow(unused_assignments)]
    let (mut a, mut b, mut c) = (
        s.clone(),
        BV::from_i64(&ctx, 0, 64),
        BV::from_i64(&ctx, 0, 64),
    );
    for x in program {
        b = &a & &BV::from_i64(&ctx, 7, 64);
        b ^= &BV::from_i64(&ctx, 1, 64);
        c = a.bvlshr(&b);
        b ^= &BV::from_i64(&ctx, 5, 64);
        b ^= c;
        a = a.bvlshr(&BV::from_i64(&ctx, 3, 64));
        opt.assert(&(&b & &BV::from_i64(&ctx, 7, 64))._eq(&BV::from_i64(&ctx, x, 64)));
    }
    opt.assert(&(a._eq(&BV::from_i64(&ctx, 0, 64))));
    opt.minimize(&s);
    assert_eq!(opt.check(&[]), z3::SatResult::Sat);
    let res = opt
        .get_model()
        .unwrap()
        .eval(&s, true)
        .unwrap()
        .as_i64()
        .unwrap();
    res
}

// read computer debug information file
fn read_data(filename: &str) -> Computer {
    let (mut a, mut b, mut c) = (0, 0, 0);
    let mut program: Vec<Instruction> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            if line.contains("Register A:") {
                let lsps = line.split(" ").collect::<Vec<&str>>();
                assert_eq!(lsps.len(), 3);
                a = lsps
                    .last()
                    .expect("No last element found")
                    .parse()
                    .expect("Couldn't parse Register A");
            }
            if line.contains("Register B:") {
                let lsps = line.split(" ").collect::<Vec<&str>>();
                assert_eq!(lsps.len(), 3);
                b = lsps
                    .last()
                    .expect("No last element found")
                    .parse()
                    .expect("Couldn't parse Register B");
            }
            if line.contains("Register C:") {
                let lsps = line.split(" ").collect::<Vec<&str>>();
                assert_eq!(lsps.len(), 3);
                c = lsps
                    .last()
                    .expect("No last element found")
                    .parse()
                    .expect("Couldn't parse Register C");
            }
            if line.contains("Program:") {
                let lsps = line.split(" ").collect::<Vec<&str>>();
                assert_eq!(lsps.len(), 2);
                let program_splits = lsps[1].split(",").collect::<Vec<&str>>();
                let program_tuples = program_splits
                    .chunks(2)
                    .map(|p| (p[0], p[1]))
                    .collect::<Vec<(&str, &str)>>();
                program = program_tuples
                    .iter()
                    .map(|(opcode_s, operand_s)| {
                        let opcode = opcode_s.parse().expect("Couldn't parse opcode");
                        let operand = operand_s.parse().expect("Couldn't parse operand");
                        Instruction::new(opcode, operand)
                    })
                    .collect::<Vec<Instruction>>();
            }
        }
    }
    Computer::new(a, b, c, program)
}

// read a file and get the lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1() {
        let instructions = vec![Instruction::new(2, 6)];
        let mut computer = Computer {
            a: 0,
            b: 0,
            c: 9,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.b, 1);
    }
    #[test]
    fn step2() {
        let instructions = vec![
            Instruction::new(5, 0),
            Instruction::new(5, 1),
            Instruction::new(5, 4),
        ];
        let mut computer = Computer {
            a: 10,
            b: 0,
            c: 0,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.out, vec!(0, 1, 2));
    }
    #[test]
    fn step3() {
        let instructions = vec![
            Instruction::new(0, 1),
            Instruction::new(5, 4),
            Instruction::new(3, 0),
        ];
        let mut computer = Computer {
            a: 2024,
            b: 0,
            c: 0,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.out, vec!(4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0));
    }
    #[test]
    fn step4() {
        let instructions = vec![Instruction::new(1, 7)];
        let mut computer = Computer {
            a: 0,
            b: 29,
            c: 0,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.b, 26);
    }
    #[test]
    fn step5() {
        let instructions = vec![Instruction::new(4, 0)];
        let mut computer = Computer {
            a: 0,
            b: 2024,
            c: 43690,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.b, 44354);
    }
    #[test]
    fn adv() {
        let instructions = vec![Instruction::new(0, 2)];
        let mut computer = Computer {
            a: 8,
            b: 0,
            c: 0,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.a, 2);
        assert_eq!(computer.b, 0);
        assert_eq!(computer.c, 0);
        assert_eq!(computer.ip, 1);
        assert!(computer.out.is_empty());
    }
    #[test]
    fn bxl() {
        let instructions = vec![Instruction::new(1, 2)];
        let mut computer = Computer {
            a: 0,
            b: 8,
            c: 0,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.a, 0);
        assert_eq!(computer.b, 10);
        assert_eq!(computer.c, 0);
        assert_eq!(computer.ip, 1);
        assert!(computer.out.is_empty());
    }
    #[test]
    fn bst() {
        let instructions = vec![Instruction::new(2, 5)];
        let mut computer = Computer {
            a: 0,
            b: 9,
            c: 0,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.a, 0);
        assert_eq!(computer.b, 1);
        assert_eq!(computer.c, 0);
        assert_eq!(computer.ip, 1);
        assert!(computer.out.is_empty());
    }
    #[test]
    fn jnz() {
        let instructions = vec![Instruction::new(3, 4)];
        let mut computer = Computer {
            a: 0,
            b: 0,
            c: 0,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.a, 0);
        assert_eq!(computer.b, 0);
        assert_eq!(computer.c, 0);
        assert_eq!(computer.ip, 1);
        assert!(computer.out.is_empty());
        computer.a = 1;
        computer.ip = 0;
        computer.run();
        assert_eq!(computer.a, 1);
        assert_eq!(computer.b, 0);
        assert_eq!(computer.c, 0);
        assert_eq!(computer.ip, 2);
        assert!(computer.out.is_empty());
    }
    #[test]
    fn bxc() {
        let instructions = vec![Instruction::new(4, 0)];
        let mut computer = Computer {
            a: 0,
            b: 8,
            c: 2,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.a, 0);
        assert_eq!(computer.b, 10);
        assert_eq!(computer.c, 2);
        assert_eq!(computer.ip, 1);
        assert!(computer.out.is_empty());
    }
    #[test]
    fn out() {
        let instructions = vec![Instruction::new(5, 4)];
        let mut computer = Computer {
            a: 17,
            b: 0,
            c: 0,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.a, 17);
        assert_eq!(computer.b, 0);
        assert_eq!(computer.c, 0);
        assert_eq!(computer.ip, 1);
        assert_eq!(computer.out, vec!(1));
    }
    #[test]
    fn bdv() {
        let instructions = vec![Instruction::new(6, 2)];
        let mut computer = Computer {
            a: 8,
            b: 0,
            c: 0,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.a, 8);
        assert_eq!(computer.b, 2);
        assert_eq!(computer.c, 0);
        assert_eq!(computer.ip, 1);
        assert!(computer.out.is_empty());
    }
    #[test]
    fn cdv() {
        let instructions = vec![Instruction::new(7, 2)];
        let mut computer = Computer {
            a: 8,
            b: 0,
            c: 0,
            ip: 0,
            out: Vec::new(),
            program: instructions,
        };
        computer.run();
        assert_eq!(computer.a, 8);
        assert_eq!(computer.b, 0);
        assert_eq!(computer.c, 2);
        assert_eq!(computer.ip, 1);
        assert!(computer.out.is_empty());
    }
    #[test]
    fn part1_test() {
        let mut computer = read_data("input.test");
        computer.run();
        assert_eq!(computer.out, vec!(4, 6, 3, 5, 6, 3, 5, 2, 1, 0));
    }
    #[test]
    fn part1() {
        let mut computer = read_data("input");
        computer.run();
        assert_eq!(computer.out, vec!(7, 6, 1, 5, 3, 1, 4, 2, 6));
    }
}
