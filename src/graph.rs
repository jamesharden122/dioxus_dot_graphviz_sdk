use crate::{
    edge::{Edge, GraphEdgeKind},
    node::Node,
    shape::GraphNodeShape,
};
use petgraph::stable_graph::{NodeIndex, StableGraph};
use serde::Deserialize;
use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub struct Graph<NodeTool = (), EdgeTool = ()> {
    state: StableGraph<Node<NodeTool>, Edge<EdgeTool>>,
    layers: Vec<Vec<NodeIndex>>,
}

impl<NodeTool, EdgeTool> Graph<NodeTool, EdgeTool> {
    pub fn new() -> Self {
        Self {
            state: StableGraph::new(),
            layers: Vec::new(),
        }
    }

    pub fn from_parts(nodes: Vec<Node<NodeTool>>, edges: Vec<Edge<EdgeTool>>) -> Self {
        Self::try_from_parts(nodes, edges).expect("graph parts must reference existing nodes")
    }

    pub fn try_from_parts(
        nodes: Vec<Node<NodeTool>>,
        edges: Vec<Edge<EdgeTool>>,
    ) -> Result<Self, GraphBuildError> {
        let mut graph = Self::new();

        for node in nodes {
            graph.add_node(node);
        }

        for edge in edges {
            let edge_id = edge.id.clone();
            let from = edge.from.clone();
            let to = edge.to.clone();
            if !graph.add_edge(edge) {
                return Err(GraphBuildError::MissingEdgeEndpoint { edge_id, from, to });
            }
        }

        Ok(graph)
    }

    pub fn from_json(json: &str) -> Result<Self, GraphJsonError>
    where
        NodeTool: for<'de> Deserialize<'de>,
        EdgeTool: for<'de> Deserialize<'de>,
    {
        let mut value: serde_json::Value = serde_json::from_str(json)?;

        if let Some(nodes) = value
            .get_mut("nodes")
            .and_then(|nodes| nodes.as_array_mut())
        {
            for node in nodes {
                let Some(shape) = node.get_mut("shape") else {
                    continue;
                };
                let Some(shape_name) = shape.as_str().map(str::to_owned) else {
                    continue;
                };
                let shape_value = GraphNodeShape::default_for_name(&shape_name)
                    .ok_or_else(|| GraphJsonError::UnknownShape(shape_name))?;
                *shape = serde_json::to_value(shape_value)?;
            }
        }

        let graph: GraphJson<NodeTool, EdgeTool> = serde_json::from_value(value)?;
        Self::try_from_parts(graph.nodes, graph.edges).map_err(GraphJsonError::Build)
    }

    pub fn add_node(&mut self, node: Node<NodeTool>) -> NodeIndex {
        self.state.add_node(node)
    }

    pub fn add_layer_with_edges<F>(
        &mut self,
        nodes: impl IntoIterator<Item = Node<NodeTool>>,
        mut edge_builder: F,
    ) -> Result<Vec<NodeIndex>, GraphBuildError>
    where
        F: FnMut(&Node<NodeTool>, &Node<NodeTool>) -> Option<Edge<EdgeTool>>,
    {
        let previous_layer = self.layers.last().cloned().unwrap_or_default();
        let new_layer = nodes
            .into_iter()
            .map(|node| self.add_node(node))
            .collect::<Vec<_>>();

        let mut edges = Vec::new();
        for from_idx in &previous_layer {
            for to_idx in &new_layer {
                let from = &self.state[*from_idx];
                let to = &self.state[*to_idx];
                if let Some(edge) = edge_builder(from, to) {
                    edges.push(edge);
                }
            }
        }

        for edge in edges {
            let edge_id = edge.id.clone();
            let from = edge.from.clone();
            let to = edge.to.clone();
            if !self.add_edge(edge) {
                return Err(GraphBuildError::MissingEdgeEndpoint { edge_id, from, to });
            }
        }

        self.layers.push(new_layer.clone());
        Ok(new_layer)
    }

