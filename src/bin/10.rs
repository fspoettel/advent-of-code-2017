use aoc::knot_hash::{get_int_list, hash_round, knot_hash};

pub fn part_one(input: &str) -> usize {
    let lens: Vec<usize> = input
        .lines()
        .next()
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
        .unwrap();

    let mut list = get_int_list();
    hash_round(&lens, &mut list, &mut 0, &mut 0);

    list.iter().take(2).product()
}

pub fn part_two(input: &str) -> String {
    knot_hash(input.lines().next().unwrap())
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 10), part_one, part_two)
}
