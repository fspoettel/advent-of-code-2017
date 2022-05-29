use hashbrown::HashMap;

type Infections = HashMap<Point, State>;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point(i32, i32);

#[derive(PartialEq)]
enum State {
    Clean,
    Flagged,
    Infected,
    Weakened,
}

enum Direction {
    Down,
    Left,
    Right,
    Up,
}

fn parse(input: &str) -> (Infections, Point) {
    let mut infections: Infections = HashMap::new();
    let mut size_x = 0;
    let mut size_y = 0;

    input.lines().enumerate().for_each(|(y, l)| {
        size_y = y as i32;
        l.chars().enumerate().for_each(|(x, c)| {
            let p = Point(x as i32, y as i32);

            if y == 0 {
                size_x = x as i32;
            }

            if c == '#' {
                infections.insert(p, State::Infected);
            }
        });
    });

    (infections, Point(size_x / 2, size_y / 2))
}

fn go(dir: &Direction, pos: &Point) -> Point {
    match dir {
        Direction::Down => Point(pos.0, pos.1 + 1),
        Direction::Left => Point(pos.0 - 1, pos.1),
        Direction::Right => Point(pos.0 + 1, pos.1),
        Direction::Up => Point(pos.0, pos.1 - 1),
    }
}

fn turn(dir: &Direction, left: bool) -> Direction {
    match dir {
        Direction::Down => {
            if left {
                Direction::Right
            } else {
                Direction::Left
            }
        }
        Direction::Left => {
            if left {
                Direction::Down
            } else {
                Direction::Up
            }
        }
        Direction::Right => {
            if left {
                Direction::Up
            } else {
                Direction::Down
            }
        }
        Direction::Up => {
            if left {
                Direction::Left
            } else {
                Direction::Right
            }
        }
    }
}

pub fn part_one(input: &str) -> u32 {
    let (mut infections, mut pos) = parse(input);
    let mut dir = Direction::Up;
    let mut cycles_with_infections = 0;

    for _ in 0..10000 {
        let state = infections.entry(pos).or_insert(State::Clean);
        let is_clean = *state == State::Clean;
        dir = turn(&dir, is_clean);

        if is_clean {
            *state = State::Infected;
            cycles_with_infections += 1;
        } else {
            *state = State::Clean;
        }

        pos = go(&dir, &pos)
    }

    cycles_with_infections
}

pub fn part_two(input: &str) -> u32 {
    let (mut infections, mut pos) = parse(input);
    let mut dir = Direction::Up;
    let mut cycles_with_infections = 0;

    for _ in 0..10000000 {
        let state = infections.entry(pos).or_insert(State::Clean);

        dir = match *state {
            State::Clean => turn(&dir, true),
            State::Flagged => turn(&turn(&dir, true), true),
            State::Infected => turn(&dir, false),
            State::Weakened => dir,
        };

        *state = match *state {
            State::Clean => State::Weakened,
            State::Flagged => State::Clean,
            State::Infected => State::Flagged,
            State::Weakened => State::Infected,
        };

        if *state == State::Infected {
            cycles_with_infections += 1;
        }

        pos = go(&dir, &pos);
    }

    cycles_with_infections
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 22), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 22);
        assert_eq!(part_one(&input), 5587);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 22);
        assert_eq!(part_two(&input), 2511944);
    }
}
