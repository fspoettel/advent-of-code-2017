enum Direction {
    North,
    South,
    West,
    East,
}

type Point = (usize, usize);

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(l.chars().collect())
            }
        })
        .collect()
}

fn entry_point(grid: &[Vec<char>]) -> Point {
    grid[0]
        .iter()
        .enumerate()
        .find_map(|(x, &val)| if val == '|' { Some((x, 0)) } else { None })
        .unwrap()
}

// bound checks can be skipped as the grid is padded by one line of empty space.
fn next_point(grid: &[Vec<char>], (x, y): Point, dir: Direction) -> Option<(Point, Direction)> {
    let mut x = x;
    let mut y = y;
    let mut dir = dir;

    if grid[y][x] != '+' {
        match dir {
            Direction::North => y -= 1,
            Direction::South => y += 1,
            Direction::East => x += 1,
            Direction::West => x -= 1,
        };
    } else {
        match dir {
            Direction::North | Direction::South => {
                if grid[y][x - 1] != ' ' {
                    x -= 1;
                    dir = Direction::West;
                } else {
                    x += 1;
                    dir = Direction::East;
                }
            }
            _ => {
                if grid[y - 1][x] != ' ' {
                    y -= 1;
                    dir = Direction::North;
                } else {
                    y += 1;
                    dir = Direction::South;
                }
            }
        }
    }

    if grid[y][x] != ' ' {
        Some(((x, y), dir))
    } else {
        None
    }
}

fn walk(grid: &[Vec<char>]) -> (String, u32) {
    let mut pos = entry_point(grid);
    let mut dir = Direction::South;
    let mut letter_order = String::new();
    let mut step_count = 1;

    while let Some(((x, y), next_dir)) = next_point(grid, pos, dir) {
        pos = (x, y);
        dir = next_dir;
        step_count += 1;
        let val = grid[y][x];
        if val.is_ascii_uppercase() {
            letter_order.push(val);
        }
    }

    (letter_order, step_count)
}

pub fn part_one(input: &str) -> String {
    walk(&parse(input)).0
}

pub fn part_two(input: &str) -> u32 {
    walk(&parse(input)).1
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 19), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 19);
        assert_eq!(part_one(&input), "ABCDEF".to_string());
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 19);
        assert_eq!(part_two(&input), 38);
    }
}
