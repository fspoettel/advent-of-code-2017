struct Scores {
    garbage: usize,
    groups: usize,
}

fn parse(input: &str) -> Vec<char> {
    input.lines().next().unwrap().chars().collect()
}

fn parse_stream(chars: &[char]) -> Scores {
    let mut scores = Scores {
        garbage: 0,
        groups: 0,
    };

    let mut current_stack: Vec<char> = Vec::new();
    let mut negate_flag = false;

    for &c in chars {
        if !matches!(current_stack.last(), Some('<')) {
            match c {
                '{' => {
                    current_stack.push(c);
                }
                '<' => {
                    current_stack.push(c);
                }
                '}' => {
                    scores.groups += current_stack.len();
                    current_stack.pop();
                }
                _ => {}
            };
        } else if negate_flag {
            negate_flag = false;
        } else {
            match c {
                '!' => negate_flag = true,
                '>' => {
                    current_stack.pop();
                }
                _ => {
                    scores.garbage += 1;
                }
            };
        }
    }

    scores
}

pub fn part_one(input: &str) -> usize {
    parse_stream(&parse(input)).groups
}

pub fn part_two(input: &str) -> usize {
    parse_stream(&parse(input)).garbage
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 9), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("{}"), 1);
        assert_eq!(part_one("{{{}}}"), 6);
        assert_eq!(part_one("{{},{}}"), 5);
        assert_eq!(part_one("{{{},{},{{}}}}"), 16);
        assert_eq!(part_one("{<{},{},{{}}>}"), 1);
        assert_eq!(part_one("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(part_one("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(part_one("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(part_one("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("<>"), 0);
        assert_eq!(part_two("<random characters>"), 17);
        assert_eq!(part_two("<<<<>"), 3);
        assert_eq!(part_two("<{!>}>"), 2);
        assert_eq!(part_two("<!!>"), 0);
        assert_eq!(part_two("<!!!>>"), 0);
        assert_eq!(part_two("<{o\"i!a,<{i<a>"), 10);
    }
}