    pub fn add_edge(&mut self, edge: Edge<EdgeTool>) -> bool {
        let Some(from) = self.node_graph_index(&edge.from) else {
            return false;
        };
        let Some(to) = self.node_graph_index(&edge.to) else {
            return false;
        };

        self.state.add_edge(from, to, edge);
        true
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node<NodeTool>> + '_ {
        self.state.node_weights()
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge<EdgeTool>> + '_ {
        self.state.edge_weights()
    }

    pub fn layers(&self) -> &[Vec<NodeIndex>] {
        &self.layers
    }

    pub fn node_index(&self, id: &str) -> Option<usize> {
        self.nodes().position(|node| node.id == id)
    }

    fn node_graph_index(&self, id: &str) -> Option<NodeIndex> {
        self.state
            .node_indices()
            .find(|&node_index| self.state[node_index].id == id)
    }
}

impl<NodeTool> Graph<NodeTool, ()> {
    pub fn add_layer(
        &mut self,
        nodes: impl IntoIterator<Item = Node<NodeTool>>,
        edge_kind: GraphEdgeKind,
    ) -> Result<Vec<NodeIndex>, GraphBuildError> {
        self.add_layer_with_edges(nodes, |from, to| {
            Some(Edge::new(
                format!("{}->{}", from.id, to.id),
                from.id.clone(),
                to.id.clone(),
                edge_kind,
            ))
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct GraphJson<NodeTool = (), EdgeTool = ()> {
    nodes: Vec<Node<NodeTool>>,
    edges: Vec<Edge<EdgeTool>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraphBuildError {
    MissingEdgeEndpoint {
        edge_id: String,
        from: String,
        to: String,
    },
}

impl fmt::Display for GraphBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEdgeEndpoint { edge_id, from, to } => write!(
                formatter,
                "edge {edge_id:?} references missing endpoint(s): from={from:?}, to={to:?}"
            ),
        }
    }
}

impl Error for GraphBuildError {}

#[derive(Debug)]
pub enum GraphJsonError {
    Parse(serde_json::Error),
    Build(GraphBuildError),
    UnknownShape(String),
}

impl fmt::Display for GraphJsonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => write!(formatter, "{error}"),
            Self::Build(error) => write!(formatter, "{error}"),
            Self::UnknownShape(shape) => write!(formatter, "unknown graph node shape {shape:?}"),
        }
    }
}

impl Error for GraphJsonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            Self::Build(error) => Some(error),
            Self::UnknownShape(_) => None,
        }
    }
}

impl From<serde_json::Error> for GraphJsonError {
    fn from(error: serde_json::Error) -> Self {
        Self::Parse(error)
    }
}

impl<NodeTool, EdgeTool> Default for Graph<NodeTool, EdgeTool> {
    fn default() -> Self {
        Self::new()
    }
}

