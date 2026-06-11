#![doc = include_str!("../README.md")]

pub mod edge;
pub mod graph;
pub mod layout_engine;
pub mod node;
pub mod shape;

use dioxus::prelude::*;

pub use dioxus;
pub use edge::{Edge, GraphEdgeKind, GraphEdgeShape};
pub use graph::{example_graph, Graph, GraphBuildError, GraphJsonError, EXAMPLE_GRAPH_JSON};
pub use layout_engine::dot::{
    graph_to_dot, DotLayoutEngine, DotLayoutOptions, GraphPoint, LayoutEdge, LayoutEngine,
    LayoutGraph, LayoutNode,
};
pub use node::{GraphNodeKind, Node, ToolArgs};
pub use rayon;
pub use shape::{GraphNodeGeometry, GraphNodeShape};

pub mod parallel {
    pub use rayon::prelude::*;
}

#[component]
fn node_object<NodeTool>(node: LayoutNode<NodeTool>) -> Element
where
    NodeTool: Clone + PartialEq + 'static,
{
    let active_class = if node.node.active {
        "is-active"
    } else {
        "is-inactive"
    };
    let class = format!(
        "graph-node graph-node--{} graph-node-shape--{} {active_class}",
        node.node.kind.css_class(),
        node.node.shape.css_class()
    );
    let style = format!(
        "left: {:.2}%; top: {:.2}%;",
        node.position.x, node.position.y
    );
    let kind_label = node.node.kind.label().to_string();

    rsx! {
        article {
            class: "{class}",
            style: "{style}",
            title: "{node.node.id}",
            span { class: "graph-node-kind", "{kind_label}" }
            strong { class: "graph-node-title", "{node.node.label}" }
            span { class: "graph-node-detail", "{node.node.detail}" }
        }
    }
}

#[component]
fn edge_object<EdgeTool>(edge: LayoutEdge<EdgeTool>) -> Element
where
    EdgeTool: Clone + PartialEq + 'static,
{
    let active_class = if edge.edge.active {
        "is-active"
    } else {
        "is-inactive"
    };
    let class = format!(
        "graph-edge graph-edge--{} {active_class}",
        edge.edge.kind.css_class()
    );
    let source_slot_count = edge.source_slot_count.max(1) as f32;
    let target_slot_count = edge.target_slot_count.max(1) as f32;
    let source_offset = if source_slot_count > 1.0 {
        (edge.source_slot as f32 - (source_slot_count - 1.0) / 2.0) * 4.0
    } else {
        0.0
    };
    let target_offset = if target_slot_count > 1.0 {
        (edge.target_slot as f32 - (target_slot_count - 1.0) / 2.0) * 4.0
    } else {
        0.0
    };
    let source_x = edge.source.x + source_offset;
    let target_x = edge.target.x + target_offset;
    let dy = edge.target.y - edge.source.y;
    let direction = if dy >= 0.0 { 1.0 } else { -1.0 };
    let source_y = edge.source.y + direction * 3.0;
    let target_y = edge.target.y - direction * 3.0;
    let dx = target_x - source_x;
    let dy = target_y - source_y;
    let bend = if dx.abs() < 0.1 && dy.abs() > 24.0 {
        8.0
    } else {
        0.0
    };
    let path = format!(
        "M {:.2} {:.2} C {:.2} {:.2}, {:.2} {:.2}, {:.2} {:.2}",
        source_x,
        source_y,
        source_x + bend,
        source_y + dy * 0.45,
        target_x + bend,
        target_y - dy * 0.45,
        target_x,
        target_y
    );

    rsx! {
        path {
            class: "{class}",
            d: "{path}",
            fill: "none",
        }
    }
}

pub fn graph_canvas<NodeTool, EdgeTool>(graph: &Graph<NodeTool, EdgeTool>) -> Element
where
    NodeTool: Clone + PartialEq + 'static,
    EdgeTool: Clone + PartialEq + 'static,
{
    let layout = DotLayoutEngine::default().layout(graph);

    rsx! {
        section { class: "graph-canvas-panel",
            div { class: "graph-canvas-surface",
                svg {
                    class: "graph-edge-layer",
                    view_box: "0 0 100 100",
                    preserve_aspect_ratio: "none",
                    for edge in layout.edges {
                        edge_object {
                            key: "{edge.edge.id}",
                            edge
                        }
                    }
                }
                div { class: "graph-node-layer",
                    for node in layout.nodes {
                        node_object {
                            key: "{node.node.id}",
                            node
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn comp_graph_canvas() -> Element {
    graph_canvas(&example_graph())
}
