pub mod edge;
pub mod graph;
pub mod layout_engine;
pub mod node;

use dioxus::prelude::*;

pub use edge::{Edge, GraphEdgeKind};
pub use graph::{example_graph, Graph};
pub use layout_engine::dot::{
    graph_to_dot, DotLayoutEngine, DotLayoutOptions, GraphPoint, LayoutEdge, LayoutEngine,
    LayoutGraph, LayoutNode,
};
pub use node::{GraphNodeKind, Node};

#[component]
fn node_object(node: LayoutNode) -> Element {
    let active_class = if node.node.active {
        "is-active"
    } else {
        "is-inactive"
    };
    let class = format!(
        "graph-node graph-node--{} {active_class}",
        node.node.kind.css_class()
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
fn edge_object(edge: LayoutEdge) -> Element {
    let active_class = if edge.edge.active {
        "is-active"
    } else {
        "is-inactive"
    };
    let class = format!(
        "graph-edge graph-edge--{} {active_class}",
        edge.edge.kind.css_class()
    );

    rsx! {
        line {
            class: "{class}",
            x1: "{edge.source.x}",
            y1: "{edge.source.y}",
            x2: "{edge.target.x}",
            y2: "{edge.target.y}",
        }
    }
}

#[component]
pub fn comp_graph_canvas() -> Element {
    let graph = example_graph();
    let layout = DotLayoutEngine::default().layout(&graph);

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
