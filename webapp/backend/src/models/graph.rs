use sqlx::FromRow;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances = HashMap::new();
        let mut heap = BinaryHeap::new();

        distances.insert(from_node_id, 0);
        heap.push(State { cost: 0, position: from_node_id });

        while let Some(State { cost, position }) = heap.pop() {
            if position == to_node_id {
                return cost;
            }

            if let Some(edges) = self.edges.get(&position) {
                for edge in edges {
                    let next = State { cost: cost + edge.weight, position: edge.node_b_id };

                    let is_shorter = distances
                        .get(&edge.node_b_id)
                        .map_or(true, |&current| next.cost < current);

                    if is_shorter {
                        distances.insert(edge.node_b_id, next.cost);
                        heap.push(next);
                    }
                }
            }
        }

        i32::MAX
    }
}