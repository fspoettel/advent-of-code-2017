use std::collections::HashMap;

struct Instance {
    register: HashMap<char, i64>,
    pos: usize,
}

impl Instance {
    fn new(id: i64) -> Self {
        let mut register = HashMap::new();
        register.insert('p', id);
        Instance { register, pos: 0 }
    }

    fn get(&self, reg: &char) -> i64 {
        *self.register.get(reg).unwrap_or(&0)
    }

    fn get_val(&self, val: &Value) -> i64 {
        match val {
            Value::Int(i) => *i,
            Value::FromReg(c) => self.get(c),
        }
    }

    fn set(&mut self, reg: &char, val: i64) {
        *self.register.entry(*reg).or_insert(0) = val;
    }
}

enum Instruction {
    Snd(char),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jump(char, Value),
}

enum Value {
    FromReg(char),
    Int(i64),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|l| {
            let mut parts = l.split(' ');
            let instruction = parts.next()?;

            let x = parts.next()?.chars().next()?;

            if instruction == "snd" {
                return Some(Instruction::Snd(x));
            } else if instruction == "rcv" {
                return Some(Instruction::Rcv(x));
            }

            let y_str = parts.next()?;
            let c = y_str.chars().next()?;
            let y = if y_str.len() > 1 || c.is_ascii_digit() {
                Value::Int(y_str.parse().ok()?)
            } else {
                Value::FromReg(c)
            };

            match instruction {
                "set" => Some(Instruction::Set(x, y)),
                "add" => Some(Instruction::Add(x, y)),
                "mul" => Some(Instruction::Mul(x, y)),
                "mod" => Some(Instruction::Mod(x, y)),
                "jgz" => Some(Instruction::Jump(x, y)),
                _ => None,
            }
        })
        .collect()
}

fn update_local_instance(instance: &mut Instance, instruction: &Instruction) {
    let mut increment = 1;

    match instruction {
        Instruction::Set(x, y) => instance.set(x, instance.get_val(y)),
        Instruction::Add(x, y) => instance.set(x, instance.get(x) + instance.get_val(y)),
        Instruction::Mul(x, y) => instance.set(x, instance.get(x) * instance.get_val(y)),
        Instruction::Mod(x, y) => instance.set(x, instance.get(x) % instance.get_val(y)),
        Instruction::Jump(x, y) => {
            if instance.get(x) > 0 {
                increment = instance.get_val(y);
            }
        }
        _ => {}
    };

    instance.pos = (instance.pos as i64 + increment) as usize;
}

pub fn part_one(input: &str) -> i64 {
    let instructions = parse(input);

    let mut instance = Instance::new(0);
    let mut last_played = 0;

    loop {
        match &instructions[instance.pos] {
            Instruction::Snd(reg) => {
                last_played = instance.get(reg);
                instance.pos += 1;
            }
            Instruction::Rcv(reg) => {
                if instance.get(reg) != 0 {
                    break;
                } else {
                    instance.pos += 1;
                }
            }
            i => {
                update_local_instance(&mut instance, i);
            }
        };
    }

    last_played
}

pub fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 18), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 18);
        assert_eq!(part_one(&input), 4);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 18);
        assert_eq!(part_two(&input), 0);
    }
}
