use std::cmp::max;

struct State {
    q: i32,
    r: i32,
    s: i32,
}

fn moves(input: &str) -> impl Iterator<Item = &str> {
    input.lines().next().unwrap().split(',')
}

fn distance(state: &State) -> i32 {
    let State { q, r, s } = state;
    ((0 - q).abs() + (0 - r).abs() + (0 - s).abs()) / 2
}

// see https://www.redblobgames.com/grids/hexagons/
fn hex_move(state: &State, dir: &str) -> State {
    let State { q, r, s } = state;

    match dir {
        "n" => State {
            s: s + 1,
            r: r - 1,
            ..*state
        },
        "ne" => State {
            q: q + 1,
            r: r - 1,
            ..*state
        },
        "nw" => State {
            q: q - 1,
            s: s + 1,
            ..*state
        },
        "s" => State {
            s: s - 1,
            r: r + 1,
            ..*state
        },
        "sw" => State {
            q: q - 1,
            r: r + 1,
            ..*state
        },
        "se" => State {
            q: q + 1,
            s: s - 1,
            ..*state
        },
        s => panic!("invalid input: {}", s),
    }
}

pub fn part_one(input: &str) -> i32 {
    let mut state = State { q: 0, r: 0, s: 0 };

    moves(input).for_each(|dir| {
        state = hex_move(&state, dir);
    });

    distance(&state)
}

pub fn part_two(input: &str) -> i32 {
    let mut state = State { q: 0, r: 0, s: 0 };
    let mut ceil = 0;

    moves(input).for_each(|dir| {
        state = hex_move(&state, dir);
        ceil = max(ceil, distance(&state));
    });

    ceil
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 11), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("ne,ne,ne"), 3);
        assert_eq!(part_one("ne,ne,sw,sw"), 0);
        assert_eq!(part_one("ne,ne,s,s"), 2);
        assert_eq!(part_one("se,sw,se,sw,sw"), 3);
    }
}
