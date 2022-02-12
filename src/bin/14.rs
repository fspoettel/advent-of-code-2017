use aoc::grid::Point;
use aoc::knot_hash::knot_hash;
use std::collections::HashSet;

type Grid = HashSet<Point>;

const GRID_SIZE: usize = 128;

fn to_hex(c: char) -> String {
    format!(
        "{:04b}",
        u32::from_str_radix(c.encode_utf8(&mut [0; 1]), 16).unwrap()
    )
}

fn parse(input: &str) -> Grid {
    let mut grid = HashSet::new();
    let hash_key = input.lines().next().unwrap();

    for i in 0..GRID_SIZE {
        knot_hash(&format!("{}-{}", hash_key, i))
            .char_indices()
            .for_each(|(j, c)| {
                for (k, c) in to_hex(c).char_indices() {
                    if c == '1' {
                        grid.insert(Point(i, j * 4 + k));
                    }
                }
            })
    }

    grid
}

pub fn part_one(input: &str) -> usize {
    let grid = parse(input);
    grid.len()
}

fn remove_with_neighbors(grid: &mut Grid, point: Point) {
    grid.remove(&point);

    let neighbors: Vec<Point> = point
        .neighbors(GRID_SIZE - 1, GRID_SIZE - 1, false)
        .into_iter()
        .filter(|point| grid.contains(point))
        .collect();

    for point in neighbors {
        remove_with_neighbors(grid, point);
    }
}

pub fn part_two(input: &str) -> u32 {
    let mut grid = parse(input);

    let mut groups = 0;

    while !grid.is_empty() {
        groups += 1;
        let point = grid.iter().next().cloned().unwrap();
        remove_with_neighbors(&mut grid, point);
    }

    groups
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
        assert_eq!(part_two(&input), 1242);
    }
}
