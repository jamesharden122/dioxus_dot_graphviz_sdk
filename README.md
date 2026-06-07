# dioxus_dot_graphviz_sdk

Dioxus components and small layout utilities for DOT/Graphviz-style directed graph canvases.

The crate is split into focused modules:

- `graph`: graph container and example graph data.
- `node`: node data and node kind metadata.
- `shape`: DOT/Graphviz polygon-based node shape metadata.
- `edge`: directed edge data and edge kind metadata.
- `layout_engine::dot`: hierarchical/layered layout for directed graphs, plus DOT export.

This README is included as the crate-level documentation with `include_str!`.

## Usage

Add the crate by path from a Dioxus app:

```toml
[dependencies]
dioxus_dot_graphviz_sdk = { path = "../dioxus_dot_graphviz_sdk" }
```

Render the bundled graph canvas component:

```rust
use dioxus::prelude::*;
use dioxus_dot_graphviz_sdk::comp_graph_canvas;

#[component]
fn App() -> Element {
    rsx! {
        comp_graph_canvas {}
    }
}
```

## Layout

`DotLayoutEngine` computes a top-to-bottom layered drawing from directed edges. Nodes with no incoming edges start at the top rank, and each edge pushes its target into a lower rank.

```rust
use dioxus_dot_graphviz_sdk::{DotLayoutEngine, LayoutEngine, example_graph};

let graph = example_graph();
let layout = DotLayoutEngine::default().layout(&graph);
```

Export the same graph shape to DOT:

```rust
use dioxus_dot_graphviz_sdk::{example_graph, graph_to_dot};

let dot = graph_to_dot(&example_graph());
```

Load a graph from JSON:

```rust
use dioxus_dot_graphviz_sdk::Graph;

let graph: Graph = Graph::from_json(r#"
{
  "nodes": [
    {
      "id": "A",
      "label": "Input",
      "detail": "source",
      "kind": "input",
      "shape": {
        "box": {
          "min": { "x": -0.5, "y": -0.3 },
          "max": { "x": 0.5, "y": 0.3 }
        }
      },
      "active": true,
      "tool": null
    }
  ],
  "edges": []
}
"#).expect("JSON graph should load");
```

The JSON loader does not fill defaults: `id`, `label`, `detail`, `kind`, `shape`, `active`, and `tool` are required for each node, and `id`, `from`, `to`, `kind`, `active`, and `tool` are required for each edge. Use `Graph<MyNodeTool, MyEdgeTool>` to deserialize caller-owned tool payloads without this crate importing those tool modules.

Rayon is available without default features through `dioxus_dot_graphviz_sdk::rayon`, and its prelude is re-exported from `dioxus_dot_graphviz_sdk::parallel`.

Build a graph layer by layer:

```rust
use dioxus_dot_graphviz_sdk::{Graph, GraphEdgeKind, GraphNodeKind, Node};

let mut graph = Graph::new();

graph.add_layer(
    vec![Node::new("input", "Input", "source", GraphNodeKind::Input)],
    GraphEdgeKind::Process,
).expect("layer should be valid");

graph.add_layer(
    vec![Node::new("state", "State", "target", GraphNodeKind::State)],
    GraphEdgeKind::Process,
).expect("layer should connect");
```

For custom edge tool payloads, use `add_layer_with_edges` and return `Edge<MyEdgeTool>` values from the closure.

## Node Kinds And Shapes

`GraphNodeKind` describes what a node means in the computational graph, such as input, state, decision variable, risk component, or utility.

`GraphNodeShape` describes how a node should be drawn when exported to DOT/Graphviz. The enum covers the polygon-based shapes from Graphviz, including common forms like `Box`, `Ellipse`, `Circle`, `Diamond`, `Hexagon`, `Octagon`, `Cylinder`, `Folder`, `Component`, and the Graphviz-specific `MDiamond`, `MSquare`, and `MCircle`.

Each `Node::new` assigns a default shape from its semantic kind. Override it explicitly when the drawing needs a different visual grammar:

```rust
use dioxus_dot_graphviz_sdk::{GraphNodeKind, GraphNodeShape, Node};
use geo::Rect;

let node = Node::new("Weights", "Decision weights", "allocator", GraphNodeKind::DecisionVariable)
    .with_shape(GraphNodeShape::Oval(Rect::new((-0.55, -0.32), (0.55, 0.32)).to_polygon()));
```

Layout output includes each node's geometry metadata. `LayoutNode::geometry` contains the `geo::Geometry`, its layout `row`, and the bounding `width` and `height`.

## Styling

The component expects the consuming app to provide CSS classes such as:

- `graph-canvas-panel`
- `graph-canvas-surface`
- `graph-edge-layer`
- `graph-node-layer`
- `graph-node`
- `graph-node-shape--diamond`
- `graph-edge`

`agent_factors_ui` already defines these classes in its app stylesheet.
