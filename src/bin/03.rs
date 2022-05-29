use hashbrown::{HashMap, HashSet};

fn parse(input: &str) -> u32 {
    input.lines().next().unwrap().parse::<u32>().unwrap()
}

type Position = (i32, i32);

#[derive(Clone, Copy)]
enum Orientation {
    North,
    East,
    South,
    West,
}

type Spiral = HashSet<Position>;
type Scores = HashMap<(i32, i32), u32>;

fn go(pos: &Position, orientation: &Orientation) -> Position {
    match orientation {
        Orientation::North => (pos.0, pos.1 - 1),
        Orientation::East => (pos.0 + 1, pos.1),
        Orientation::South => (pos.0, pos.1 + 1),
        Orientation::West => (pos.0 - 1, pos.1),
    }
}

fn rotate(spiral: &Spiral, pos: &Position, orientation: &Orientation) -> Orientation {
    let next_rotation = match orientation {
        Orientation::North => Orientation::West,
        Orientation::East => Orientation::North,
        Orientation::South => Orientation::East,
        Orientation::West => Orientation::South,
    };

    if spiral.contains(&go(pos, &next_rotation)) {
        *orientation
    } else {
        next_rotation
    }
}

fn sum_neighbors(scores: &Scores, pos: &Position) -> u32 {
    scores.get(&(pos.0 - 1, pos.1 - 1)).unwrap_or(&0)
        + scores.get(&(pos.0, pos.1 - 1)).unwrap_or(&0)
        + scores.get(&(pos.0 + 1, pos.1 - 1)).unwrap_or(&0)
        + scores.get(&(pos.0 + 1, pos.1)).unwrap_or(&0)
        + scores.get(&(pos.0 + 1, pos.1 + 1)).unwrap_or(&0)
        + scores.get(&(pos.0, pos.1 + 1)).unwrap_or(&0)
        + scores.get(&(pos.0 - 1, pos.1 + 1)).unwrap_or(&0)
        + scores.get(&(pos.0 - 1, pos.1)).unwrap_or(&0)
}

pub fn part_one(input: &str) -> i32 {
    let mut spiral: Spiral = HashSet::new();
    let mut current_position = (0, 0);
    let mut current_orientation = Orientation::East;
    spiral.insert(current_position);

    for _ in 1..parse(input) {
        current_position = go(&current_position, &current_orientation);
        current_orientation = rotate(&spiral, &current_position, &current_orientation);
        spiral.insert(current_position);
    }

    current_position.0.abs() + current_position.1.abs()
}

pub fn part_two(input: &str) -> u32 {
    let target_score = parse(input);

    let mut spiral: Spiral = HashSet::new();
    let mut scores = HashMap::new();
    let mut current_position = (0, 0);
    let mut current_orientation = Orientation::East;
    spiral.insert(current_position);
    scores.insert(current_position, 1);

    loop {
        current_position = go(&current_position, &current_orientation);
        current_orientation = rotate(&spiral, &current_position, &current_orientation);
        let score = sum_neighbors(&scores, &current_position);
        scores.insert(current_position, score);
        spiral.insert(current_position);

        if score > target_score {
            break;
        }
    }

    *scores.get(&current_position).unwrap()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 3), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1"), 0);
        assert_eq!(part_one("12"), 3);
        assert_eq!(part_one("23"), 2);
        assert_eq!(part_one("1024"), 31);
    }
}
