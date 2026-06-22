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
    graph_to_dot, DotLayoutEngine, DotLayoutOptions, GraphPoint, LayeredSvgGeometry,
    LayeredSvgGeometryError, LayoutEdge, LayoutEngine, LayoutGraph, LayoutNode,
    DEFAULT_CONNECTOR_RATIO, DEFAULT_NODE_RATIO,
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
    let marker_url = format!("url(#graph-arrow-{})", edge.edge.kind.css_class());
    let path = format!(
        "M {:.2} {:.2} C {:.2} {:.2}, {:.2} {:.2}, {:.2} {:.2}",
        edge.source.x,
        edge.source.y,
        edge.source_control.x,
        edge.source_control.y,
        edge.target_control.x,
        edge.target_control.y,
        edge.target.x,
        edge.target.y
    );

    rsx! {
        path {
            class: "{class}",
            d: "{path}",
            fill: "none",
            marker_end: "{marker_url}",
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
                    width: "100%",
                    height: "100%",
                    view_box: "0 0 100 100",
                    preserve_aspect_ratio: "none",
                    defs {
                        marker {
                            id: "graph-arrow-data",
                            view_box: "0 0 8 8",
                            ref_x: "7",
                            ref_y: "4",
                            marker_width: "6",
                            marker_height: "6",
                            orient: "auto",
                            marker_units: "strokeWidth",
                            path { d: "M 0 0 L 8 4 L 0 8 z", fill: "#7bbf8a" }
                        }
                        marker {
                            id: "graph-arrow-process",
                            view_box: "0 0 8 8",
                            ref_x: "7",
                            ref_y: "4",
                            marker_width: "6",
                            marker_height: "6",
                            orient: "auto",
                            marker_units: "strokeWidth",
                            path { d: "M 0 0 L 8 4 L 0 8 z", fill: "#74a7c9" }
                        }
                        marker {
                            id: "graph-arrow-state",
                            view_box: "0 0 8 8",
                            ref_x: "7",
                            ref_y: "4",
                            marker_width: "6",
                            marker_height: "6",
                            orient: "auto",
                            marker_units: "strokeWidth",
                            path { d: "M 0 0 L 8 4 L 0 8 z", fill: "#74a7c9" }
                        }
                        marker {
                            id: "graph-arrow-decision",
                            view_box: "0 0 8 8",
                            ref_x: "7",
                            ref_y: "4",
                            marker_width: "6",
                            marker_height: "6",
                            orient: "auto",
                            marker_units: "strokeWidth",
                            path { d: "M 0 0 L 8 4 L 0 8 z", fill: "#d7b46a" }
                        }
                        marker {
                            id: "graph-arrow-utility",
                            view_box: "0 0 8 8",
                            ref_x: "7",
                            ref_y: "4",
                            marker_width: "6",
                            marker_height: "6",
                            orient: "auto",
                            marker_units: "strokeWidth",
                            path { d: "M 0 0 L 8 4 L 0 8 z", fill: "#d98c8c" }
                        }
                    }
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