pub const EXAMPLE_GRAPH_JSON: &str = r#"
{
  "nodes": [
    {
      "id": "D",
      "label": "ReadBin data",
      "detail": "input panel",
      "kind": "input",
      "shape": "cylinder",
      "active": true,
    "tool": null
    },
    {
      "id": "ProbabilitySpace",
      "label": "Probability space",
      "detail": "moments",
      "kind": "state",
      "shape": "ellipse",
      "active": true,
    "tool": null
    },
    {
      "id": "SecurityReturnProcesses",
      "label": "Return processes",
      "detail": "state",
      "kind": "state",
      "shape": "oval",
      "active": true,
    "tool": null
    },
    {
      "id": "Weights",
      "label": "Decision weights",
      "detail": "allocator",
      "kind": "decision_variable",
      "shape": "diamond",
      "active": true,
    "tool": null
    },
    {
      "id": "PortfolioExpectedReturn",
      "label": "Expected return",
      "detail": "component",
      "kind": "portfolio_component",
      "shape": "box",
      "active": true,
    "tool": null
    },
    {
      "id": "PortfolioVariance",
      "label": "Variance",
      "detail": "risk",
      "kind": "portfolio_risk_component",
      "shape": "hexagon",
      "active": true,
    "tool": null
    },
    {
      "id": "PortfolioSkewness",
      "label": "Skewness",
      "detail": "asymmetry",
      "kind": "portfolio_asymmetry_component",
      "shape": "octagon",
      "active": true,
    "tool": null
    },
    {
      "id": "Utility",
      "label": "Utility",
      "detail": "objective",
      "kind": "utility",
      "shape": "double_circle",
      "active": true,
    "tool": null
    }
  ],
  "edges": [
    {
      "id": "D->ProbabilitySpace",
      "from": "D",
      "to": "ProbabilitySpace",
      "kind": "data",
      "active": true,
    "tool": null
    },
    {
      "id": "ProbabilitySpace->SecurityReturnProcesses",
      "from": "ProbabilitySpace",
      "to": "SecurityReturnProcesses",
      "kind": "process",
      "active": true,
    "tool": null
    },
    {
      "id": "SecurityReturnProcesses->Weights",
      "from": "SecurityReturnProcesses",
      "to": "Weights",
      "kind": "process",
      "active": true,
    "tool": null
    },
    {
      "id": "SecurityReturnProcesses->PortfolioExpectedReturn",
      "from": "SecurityReturnProcesses",
      "to": "PortfolioExpectedReturn",
      "kind": "state_contribution",
      "active": true,
    "tool": null
    },
    {
      "id": "SecurityReturnProcesses->PortfolioVariance",
      "from": "SecurityReturnProcesses",
      "to": "PortfolioVariance",
      "kind": "state_contribution",
      "active": true,
    "tool": null
    },
    {
      "id": "SecurityReturnProcesses->PortfolioSkewness",
      "from": "SecurityReturnProcesses",
      "to": "PortfolioSkewness",
      "kind": "state_contribution",
      "active": true,
    "tool": null
    },
    {
      "id": "Weights->PortfolioExpectedReturn",
      "from": "Weights",
      "to": "PortfolioExpectedReturn",
      "kind": "decision_contribution",
      "active": true,
    "tool": null
    },
    {
      "id": "Weights->PortfolioVariance",
      "from": "Weights",
      "to": "PortfolioVariance",
      "kind": "decision_contribution",
      "active": true,
    "tool": null
    },
    {
      "id": "Weights->PortfolioSkewness",
      "from": "Weights",
      "to": "PortfolioSkewness",
      "kind": "decision_contribution",
      "active": true,
    "tool": null
    },
    {
      "id": "PortfolioExpectedReturn->Utility",
      "from": "PortfolioExpectedReturn",
      "to": "Utility",
      "kind": "utility_aggregation",
      "active": true,
    "tool": null
    },
    {
      "id": "PortfolioVariance->Utility",
      "from": "PortfolioVariance",
      "to": "Utility",
      "kind": "utility_aggregation",
      "active": true,
    "tool": null
    },
    {
      "id": "PortfolioSkewness->Utility",
      "from": "PortfolioSkewness",
      "to": "Utility",
      "kind": "utility_aggregation",
      "active": true,
    "tool": null
    }
  ]
}
"#;

