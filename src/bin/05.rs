fn parse(input: &str) -> Vec<i32> {
    input.lines().filter_map(|l| l.parse().ok()).collect()
}

fn resolve_instructions(instructions: &mut Vec<i32>, next_instruction: fn(i32) -> i32) -> u32 {
    let len = instructions.len() as i32;
    let mut step = 1;
    let mut offset: i32 = 0;

    loop {
        let instruction = &mut instructions[offset as usize];
        let next_offset = offset + *instruction;

        if next_offset < 0 || next_offset >= len {
            break;
        }

        step += 1;
        offset = next_offset;
        *instruction = next_instruction(*instruction);
    }

    step
}

pub fn part_one(input: &str) -> u32 {
    resolve_instructions(&mut parse(input), |x| x + 1)
}

pub fn part_two(input: &str) -> u32 {
    resolve_instructions(&mut parse(input), |x| if x >= 3 { x - 1 } else { x + 1 })
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 5), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_one(&input), 5);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_two(&input), 10);
    }
}
