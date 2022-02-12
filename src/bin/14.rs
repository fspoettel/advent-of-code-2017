use aoc::knot_hash::knot_hash;

type Grid = [[bool; 128]; 128];

fn to_hex(c: char) -> String {
    format!(
        "{:04b}",
        u32::from_str_radix(c.encode_utf8(&mut [0; 1]), 16).unwrap()
    )
}

fn parse(input: &str) -> Grid {
    let mut grid = [[false; 128]; 128];
    let hash_key = input.lines().next().unwrap();

    for (i, row) in grid.iter_mut().enumerate() {
        knot_hash(&format!("{}-{}", hash_key, i))
            .char_indices()
            .for_each(|(j, c)| {
                for (k, c) in to_hex(c).char_indices() {
                    if c == '1' {
                        row[j * 4 + k] = true;
                    }
                }
            })
    }

    grid
}

pub fn part_one(input: &str) -> u32 {
    let grid = parse(&input);
    grid.iter()
        .flat_map(|r| {
            r.iter()
                .filter_map(|&x| if x { Some(x as u32) } else { None })
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let grid = parse(&input);
    0
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 14), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 14);
        assert_eq!(part_one(&input), 8108);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 14);
        assert_eq!(part_two(&input), 0);
    }
}
