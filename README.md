# dioxus_dot_graphviz_sdk

Dioxus components and small layout utilities for DOT/Graphviz-style directed graph canvases.

The crate is split into focused modules:

- `graph`: graph container and example graph data.
- `node`: node data and node kind metadata.
- `edge`: directed edge data and edge kind metadata.
- `layout_engine::dot`: hierarchical/layered layout for directed graphs, plus DOT export.

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

## Styling

The component expects the consuming app to provide CSS classes such as:

- `graph-canvas-panel`
- `graph-canvas-surface`
- `graph-edge-layer`
- `graph-node-layer`
- `graph-node`
- `graph-edge`

`agent_factors_ui` already defines these classes in its app stylesheet.
# dioxus_dot_graphviz_sdk
