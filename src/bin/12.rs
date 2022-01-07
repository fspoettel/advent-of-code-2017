use std::collections::HashSet;

type AdjacencyList = Vec<usize>;

fn parse(input: &str) -> Vec<AdjacencyList> {
    input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(
                    l.split(" <-> ")
                        .last()
                        .unwrap()
                        .split(", ")
                        .map(|x| x.parse().unwrap())
                        .collect(),
                )
            }
        })
        .collect()
}

fn traverse(i: usize, graph: &[AdjacencyList], seen: &mut HashSet<usize>) {
    let neighbors = &graph[i];
    seen.insert(i);

    for x in neighbors {
        if !seen.contains(x) {
            traverse(*x, graph, seen);
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let graph = parse(input);
    let mut seen = HashSet::new();
    traverse(0, &graph, &mut seen);
    seen.len()
}

pub fn part_two(input: &str) -> u32 {
    let graph = parse(input);
    let mut seen = HashSet::new();
    let mut groups = 0;

    while seen.len() < graph.len() {
        groups += 1;

        let next_unseen = graph
            .iter()
            .enumerate()
            .find(|(i, _)| !seen.contains(i))
            .map(|(i, _)| i)
            .unwrap();

        traverse(next_unseen, &graph, &mut seen);
    }

    groups
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 12), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 12);
        assert_eq!(part_one(&input), 6);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 12);
        assert_eq!(part_two(&input), 2);
    }
}
