use std::cmp::max;
use std::collections::HashMap;

enum Op {
    Decrement(i32),
    Increment(i32),
}

enum Cmp {
    Eq(i32),
    Gt(i32),
    Gte(i32),
    Lt(i32),
    Lte(i32),
    Neq(i32),
}

struct Cond<'a> {
    cmp: Cmp,
    target: &'a str,
}

struct Instruction<'a> {
    cond: Cond<'a>,
    op: Op,
    target: &'a str,
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }

            let mut parts = l.split(' ');

            let target = parts.next()?;
            let op = parts.next()?;
            let amount = parts.next()?.parse().unwrap();
            let cond_target = parts.nth(1)?;
            let cmp = parts.next()?;
            let cmp_amount = parts.next()?.parse().unwrap();

            let op = match op {
                "inc" => Op::Increment(amount),
                "dec" => Op::Decrement(amount),
                v => panic!("bad syntax: {}", v),
            };

            let cmp = match cmp {
                ">" => Cmp::Gt(cmp_amount),
                "<" => Cmp::Lt(cmp_amount),
                ">=" => Cmp::Gte(cmp_amount),
                "<=" => Cmp::Lte(cmp_amount),
                "==" => Cmp::Eq(cmp_amount),
                "!=" => Cmp::Neq(cmp_amount),
                v => panic!("bad syntax: {}", v),
            };

            Some(Instruction {
                target,
                op,
                cond: Cond {
                    cmp,
                    target: cond_target,
                },
            })
        })
        .collect()
}

fn apply(instructions: &[Instruction]) -> (i32, i32) {
    let mut memory = HashMap::new();
    let mut ceil = 0;

    for inst in instructions {
        let x = *memory.entry(inst.cond.target).or_default();

        if match inst.cond.cmp {
            Cmp::Gt(y) => x > y,
            Cmp::Gte(y) => x >= y,
            Cmp::Lt(y) => x < y,
            Cmp::Lte(y) => x <= y,
            Cmp::Eq(y) => x == y,
            Cmp::Neq(y) => x != y,
        } {
            let entry = memory.entry(inst.target).or_default();
            let amount = match inst.op {
                Op::Decrement(y) => -y,
                Op::Increment(y) => y,
            };

            *entry += amount;
            ceil = max(ceil, *entry);
        }
    }

    (*memory.values().max().unwrap(), ceil)
}

pub fn part_one(input: &str) -> i32 {
    let (max, _) = apply(&parse(input));
    max
}

pub fn part_two(input: &str) -> i32 {
    let (_, ceil) = apply(&parse(input));
    ceil
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 8), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_one(&input), 1);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_two(&input), 10);
    }
}
