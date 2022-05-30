use hashbrown::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point(i32, i32);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse(input: &str) -> u32 {
    input
        .lines()
        .next()
        .map(|s| s.parse::<u32>().unwrap())
        .unwrap()
}

type Spiral = HashSet<Point>;
type Scores = HashMap<Point, u32>;

fn go(pos: &Point, dir: &Direction) -> Point {
    match dir {
        Direction::Up => Point(pos.0, pos.1 - 1),
        Direction::Right => Point(pos.0 + 1, pos.1),
        Direction::Down => Point(pos.0, pos.1 + 1),
        Direction::Left => Point(pos.0 - 1, pos.1),
    }
}

fn turn(spiral: &Spiral, pos: &Point, dir: &Direction) -> Direction {
    let next_dir = match dir {
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
    };

    if spiral.contains(&go(pos, &next_dir)) {
        *dir
    } else {
        next_dir
    }
}

fn sum_neighbors(scores: &Scores, pos: &Point) -> u32 {
    scores.get(&Point(pos.0 - 1, pos.1 - 1)).unwrap_or(&0)
        + scores.get(&Point(pos.0, pos.1 - 1)).unwrap_or(&0)
        + scores.get(&Point(pos.0 + 1, pos.1 - 1)).unwrap_or(&0)
        + scores.get(&Point(pos.0 + 1, pos.1)).unwrap_or(&0)
        + scores.get(&Point(pos.0 + 1, pos.1 + 1)).unwrap_or(&0)
        + scores.get(&Point(pos.0, pos.1 + 1)).unwrap_or(&0)
        + scores.get(&Point(pos.0 - 1, pos.1 + 1)).unwrap_or(&0)
        + scores.get(&Point(pos.0 - 1, pos.1)).unwrap_or(&0)
}

pub fn part_one(input: &str) -> i32 {
    let mut spiral: Spiral = HashSet::new();
    let mut pos = Point(0, 0);
    let mut dir = Direction::Right;

    spiral.insert(pos);

    for _ in 1..parse(input) {
        pos = go(&pos, &dir);
        dir = turn(&spiral, &pos, &dir);
        spiral.insert(pos);
    }

    pos.0.abs() + pos.1.abs()
}

pub fn part_two(input: &str) -> u32 {
    let target_score = parse(input);

    let mut spiral: Spiral = HashSet::new();
    let mut scores = HashMap::new();
    let mut pos = Point(0, 0);
    let mut dir = Direction::Right;

    spiral.insert(pos);
    scores.insert(pos, 1);

    loop {
        pos = go(&pos, &dir);
        dir = turn(&spiral, &pos, &dir);

        let score = sum_neighbors(&scores, &pos);
        scores.insert(pos, score);
        spiral.insert(pos);

        if score > target_score {
            break;
        }
    }

    *scores.get(&pos).unwrap()
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
