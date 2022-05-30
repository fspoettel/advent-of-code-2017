use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct Node<'a> {
    name: &'a str,
    weight: u32,
    weight_children: u32,
}

impl Node<'_> {
    fn total_weight(&self) -> u32 {
        self.weight + self.weight_children
    }
}

#[derive(Clone, Debug)]
struct Tree<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
    // optimization: we need to traverse the graph in both directions.
    // storing a bi-directional reduces time complexity of lookups.
    child_edges: HashMap<&'a str, Vec<&'a str>>,
    parent_edges: HashMap<&'a str, &'a str>,
}

fn parse(input: &str) -> Tree {
    let tree = Tree {
        nodes: HashMap::new(),
        child_edges: HashMap::new(),
        parent_edges: HashMap::new(),
    };

    input.lines().fold(tree, |mut tree, line| {
        let (name, weight_str) = line.split(" (").collect_tuple().unwrap();
        let (weight, children_str) = weight_str.split(')').collect_tuple().unwrap();

        if !children_str.is_empty() {
            let children: Vec<&str> = children_str
                .split("-> ")
                .last()
                .unwrap()
                .split(", ")
                .collect();

            for child_name in children.iter() {
                tree.parent_edges.insert(child_name, name);
            }

            tree.child_edges.insert(name, children);
        }

        let node = Node {
            name,
            weight: weight.parse().unwrap(),
            weight_children: 0,
        };
        tree.nodes.insert(name, node);

        tree
    })
}

fn get_root_node_name<'a>(tree: &'a Tree) -> &'a str {
    let child_names: HashSet<&str> =
        tree.child_edges
            .values()
            .fold(HashSet::new(), |mut acc, names| {
                acc.extend(names);
                acc
            });

    *tree
        .nodes
        .keys()
        .find(|p| !child_names.contains(*p))
        .unwrap()
}

fn get_leaf_node_names<'a, 'b>(tree: &'a Tree<'b>) -> Vec<&'b str> {
    tree.nodes
        .keys()
        .filter_map(|p| {
            if !tree.child_edges.contains_key(*p) {
                Some(*p)
            } else {
                None
            }
        })
        .collect()
}

fn get_child_nodes<'a>(tree: &'a Tree, name: &'a str) -> Vec<&'a Node<'a>> {
    let child_names = tree.child_edges.get(name).unwrap();
    child_names
        .iter()
        .map(|child_name| tree.nodes.get(child_name).unwrap())
        .collect()
}

fn get_sibling_nodes<'a>(tree: &'a Tree, name: &'a str) -> Vec<&'a Node<'a>> {
    get_child_nodes(tree, tree.parent_edges.get(name).unwrap())
}

// traverses the tree bottom-up and adds total weight of children to each node.
fn sum_branch_weights(tree: &mut Tree, names: Vec<&str>) {
    let mut parent_names = vec![];

    for name in names {
        let node = tree.nodes.get(name).unwrap();
        let parent_name = tree.parent_edges.get(name);
        if let Some(parent_name) = parent_name {
            let child_weight = get_sibling_nodes(tree, node.name)
                .iter()
                .map(|c| c.total_weight())
                .sum();
            let parent = tree.nodes.get_mut(parent_name).unwrap();
            parent.weight_children = child_weight;
            parent_names.push(parent.name);
        }
    }

    if !parent_names.is_empty() {
        sum_branch_weights(tree, parent_names);
    }
}

// looks at a set of nodes and returns an unbalanced node if present, None otherwise.
fn find_unbalanced_node_name<'a>(nodes: &[&'a Node<'a>]) -> Option<&'a str> {
    let mut weights: HashMap<u32, Vec<&str>> = HashMap::new();

    for node in nodes {
        weights
            .entry(node.total_weight())
            .or_default()
            .push(node.name);
    }

    if weights.len() == 1 {
        None
    } else {
        weights
            .values()
            .find(|v| v.len() == 1)
            .unwrap()
            .first()
            .copied()
    }
}

pub fn part_one(input: &str) -> String {
    get_root_node_name(&parse(input)).to_string()
}

pub fn part_two(input: &str) -> u32 {
    let mut tree = parse(input);
    let leaf_node_names = get_leaf_node_names(&tree);
    sum_branch_weights(&mut tree, leaf_node_names);

    let mut current_node_name = get_root_node_name(&tree);
    let mut corrected_weight: Option<u32> = None;

    while corrected_weight.is_none() {
        let child_nodes = get_child_nodes(&tree, current_node_name);
        let unbalanced_node_name = find_unbalanced_node_name(&child_nodes);

        match unbalanced_node_name {
            Some(name) => {
                current_node_name = name;
            }
            None => {
                let siblings = get_sibling_nodes(&tree, current_node_name);
                let current = tree.nodes.get(current_node_name).unwrap();
                let current_total = current.total_weight();

                let canonical_weight = siblings
                    .into_iter()
                    .find_map(|node| {
                        let total_weight = node.total_weight();
                        if total_weight != current_total {
                            Some(total_weight)
                        } else {
                            None
                        }
                    })
                    .unwrap();

                corrected_weight = Some(current.weight + canonical_weight - current_total);
            }
        }
    }

    corrected_weight.unwrap()
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
        assert_eq!(part_two(&input), 60);
    }
}