pub fn example_graph() -> Graph {
    Graph::from_json(EXAMPLE_GRAPH_JSON).expect("example graph JSON must be valid")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::GraphNodeKind;

    #[test]
    fn add_layer_connects_from_previous_layer() {
        let mut graph = Graph::new();

        graph
            .add_layer(
                vec![Node::new("A", "Input", "source", GraphNodeKind::Input)],
                GraphEdgeKind::Process,
            )
            .expect("first layer should build");
        graph
            .add_layer(
                vec![
                    Node::new("B", "State B", "target", GraphNodeKind::State),
                    Node::new("C", "State C", "target", GraphNodeKind::State),
                ],
                GraphEdgeKind::Process,
            )
            .expect("second layer should connect");

        assert_eq!(graph.layers().len(), 2);
        assert_eq!(graph.nodes().count(), 3);
        assert_eq!(graph.edges().count(), 2);
    }

    #[test]
    fn add_layer_accepts_custom_tool_payloads() {
        let mut graph: Graph<String, String> = Graph::new();

        graph
            .add_layer_with_edges(
                vec![Node::new_with_tool(
                    "A",
                    "Input",
                    "source",
                    GraphNodeKind::Input,
                    "node-tool-a".to_string(),
                )],
                |_from, _to| None,
            )
            .expect("first layer should build");
        graph
            .add_layer_with_edges(
                vec![Node::new_with_tool(
                    "B",
                    "State",
                    "target",
                    GraphNodeKind::State,
                    "node-tool-b".to_string(),
                )],
                |from: &Node<String>, to: &Node<String>| {
                    Some(Edge::new_with_tool(
                        format!("{}->{}", from.id, to.id),
                        from.id.clone(),
                        to.id.clone(),
                        GraphEdgeKind::Process,
                        "edge-tool".to_string(),
                    ))
                },
            )
            .expect("second layer should connect");

        assert_eq!(
            graph.edges().next().map(|edge| edge.tool.as_str()),
            Some("edge-tool")
        );
    }

    #[test]
    fn builds_graph_from_json_with_default_shape_name() {
        let graph: Graph = Graph::from_json(
            r#"
            {
              "nodes": [
                {
                  "id": "A",
                  "label": "Input",
                  "detail": "source",
                  "kind": "input",
                  "shape": "cylinder",
                  "active": true,
                "tool": null
                },
                {
                  "id": "B",
                  "label": "Risk",
                  "detail": "target",
                  "kind": "portfolio_risk_component",
                  "shape": "hexagon",
                  "active": true,
                "tool": null
                }
              ],
              "edges": [
                {
                  "id": "A->B",
                  "from": "A",
                  "to": "B",
                  "kind": "data",
                  "active": true,
                "tool": null
                }
              ]
            }
            "#,
        )
        .expect("valid JSON graph should load");

        let shapes = graph
            .nodes()
            .map(|node| node.shape.dot_name())
            .collect::<Vec<_>>();

        assert_eq!(shapes, vec!["cylinder", "hexagon"]);
    }

    #[test]
    fn builds_graph_from_json_with_explicit_shape() {
        let graph: Graph = Graph::from_json(
            r#"
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
                },
                {
                  "id": "B",
                  "label": "State",
                  "detail": "target",
                  "kind": "state",
                  "shape": {
                    "rect": {
                      "min": { "x": -0.5, "y": -0.3 },
                      "max": { "x": 0.5, "y": 0.3 }
                    }
                  },
                  "active": true,
                "tool": null
                }
              ],
              "edges": [
                {
                  "id": "A->B",
                  "from": "A",
                  "to": "B",
                  "kind": "data",
                  "active": true,
                "tool": null
                }
              ]
            }
            "#,
        )
        .expect("valid JSON graph should load");

        assert_eq!(graph.nodes().count(), 2);
        assert_eq!(graph.edges().count(), 1);
    }

    #[test]
    fn from_json_rejects_unknown_default_shape_name() {
        let error = Graph::<(), ()>::from_json(
            r#"
            {
              "nodes": [
                {
                  "id": "A",
                  "label": "Input",
                  "detail": "source",
                  "kind": "input",
                  "shape": "not_a_shape",
                  "active": true,
                "tool": null
                }
              ],
              "edges": []
            }
            "#,
        )
        .expect_err("unknown shape should be rejected");

        assert!(matches!(error, GraphJsonError::UnknownShape(shape) if shape == "not_a_shape"));
    }

    #[test]
    fn from_json_rejects_missing_shape() {
        let error = Graph::<(), ()>::from_json(
            r#"
            {
              "nodes": [
                {
                  "id": "A",
                  "label": "Input",
                  "detail": "source",
                  "kind": "input",
                  "active": true,
                "tool": null
                }
              ],
              "edges": []
            }
            "#,
        )
        .expect_err("shape is required");

        assert!(matches!(error, GraphJsonError::Parse(_)));
    }
}
