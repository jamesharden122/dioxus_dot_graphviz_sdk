use std::collections::{BTreeMap, HashMap, VecDeque};

use crate::{edge::Edge, graph::Graph, node::Node};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GraphPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutNode {
    pub node: Node,
    pub position: GraphPoint,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutEdge {
    pub edge: Edge,
    pub source: GraphPoint,
    pub target: GraphPoint,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutGraph {
    pub nodes: Vec<LayoutNode>,
    pub edges: Vec<LayoutEdge>,
}

pub trait LayoutEngine {
    fn layout(&self, graph: &Graph) -> LayoutGraph;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DotLayoutOptions {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl Default for DotLayoutOptions {
    fn default() -> Self {
        Self {
            x_min: 8.0,
            x_max: 92.0,
            y_min: 9.0,
            y_max: 91.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DotLayoutEngine {
    pub options: DotLayoutOptions,
}

impl LayoutEngine for DotLayoutEngine {
    fn layout(&self, graph: &Graph) -> LayoutGraph {
        let ranks = hierarchical_ranks(graph);
        let positions = rank_positions(&ranks, self.options);

        let nodes = graph
            .nodes
            .iter()
            .cloned()
            .enumerate()
            .map(|(idx, node)| LayoutNode {
                node,
                position: positions[idx],
            })
            .collect();

        let edges = graph
            .edges
            .iter()
            .filter_map(|edge| {
                let from = graph.node_index(edge.from)?;
                let to = graph.node_index(edge.to)?;
                Some(LayoutEdge {
                    edge: edge.clone(),
                    source: positions[from],
                    target: positions[to],
                })
            })
            .collect();

        LayoutGraph { nodes, edges }
    }
}

pub fn graph_to_dot(graph: &Graph) -> String {
    let mut dot = String::from("digraph G {\n  rankdir=TB;\n");

    for node in &graph.nodes {
        dot.push_str("  \"");
        dot.push_str(&escape_dot(node.id));
        dot.push_str("\" [label=\"");
        dot.push_str(&escape_dot(node.label));
        dot.push_str("\"];\n");
    }

    for edge in &graph.edges {
        dot.push_str("  \"");
        dot.push_str(&escape_dot(edge.from));
        dot.push_str("\" -> \"");
        dot.push_str(&escape_dot(edge.to));
        dot.push_str("\" [label=\"");
        dot.push_str(&escape_dot(edge.id));
        dot.push_str("\"];\n");
    }

    dot.push_str("}\n");
    dot
}

fn hierarchical_ranks(graph: &Graph) -> Vec<usize> {
    let node_count = graph.nodes.len();
    let mut node_indices = HashMap::with_capacity(node_count);
    for (idx, node) in graph.nodes.iter().enumerate() {
        node_indices.insert(node.id, idx);
    }

    let mut outgoing = vec![Vec::new(); node_count];
    let mut indegree = vec![0usize; node_count];
    for edge in graph.edges.iter().filter(|edge| edge.active) {
        if let (Some(&from), Some(&to)) = (node_indices.get(edge.from), node_indices.get(edge.to)) {
            outgoing[from].push(to);
            indegree[to] += 1;
        }
    }

    let mut ranks = vec![0usize; node_count];
    let mut queue = VecDeque::new();
    for (idx, degree) in indegree.iter().enumerate() {
        if *degree == 0 {
            queue.push_back(idx);
        }
    }

    let mut visited = 0usize;
    while let Some(from) = queue.pop_front() {
        visited += 1;
        for &to in &outgoing[from] {
            ranks[to] = ranks[to].max(ranks[from] + 1);
            indegree[to] -= 1;
            if indegree[to] == 0 {
                queue.push_back(to);
            }
        }
    }

    if visited < node_count {
        for (idx, rank) in ranks.iter_mut().enumerate() {
            *rank = (*rank).max(idx);
        }
    }

    ranks
}

fn rank_positions(ranks: &[usize], options: DotLayoutOptions) -> Vec<GraphPoint> {
    let mut by_rank: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (idx, rank) in ranks.iter().enumerate() {
        by_rank.entry(*rank).or_default().push(idx);
    }

    let rank_count = by_rank.len().max(1);
    let y_step = if rank_count == 1 {
        0.0
    } else {
        (options.y_max - options.y_min) / (rank_count - 1) as f32
    };

    let mut positions = vec![
        GraphPoint {
            x: midpoint(options.x_min, options.x_max),
            y: options.y_min,
        };
        ranks.len()
    ];

    for (rank_slot, indices) in by_rank.values().enumerate() {
        let y = options.y_min + y_step * rank_slot as f32;
        for (node_slot, node_idx) in indices.iter().enumerate() {
            positions[*node_idx] = GraphPoint {
                x: slot_position(options.x_min, options.x_max, node_slot, indices.len()),
                y,
            };
        }
    }

    positions
}

fn slot_position(min: f32, max: f32, slot: usize, count: usize) -> f32 {
    if count <= 1 {
        return midpoint(min, max);
    }

    min + (max - min) * (slot + 1) as f32 / (count + 1) as f32
}

fn midpoint(min: f32, max: f32) -> f32 {
    min + (max - min) / 2.0
}

fn escape_dot(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
