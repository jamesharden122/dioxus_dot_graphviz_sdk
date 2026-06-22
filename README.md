# dioxus_dot_graphviz_sdk

Dioxus components and layout utilities for DOT/Graphviz-style directed graph canvases. The crate is intended for applications that want to render planner-generated or programmatically-built directed computational graphs without depending on a browser Graphviz runtime.

This README is included as the crate-level documentation with `include_str!`.

## Modules

- `graph`: graph container, JSON loading, dependency-level traversal, and example graph data.
- `node`: node data, semantic node kinds, `ToolArgs`, and node tool payload helpers.
- `edge`: directed edge data, semantic edge kinds, route-shape metadata, and edge tool payload helpers.
- `shape`: DOT/Graphviz polygon-based node shape metadata backed by `geo` geometry.
- `layout_engine::dot`: hierarchical/layered directed graph layout, SVG geometry, and DOT export.

## Install

Add the crate by path from a Dioxus app:

```toml
[dependencies]
dioxus_dot_graphviz_sdk = { path = "../dioxus_dot_graphviz_sdk" }
```

The crate re-exports `dioxus`, `rayon`, and the common graph types from the crate root.

## Quick Start

Render the bundled example graph:

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

Render a graph you build yourself:

```rust
use dioxus::prelude::*;
use dioxus_dot_graphviz_sdk::{Graph, GraphEdgeKind, GraphNodeKind, Node, graph_canvas};

#[component]
fn App() -> Element {
    let mut graph = Graph::new();
    graph
        .add_layer(
            vec![Node::new("data", "Data", "input panel", GraphNodeKind::Input)],
            GraphEdgeKind::Process,
        )
        .expect("first layer should be valid");
    graph
        .add_layer(
            vec![Node::new("state", "State", "estimated process", GraphNodeKind::State)],
            GraphEdgeKind::Process,
        )
        .expect("second layer should connect");

    rsx! {
        {graph_canvas(&graph)}
    }
}
```

## Graph Construction

Use `Graph::new`, `add_node`, and `add_edge` for explicit graph construction, or `Graph::try_from_parts` when you already have node and edge vectors.

```rust
use dioxus_dot_graphviz_sdk::{Edge, Graph, GraphEdgeKind, GraphNodeKind, Node};

let nodes = vec![
    Node::new("data", "Data", "source", GraphNodeKind::Input),
    Node::new("utility", "Utility", "objective", GraphNodeKind::Utility),
];
let edges = vec![Edge::new("data_to_utility", "data", "utility", GraphEdgeKind::Process)];

let graph = Graph::try_from_parts(nodes, edges).expect("edge endpoints should exist");
```

`Graph::node_mut(id)` gives mutable access to a node by stable string id, which is useful for render state such as marking a node active while it runs.

```rust
use dioxus_dot_graphviz_sdk::{Edge, Graph, GraphEdgeKind, GraphNodeKind, Node};

let mut graph = Graph::try_from_parts(
    vec![
        Node::new("data", "Data", "source", GraphNodeKind::Input),
        Node::new("utility", "Utility", "objective", GraphNodeKind::Utility),
    ],
    vec![Edge::new("data_to_utility", "data", "utility", GraphEdgeKind::Process)],
).expect("edge endpoints should exist");

if let Some(node) = graph.node_mut("utility") {
    node.active = false;
}
```

`Graph::dependency_levels()` returns topological execution/render levels as stable node ids. It returns `GraphBuildError::CyclicDependencies` if the graph contains a cycle.

```rust
use dioxus_dot_graphviz_sdk::{Edge, Graph, GraphEdgeKind, GraphNodeKind, Node};

let graph = Graph::try_from_parts(
    vec![
        Node::new("data", "Data", "source", GraphNodeKind::Input),
        Node::new("utility", "Utility", "objective", GraphNodeKind::Utility),
    ],
    vec![Edge::new("data_to_utility", "data", "utility", GraphEdgeKind::Process)],
).expect("edge endpoints should exist");

let levels = graph.dependency_levels().expect("graph should be acyclic");
for level in levels {
    println!("run these nodes together: {level:?}");
}
```

## Tool Payloads

`Node<Tool>` and `Edge<Tool>` are generic over caller-owned tool payloads. The SDK does not import application tool modules; your app chooses the payload type.

```rust
use dioxus_dot_graphviz_sdk::{GraphNodeKind, Node};

#[derive(Clone, Debug, PartialEq)]
enum Command {
    ReadBin,
    Simulate,
}

let node = Node::new("data", "Data", "load panel", GraphNodeKind::Input)
    .with_tool(Some(Command::ReadBin));
```

Use `with_tool` to change the payload type and `with_active` to update render state fluently. The same helpers exist on `Edge<Tool>`.

```rust
use dioxus_dot_graphviz_sdk::{Edge, GraphEdgeKind};

let edge = Edge::new("data_to_state", "data", "state", GraphEdgeKind::Data)
    .with_tool(serde_json::json!({ "dataset_key": "returns_panel" }))
    .with_active(false);
```

## Planner Prompt Spec

For planner-produced graphs, emit a minimal JSON object with `edges` and `nodes`. Runners such as `agent_factors_bm::eventloop::EngineGraphPlan` still need both collections, but planner prompts should be edge-first: edges define dependency flow and carry the parameters that feed the child node tool.

Return only JSON. Do not include prose or markdown fences. Tool names should come from the runtime `available_tools` context, not from a static tool catalog. Prefer empty node `call_tool.arguments` and place executable parameters on incoming edges. For a root executable node, add a non-executable context/source node and a bootstrap edge into that root node.

