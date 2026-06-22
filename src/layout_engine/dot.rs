use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    error::Error,
    fmt,
};

use crate::{edge::Edge, graph::Graph, node::Node, shape::GraphNodeGeometry};

const NODE_GRID_WIDTH: f32 = 17.6;
const NODE_GRID_HEIGHT: f32 = 9.6;
pub const DEFAULT_NODE_RATIO: f32 = 0.48;
pub const DEFAULT_CONNECTOR_RATIO: f32 = 0.52;
const RATIO_EPSILON: f32 = 0.0001;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GraphPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayeredSvgGeometry {
    pub svg_width: f32,
    pub svg_height: f32,
    pub margin_left: f32,
    pub margin_right: f32,
    pub margin_top: f32,
    pub margin_bottom: f32,
    pub layer_count: usize,
    pub node_ratio: f32,
    pub connector_ratio: f32,
}

impl LayeredSvgGeometry {
    pub fn new(
        svg_width: f32,
        svg_height: f32,
        margin_left: f32,
        margin_right: f32,
        margin_top: f32,
        margin_bottom: f32,
        layer_count: usize,
    ) -> Result<Self, LayeredSvgGeometryError> {
        Self::with_ratios(
            svg_width,
            svg_height,
            margin_left,
            margin_right,
            margin_top,
            margin_bottom,
            layer_count,
            DEFAULT_NODE_RATIO,
            DEFAULT_CONNECTOR_RATIO,
            false,
        )
    }

    pub fn with_ratios(
        svg_width: f32,
        svg_height: f32,
        margin_left: f32,
        margin_right: f32,
        margin_top: f32,
        margin_bottom: f32,
        layer_count: usize,
        mut node_ratio: f32,
        mut connector_ratio: f32,
        normalize_ratios: bool,
    ) -> Result<Self, LayeredSvgGeometryError> {
        if layer_count == 0 {
            return Err(LayeredSvgGeometryError::EmptyLayerCount);
        }

        let inner_width = svg_width - margin_left - margin_right;
        let inner_height = svg_height - margin_top - margin_bottom;
        if inner_width <= 0.0 || inner_height <= 0.0 {
            return Err(LayeredSvgGeometryError::InvalidBounds);
        }

        let ratio_sum = node_ratio + connector_ratio;
        if !node_ratio.is_finite()
            || !connector_ratio.is_finite()
            || node_ratio <= 0.0
            || connector_ratio <= 0.0
            || ratio_sum <= 0.0
        {
            return Err(LayeredSvgGeometryError::InvalidRatios);
        }

        if normalize_ratios {
            node_ratio /= ratio_sum;
            connector_ratio /= ratio_sum;
        } else if (ratio_sum - 1.0).abs() > RATIO_EPSILON {
            return Err(LayeredSvgGeometryError::InvalidRatioSum { ratio_sum });
        }

        Ok(Self {
            svg_width,
            svg_height,
            margin_left,
            margin_right,
            margin_top,
            margin_bottom,
            layer_count,
            node_ratio,
            connector_ratio,
        })
    }

    pub fn inner_width(self) -> f32 {
        self.svg_width - self.margin_left - self.margin_right
    }

    pub fn inner_height(self) -> f32 {
        self.svg_height - self.margin_top - self.margin_bottom
    }

    pub fn node_band_height(self) -> f32 {
        self.inner_height() * self.node_ratio / self.layer_count as f32
    }

    pub fn connector_band_height(self) -> f32 {
        self.inner_height() * self.connector_ratio / self.layer_count as f32
    }

    pub fn x_from_norm(self, x_norm: f32) -> f32 {
        self.margin_left + x_norm.clamp(0.0, 1.0) * self.inner_width()
    }

    pub fn node_band_top(self, layer: usize) -> f32 {
        self.margin_top + layer as f32 * (self.node_band_height() + self.connector_band_height())
    }

    pub fn connector_band_top(self, layer: usize) -> f32 {
        self.node_band_top(layer) + self.node_band_height()
    }

    pub fn connector_band_center_y(self, layer: usize) -> f32 {
        self.connector_band_top(layer) + self.connector_band_height() / 2.0
    }

    pub fn node_center(self, layer: usize, x_norm: f32) -> GraphPoint {
        GraphPoint {
            x: self.x_from_norm(x_norm),
            y: self.node_band_top(layer) + self.node_band_height() / 2.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayeredSvgGeometryError {
    EmptyLayerCount,
    InvalidBounds,
    InvalidRatios,
    InvalidRatioSum { ratio_sum: f32 },
}

impl fmt::Display for LayeredSvgGeometryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLayerCount => write!(formatter, "layer count must be greater than zero"),
            Self::InvalidBounds => write!(formatter, "SVG margins must leave positive inner space"),
            Self::InvalidRatios => write!(
                formatter,
                "node and connector ratios must be positive finite values"
            ),
            Self::InvalidRatioSum { ratio_sum } => {
                write!(
                    formatter,
                    "node and connector ratios must sum to 1.0, got {ratio_sum}"
                )
            }
        }
    }
}

