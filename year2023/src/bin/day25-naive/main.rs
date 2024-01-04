use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Graph = HashMap<String, HashSet<String>>;

fn get_component_members(
    node: &String,
    graph: &Graph,
    removed_edges: &HashSet<Edge>,
) -> HashSet<String> {
    let mut queue = vec![node];
    let mut component_members = HashSet::new();
    while !queue.is_empty() {
        let current_node = queue.pop().unwrap();
        if component_members.contains(current_node) {
            continue;
        }
        component_members.insert(current_node.clone());

        for neighbor in graph.get(current_node).unwrap() {
            if removed_edges.contains(&(current_node.clone(), neighbor.clone()))
                || removed_edges.contains(&(neighbor.clone(), current_node.clone()))
            {
                continue;
            }
            queue.push(neighbor);
        }
    }

    return component_members;
}

fn get_two_components(graph: &Graph, removed_edges: &HashSet<Edge>) -> Option<Vec<usize>> {
    let mut seen = HashSet::<String>::new();
    let mut component_sizes = vec![];
    for node in graph.keys() {
        if seen.contains(node) {
            continue;
        }
        if component_sizes.len() == 2 {
            // already split into enough parts
            return None;
        }

        let component = get_component_members(node, graph, removed_edges);
        component_sizes.push(component.len());
        seen.extend(component);
    }

    return match component_sizes.len() {
        1 => None,
        2 => Some(component_sizes),
        _ => panic!("Invalid components found: {component_sizes:?}"),
    };
}

type Edge = (String, String);

fn main() {
    let mut graph = Graph::default();
    let mut edges = Vec::<Edge>::new();
    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let parts: Vec<&str> = line.split(": ").collect();
        assert_eq!(parts.len(), 2);
        let parent_node = parts[0].to_string();
        for child_node in parts[1].split(" ") {
            graph
                .entry(parent_node.clone())
                .or_default()
                .insert(child_node.to_string());

            graph
                .entry(child_node.to_string())
                .or_default()
                .insert(parent_node.clone());

            if parent_node == child_node {
                println!("Self edge detected: {parent_node}");
            }
            edges.push((parent_node.clone(), child_node.to_string()))
        }
    }

    let edge_count = edges.len();
    println!("Edges: {}", edge_count);
    println!(
        "Edge triplets: {}",
        edge_count * (edge_count - 1) * (edge_count - 2) / 6
    );
    println!(
        "This is a naive solution that would take around 1.5 years to finish on 1 core of my CPU"
    );

    for (node, neighbors) in graph.iter() {
        let neighbor_count = neighbors.len();
        if neighbor_count <= 3 {
            println!("{node} has {neighbor_count} neighbors");
        }
    }

    for removed_edge_indices in (0..edges.len()).combinations(3) {
        let removed_edges =
            HashSet::from_iter(removed_edge_indices.iter().map(|i| edges[*i].clone()));
        if let Some(component_sizes) = get_two_components(&graph, &removed_edges) {
            assert_eq!(component_sizes.len(), 2);
            let sol: usize = component_sizes.iter().product();
            println!("{}", sol);
            break;
        }
    }
}
