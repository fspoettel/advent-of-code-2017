use std::cmp::max;

type Component = (u32, u32);
type Bridge = Vec<Component>;

fn parse(input: &str) -> Vec<Component> {
    input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                let mut it = l.split('/').map(|c| c.parse().unwrap());
                Some((it.next().unwrap(), it.next().unwrap()))
            }
        })
        .collect()
}

fn walk_bridges(
    bridge: Bridge,
    port: u32,
    components: &[Component],
    all_bridges: &mut Vec<Bridge>,
) {
    components
        .iter()
        .filter(|c| (c.0 == port || c.1 == port) && !bridge.contains(c))
        .for_each(|c| {
            let mut new_bridge = bridge.clone();
            new_bridge.push(*c);
            all_bridges.push(new_bridge.clone());
            let target_port = if c.0 != port { c.0 } else { c.1 };
            walk_bridges(new_bridge, target_port, components, all_bridges)
        })
}

fn sum_bridge(bridge: &[Component]) -> u32 {
    bridge.iter().map(|(a, b)| a + b).sum()
}

fn find_best_bridge(bridges: &[Bridge]) -> u32 {
    bridges.iter().map(|b| sum_bridge(b)).max().unwrap()
}

fn find_best_long_bridge(bridges: &[Bridge]) -> u32 {
    let mut max_len = 0;
    let mut max_sum = 0;

    bridges.iter().for_each(|bridge| {
        let len = bridge.len();
        if len > max_len {
            max_len = len;
            max_sum = sum_bridge(bridge);
        } else if len == max_len {
            max_sum = max(sum_bridge(bridge), max_sum);
        }
    });

    max_sum
}

pub fn part_one(input: &str) -> u32 {
    let components = parse(input);
    let mut bridges = Vec::new();
    walk_bridges(vec![], 0, &components, &mut bridges);
    find_best_bridge(&bridges)
}

pub fn part_two(input: &str) -> u32 {
    let components = parse(input);
    let mut bridges = Vec::new();
    walk_bridges(vec![], 0, &components, &mut bridges);
    find_best_long_bridge(&bridges)
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 24), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 24);
        assert_eq!(part_one(&input), 31);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 24);
        assert_eq!(part_two(&input), 19);
    }
}