```json
{
  "edges": [
    {
      "id": "context_to_data",
      "from": "planner_context",
      "to": "data",
      "kind": "data",
      "label": "read panel arguments",
      "tool": {
        "dataset_key": "returns_panel",
        "panel_kind": "ols",
        "base_path": "data/monthly_usbank_crsp"
      }
    },
    {
      "id": "data_to_probability_space",
      "from": "data",
      "to": "probability_space",
      "kind": "process",
      "label": "simulation arguments from loaded data",
      "tool": {}
    }
  ],
  "nodes": [
    {
      "id": "planner_context",
      "label": "Planner context",
      "description": "non-executable parameter source",
      "kind": "input",
      "tool": null
    },
    {
      "id": "data",
      "label": "Data",
      "description": "load return panel",
      "kind": "input",
      "tool": {
        "call_tool": {
          "name": "read_bin",
          "arguments": {}
        }
      }
    },
    {
      "id": "probability_space",
      "label": "Probability space",
      "description": "simulate return distribution",
      "kind": "state",
      "tool": {
        "call_tool": {
          "name": "simulation",
          "arguments": {}
        }
      }
    }
  ]
}
```

Edge fields:

- `id`: stable string id. If omitted by a permissive runner, `from->to` may be used, but explicit ids are preferred.
- `from` and `to`: parent and child node ids. The child receives parameters from the incoming edge.
- `kind`: semantic edge kind, for example `data`, `process`, `decision_contribution`, `state_contribution`, or `utility_aggregation`.
- `tool`: JSON object for child-node arguments or transform metadata. Use `{}` when no edge payload is needed.

Node fields:

- `id`: stable string id used by edges and render state.
- `label`: short display label.
- `description` or `detail`: compact display detail.
- `kind`: semantic node kind, for example `input`, `state`, `decision_variable`, `portfolio_component`, `portfolio_risk_component`, `portfolio_asymmetry_component`, or `utility`.
- `tool`: `null` for display/pass-through nodes, or `{ "call_tool": { "name": string, "arguments": {} } }` for executable nodes. Put dynamic arguments on incoming edges unless the node has true static defaults.

For economic utility prompts, wrap the user utility objective with a preconfigured utility graph scaffold, then let the planner prune irrelevant scaffold nodes and attach edge payloads where execution parameters are required.

## JSON Loading

Load a graph from JSON when the graph is produced by a planner or stored outside Rust code:

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
      "shape": "box",
      "active": true,
      "tool": null
    }
  ],
  "edges": []
}
"#).expect("JSON graph should load");
```

The JSON loader does not fill missing fields: `id`, `label`, `detail`, `kind`, `shape`, `active`, and `tool` are required for each node, and `id`, `from`, `to`, `kind`, `active`, and `tool` are required for each edge. `shape` may be either a default shape name such as `"box"`, `"ellipse"`, `"diamond"`, `"hexagon"`, or a full geometry object.

Use `Graph<MyNodeTool, MyEdgeTool>` to deserialize caller-owned tool payloads.

## Layout And DOT Export

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

The layout module also exposes lower-level geometry types such as `LayeredSvgGeometry`, `LayeredSvgGeometryError`, `GraphPoint`, `LayoutNode`, and `LayoutEdge`. `DEFAULT_NODE_RATIO` and `DEFAULT_CONNECTOR_RATIO` are exported for apps that want to align custom rendering with the default SVG layout.

## Node Kinds And Shapes

`GraphNodeKind` describes what a node means in the computational graph, such as input, state, decision variable, risk component, asymmetry component, or utility.

`GraphNodeShape` describes how a node should be drawn when exported to DOT/Graphviz. The enum covers the polygon-based shapes from Graphviz, including common forms like `Box`, `Ellipse`, `Circle`, `Diamond`, `Hexagon`, `Octagon`, `Cylinder`, `Folder`, `Component`, and Graphviz-specific `MDiamond`, `MSquare`, and `MCircle`.

`GraphNodeShape::default_for_name("hexagon")` returns the default geometry used when JSON specifies `"shape": "hexagon"`.

Each `Node::new` assigns a default shape from its semantic kind. Override it explicitly when the drawing needs a different visual grammar:

```rust
use dioxus_dot_graphviz_sdk::{GraphNodeKind, GraphNodeShape, Node};
use geo::Rect;

let node = Node::new("Weights", "Decision weights", "allocator", GraphNodeKind::DecisionVariable)
    .with_shape(GraphNodeShape::Oval(Rect::new((-0.55, -0.32), (0.55, 0.32)).to_polygon()));
```

Layout output includes each node's geometry metadata. `LayoutNode::geometry` contains the `geo::Geometry`, its layout row, and the bounding width and height.

`GraphEdgeShape` describes the route family for an edge, currently `Straight`, `Curved`, `Orthogonal`, or `Stepped`.

## Styling

The component expects the consuming app to provide CSS classes such as:

- `graph-canvas-panel`
- `graph-canvas-surface`
- `graph-edge-layer`
- `graph-node-layer`
- `graph-node`
- `graph-node-shape--diamond`
- `graph-edge`
- `graph-edge--data`
- `graph-edge--process`

`agent_factors_ui` already defines these classes in its app stylesheet.

## Parallel Utilities

Rayon is available without default features through `dioxus_dot_graphviz_sdk::rayon`, and its prelude is re-exported from `dioxus_dot_graphviz_sdk::parallel`.