impl Error for LayeredSvgGeometryError {}

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutNode<NodeTool = ()> {
    pub node: Node<NodeTool>,
    pub position: GraphPoint,
    pub geometry: Option<GraphNodeGeometry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutEdge<EdgeTool = ()> {
    pub edge: Edge<EdgeTool>,
    pub source: GraphPoint,
    pub source_control: GraphPoint,
    pub target_control: GraphPoint,
    pub target: GraphPoint,
    pub source_slot: usize,
    pub source_slot_count: usize,
    pub target_slot: usize,
    pub target_slot_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutGraph<NodeTool = (), EdgeTool = ()> {
    pub nodes: Vec<LayoutNode<NodeTool>>,
    pub edges: Vec<LayoutEdge<EdgeTool>>,
}

#[derive(Clone, Debug, PartialEq)]
struct RankLayout {
    positions: Vec<GraphPoint>,
    layers: Vec<usize>,
    geometry: LayeredSvgGeometry,
}

pub trait LayoutEngine<NodeTool = (), EdgeTool = ()> {
    fn layout(&self, graph: &Graph<NodeTool, EdgeTool>) -> LayoutGraph<NodeTool, EdgeTool>;
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

impl<NodeTool, EdgeTool> LayoutEngine<NodeTool, EdgeTool> for DotLayoutEngine
where
    NodeTool: Clone,
    EdgeTool: Clone,
{
    fn layout(&self, graph: &Graph<NodeTool, EdgeTool>) -> LayoutGraph<NodeTool, EdgeTool> {
        let ranks = hierarchical_ranks(graph);
        let rank_layout = rank_positions(&ranks, self.options);

        let nodes = graph
            .nodes()
            .cloned()
            .enumerate()
            .map(|(idx, node)| {
                let geometry = node.shape.geometry_for_row(ranks[idx]);

                LayoutNode {
                    node,
                    position: rank_layout.positions[idx],
                    geometry,
                }
            })
            .collect();

        let edge_records = graph
            .edges()
            .filter_map(|edge| {
                let from = graph.node_index(&edge.from)?;
                let to = graph.node_index(&edge.to)?;
                Some((edge, from, to))
            })
            .collect::<Vec<_>>();

        let mut source_slot_counts = vec![0usize; rank_layout.positions.len()];
        let mut target_slot_counts = vec![0usize; rank_layout.positions.len()];
        for (_, from, to) in &edge_records {
            source_slot_counts[*from] += 1;
            target_slot_counts[*to] += 1;
        }

        let mut source_slots = vec![0usize; rank_layout.positions.len()];
        let mut target_slots = vec![0usize; rank_layout.positions.len()];
        let edges = edge_records
            .into_iter()
            .map(|(edge, from, to)| {
                let source_slot = source_slots[from];
                let target_slot = target_slots[to];
                let source_slot_count = source_slot_counts[from].max(1);
                let target_slot_count = target_slot_counts[to].max(1);
                source_slots[from] += 1;
                target_slots[to] += 1;

                let source_layer = rank_layout.layers[from];
                let target_layer = rank_layout.layers[to];
                let source_center = rank_layout.positions[from];
                let target_center = rank_layout.positions[to];
                let source_offset = ((source_slot + 1) as f32 / (source_slot_count + 1) as f32
                    - 0.5)
                    * NODE_GRID_WIDTH;
                let target_offset = ((target_slot + 1) as f32 / (target_slot_count + 1) as f32
                    - 0.5)
                    * NODE_GRID_WIDTH;
                let source_side_offset =
                    ((source_slot + 1) as f32 / (source_slot_count + 1) as f32 - 0.5)
                        * NODE_GRID_HEIGHT;
                let target_side_offset =
                    ((target_slot + 1) as f32 / (target_slot_count + 1) as f32 - 0.5)
                        * NODE_GRID_HEIGHT;
                let (source, source_control, target_control, target) =
                    if target_layer > source_layer {
                        let source_connector_y =
                            rank_layout.geometry.connector_band_center_y(source_layer);
                        let target_connector_y = rank_layout
                            .geometry
                            .connector_band_center_y(target_layer.saturating_sub(1));

                        (
                            GraphPoint {
                                x: source_center.x + source_offset,
                                y: source_center.y + NODE_GRID_HEIGHT / 2.0,
                            },
                            GraphPoint {
                                x: source_center.x + source_offset,
                                y: source_connector_y,
                            },
                            GraphPoint {
                                x: target_center.x + target_offset,
                                y: target_connector_y,
                            },
                            GraphPoint {
                                x: target_center.x + target_offset,
                                y: target_center.y - NODE_GRID_HEIGHT / 2.0,
                            },
                        )
                    } else if target_layer < source_layer {
                        let source_connector_y = rank_layout
                            .geometry
                            .connector_band_center_y(source_layer.saturating_sub(1));
                        let target_connector_y =
                            rank_layout.geometry.connector_band_center_y(target_layer);

                        (
                            GraphPoint {
                                x: source_center.x + source_offset,
                                y: source_center.y - NODE_GRID_HEIGHT / 2.0,
                            },
                            GraphPoint {
                                x: source_center.x + source_offset,
                                y: source_connector_y,
                            },
                            GraphPoint {
                                x: target_center.x + target_offset,
                                y: target_connector_y,
                            },
                            GraphPoint {
                                x: target_center.x + target_offset,
                                y: target_center.y + NODE_GRID_HEIGHT / 2.0,
                            },
                        )
                    } else if target_center.x >= source_center.x {
                        (
                            GraphPoint {
                                x: source_center.x + NODE_GRID_WIDTH / 2.0,
                                y: source_center.y + source_side_offset,
                            },
                            GraphPoint {
                                x: midpoint(source_center.x, target_center.x),
                                y: source_center.y + source_side_offset,
                            },
                            GraphPoint {
                                x: midpoint(source_center.x, target_center.x),
                                y: target_center.y + target_side_offset,
                            },
                            GraphPoint {
                                x: target_center.x - NODE_GRID_WIDTH / 2.0,
                                y: target_center.y + target_side_offset,
                            },
                        )
                    } else {
                        (
                            GraphPoint {
                                x: source_center.x - NODE_GRID_WIDTH / 2.0,
                                y: source_center.y + source_side_offset,
                            },
                            GraphPoint {
                                x: midpoint(source_center.x, target_center.x),
                                y: source_center.y + source_side_offset,
                            },
                            GraphPoint {
                                x: midpoint(source_center.x, target_center.x),
                                y: target_center.y + target_side_offset,
                            },
                            GraphPoint {
                                x: target_center.x + NODE_GRID_WIDTH / 2.0,
                                y: target_center.y + target_side_offset,
                            },
                        )
                    };

                LayoutEdge {
                    edge: edge.clone(),
                    source,
                    source_control,
                    target_control,
                    target,
                    source_slot,
                    source_slot_count,
                    target_slot,
                    target_slot_count,
                }
            })
            .collect();

        LayoutGraph { nodes, edges }
    }
}

pub fn graph_to_dot<NodeTool, EdgeTool>(graph: &Graph<NodeTool, EdgeTool>) -> String {
    let mut dot = String::from("digraph G {\n  rankdir=TB;\n");

    for node in graph.nodes() {
        dot.push_str("  \"");
        dot.push_str(&escape_dot(&node.id));
        dot.push_str("\" [label=\"");
        dot.push_str(&escape_dot(&node.label));
        dot.push_str("\", shape=\"");
        dot.push_str(node.shape.dot_name());
        dot.push_str("\"];\n");
    }

    for edge in graph.edges() {
        dot.push_str("  \"");
        dot.push_str(&escape_dot(&edge.from));
        dot.push_str("\" -> \"");
        dot.push_str(&escape_dot(&edge.to));
        dot.push_str("\" [label=\"");
        dot.push_str(&escape_dot(&edge.id));
        dot.push_str("\"];\n");
    }

    dot.push_str("}\n");
    dot
}

fn hierarchical_ranks<NodeTool, EdgeTool>(graph: &Graph<NodeTool, EdgeTool>) -> Vec<usize> {
    let nodes: Vec<_> = graph.nodes().collect();
    let node_count = nodes.len();
    let mut node_indices = HashMap::with_capacity(node_count);
    for (idx, node) in nodes.iter().enumerate() {
        node_indices.insert(node.id.as_str(), idx);
    }

    let mut outgoing = vec![Vec::new(); node_count];
    let mut indegree = vec![0usize; node_count];
    for edge in graph.edges().filter(|edge| edge.active) {
        if let (Some(&from), Some(&to)) = (
            node_indices.get(edge.from.as_str()),
            node_indices.get(edge.to.as_str()),
        ) {
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

fn rank_positions(ranks: &[usize], options: DotLayoutOptions) -> RankLayout {
    let mut by_rank: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (idx, rank) in ranks.iter().enumerate() {
        by_rank.entry(*rank).or_default().push(idx);
    }

    let layer_count = by_rank.len().max(1);
    let geometry = LayeredSvgGeometry::new(
        100.0,
        100.0,
        options.x_min,
        100.0 - options.x_max,
        options.y_min,
        100.0 - options.y_max,
        layer_count,
    )
    .expect("default layered SVG geometry must be valid");

    let mut positions = vec![
        GraphPoint {
            x: geometry.x_from_norm(0.5),
            y: geometry.node_center(0, 0.5).y,
        };
        ranks.len()
    ];
    let mut layers = vec![0usize; ranks.len()];

    for (layer, indices) in by_rank.values().enumerate() {
        for (node_slot, node_idx) in indices.iter().enumerate() {
            let x_norm = (node_slot + 1) as f32 / (indices.len() + 1) as f32;
            positions[*node_idx] = geometry.node_center(layer, x_norm);
            layers[*node_idx] = layer;
        }
    }

    RankLayout {
        positions,
        layers,
        geometry,
    }
}

fn midpoint(min: f32, max: f32) -> f32 {
    min + (max - min) / 2.0
}

fn escape_dot(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::example_graph;

    fn assert_close(left: f32, right: f32) {
        assert!(
            (left - right).abs() < 0.001,
            "expected {left} to be close to {right}"
        );
    }

    #[test]
    fn layered_svg_geometry_splits_global_ratios_by_layer_count() {
        let geometry = LayeredSvgGeometry::new(800.0, 600.0, 40.0, 60.0, 20.0, 30.0, 4)
            .expect("default ratios should be valid");

        assert_close(geometry.inner_width(), 700.0);
        assert_close(geometry.inner_height(), 550.0);
        assert_close(
            geometry.node_band_height(),
            550.0 * DEFAULT_NODE_RATIO / 4.0,
        );
        assert_close(
            geometry.connector_band_height(),
            550.0 * DEFAULT_CONNECTOR_RATIO / 4.0,
        );
        assert_close(geometry.x_from_norm(0.25), 215.0);
        assert_close(
            geometry.node_center(1, 0.5).y,
            20.0 + geometry.node_band_height()
                + geometry.connector_band_height()
                + geometry.node_band_height() / 2.0,
        );
        assert_close(
            geometry.connector_band_center_y(0),
            20.0 + geometry.node_band_height() + geometry.connector_band_height() / 2.0,
        );
    }

    #[test]
    fn layered_svg_geometry_rejects_bad_ratio_sum_without_normalization() {
        let error =
            LayeredSvgGeometry::with_ratios(100.0, 100.0, 0.0, 0.0, 0.0, 0.0, 2, 0.7, 0.4, false)
                .expect_err("bad ratio sum should be rejected");

        assert!(matches!(
            error,
            LayeredSvgGeometryError::InvalidRatioSum { .. }
        ));

        let geometry =
            LayeredSvgGeometry::with_ratios(100.0, 100.0, 0.0, 0.0, 0.0, 0.0, 2, 0.7, 0.4, true)
                .expect("normalization should allow non-1 ratio sum");
        assert_close(geometry.node_ratio + geometry.connector_ratio, 1.0);
    }

    #[test]
    fn layout_fans_out_edges_from_weights() {
        let graph = example_graph();
        let layout = DotLayoutEngine::default().layout(&graph);
        let weights_node = layout
            .nodes
            .iter()
            .find(|node| node.node.id == "Weights")
            .expect("weights node should be laid out");
        let weight_edges = layout
            .edges
            .iter()
            .filter(|edge| edge.edge.from == "Weights")
            .collect::<Vec<_>>();

        assert_eq!(weight_edges.len(), 3);
        assert!(weight_edges.iter().all(|edge| edge.source_slot_count == 3));
        assert!(weight_edges
            .iter()
            .all(|edge| edge.source.y > weights_node.position.y));
        assert!(weight_edges[0].source.x < weight_edges[1].source.x);
        assert!(weight_edges[1].source.x < weight_edges[2].source.x);
        for edge in &weight_edges {
            let target_node = layout
                .nodes
                .iter()
                .find(|node| node.node.id == edge.edge.to)
                .expect("target node should be laid out");
            assert!(edge.target.y < target_node.position.y);
            assert!(edge.source_control.y > edge.source.y);
            assert!(edge.target_control.y < edge.target.y);
        }
        assert_eq!(
            weight_edges
                .iter()
                .map(|edge| (edge.edge.to.as_str(), edge.source_slot))
                .collect::<Vec<_>>(),
            vec![
                ("PortfolioExpectedReturn", 0),
                ("PortfolioVariance", 1),
                ("PortfolioSkewness", 2)
            ]
        );
    }
}
