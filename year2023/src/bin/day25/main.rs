use std::collections::{HashMap, HashSet};

type Node = usize;
type Graph = Vec<HashSet<Node>>;
type Time = i32;
type TimeMap = Vec<Time>;

type Edge = (Node, Node);

const UNDISCOVERED: Time = 0;

fn traverse_component(node: &Node, graph: &Graph, seen: &mut HashSet<Node>) {
    if seen.contains(node) {
        return;
    }
    seen.insert(node.clone());
    for neighbor in &graph[*node] {
        traverse_component(neighbor, graph, seen);
    }
}

/// Tarjan's algorithm; modified to return on first bridge
fn find_bridge_rec(
    current_node: &Node,
    parent_node: &Node,
    graph: &Graph,
    current_time: &mut Time,
    discovery_time: &mut TimeMap,
    min_time: &mut TimeMap,
) -> Option<Edge> {
    *current_time += 1;
    discovery_time[*current_node] = *current_time;
    min_time[*current_node] = *current_time;

    for neighbor in &graph[*current_node] {
        if neighbor == parent_node {
            continue;
        }

        if min_time[*neighbor] == UNDISCOVERED {
            match find_bridge_rec(
                neighbor,
                current_node,
                graph,
                current_time,
                discovery_time,
                min_time,
            ) {
                Some(bridge) => return Some(bridge),
                None => (),
            }
        }

        let neighbor_min_time = *min_time.get(*neighbor).unwrap();
        let old_min_time = *min_time.get(*current_node).unwrap();
        if neighbor_min_time < old_min_time {
            min_time[*current_node] = neighbor_min_time;
        }
        if neighbor_min_time > discovery_time[*current_node] {
            return Some((*current_node, *neighbor));
        }
    }
    return None;
}

fn find_bridge(graph: &Graph) -> Option<Edge> {
    let start_node = 0;
    let mut discovery_time = vec![UNDISCOVERED; graph.len()];
    let mut min_time = vec![UNDISCOVERED; graph.len()];
    return find_bridge_rec(
        &start_node,
        &discovery_time.len(), // initial parent - non-existent
        graph,
        &mut 0,
        &mut discovery_time,
        &mut min_time,
    );
}

#[inline]
fn remove_undirected_edge(graph: &mut Graph, n1: &Node, n2: &Node) {
    graph[*n1].remove(n2);
    graph[*n2].remove(n1);
}

#[inline]
/// Since we start with an empty graph, in case this function is called with a too big node,
/// i.e. index >= length, we must first add the missing neighbor sets.
fn get_mut_neighbors<'a>(graph: &'a mut Graph, node: &Node) -> &'a mut HashSet<Node> {
    for _ in graph.len()..(*node + 1) {
        graph.push(Default::default());
    }
    &mut graph[*node]
}

#[inline]
fn add_undirected_edge(graph: &mut Graph, n1: &Node, n2: &Node) {
    get_mut_neighbors(graph, n1).insert(*n2);
    get_mut_neighbors(graph, n2).insert(*n1);
}

fn get_numeric_node_name(node_names: &mut HashMap<String, Node>, name: &String) -> usize {
    match node_names.get(name) {
        Some(numeric_name) => *numeric_name,
        None => {
            let numeric_name = node_names.len();
            node_names.insert(name.clone(), numeric_name);
            numeric_name
        }
    }
}

fn main() {
    let mut node_names: HashMap<String, Node> = HashMap::new();
    let mut graph = Graph::default();
    let mut edges = Vec::<Edge>::new();
    for line in std::io::stdin().lines().map(|l| l.unwrap()) {
        let parts: Vec<String> = line.split(": ").map(|w| w.into()).collect();
        assert_eq!(parts.len(), 2);
        let parent_node = get_numeric_node_name(&mut node_names, &parts[0]);
        for child_node_raw in parts[1].split(" ") {
            let child_node = get_numeric_node_name(&mut node_names, &child_node_raw.to_string());
            add_undirected_edge(&mut graph, &parent_node, &child_node);
            edges.push((parent_node, child_node));
        }
    }

    for (i, e1) in edges.iter().enumerate() {
        remove_undirected_edge(&mut graph, &e1.0, &e1.1);
        for e2 in edges.iter().skip(i + 1) {
            remove_undirected_edge(&mut graph, &e2.0, &e2.1);

            match find_bridge(&graph) {
                None => (),
                Some(bridge) => {
                    remove_undirected_edge(&mut graph, &bridge.0, &bridge.1);
                    let mut seen = HashSet::new();
                    let start_node = 0;
                    traverse_component(&start_node, &graph, &mut seen);
                    println!("{}", seen.len() * (graph.len() - seen.len()));
                    return;
                }
            }

            add_undirected_edge(&mut graph, &e2.0, &e2.1);
        }
        add_undirected_edge(&mut graph, &e1.0, &e1.1);
    }
}
