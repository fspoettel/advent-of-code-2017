use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

struct Tree<'a> {
    nodes: HashMap<&'a str, u32>,
    edges: HashMap<&'a str, Vec<&'a str>>,
}

fn parse(input: &str) -> Tree {
    let tree = Tree {
        nodes: HashMap::new(),
        edges: HashMap::new(),
    };

    input.lines().fold(tree, |mut acc, l| {
        let (name, weight_str) = l.split(" (").collect_tuple().unwrap();
        let (weight, children_str) = weight_str.split(')').collect_tuple().unwrap();

        if !children_str.is_empty() {
            acc.edges.insert(
                name,
                children_str
                    .split("-> ")
                    .last()
                    .unwrap()
                    .split(", ")
                    .collect(),
            );
        }

        acc.nodes.insert(name, weight.parse().unwrap());
        acc
    })
}

fn find_root_node<'a>(tree: &'a Tree) -> &'a str {
    let children: HashSet<&str> = tree
        .edges
        .values()
        .fold(HashSet::new(), |mut acc, children| {
            acc.extend(children);
            acc
        });

    *tree.nodes.keys().find(|p| !children.contains(*p)).unwrap()
}

pub fn part_one(input: &str) -> String {
    find_root_node(&parse(input)).to_string()
}

pub fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 7), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_one(&input), "tknk".to_string());
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_two(&input), 0);
    }
}
