use crate::{edge::Edge, node::Node};
use petgraph::stable_graph::{NodeIndex, StableGraph};
use serde::Deserialize;
use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub struct Graph {
    state: StableGraph<Node, Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            state: StableGraph::new(),
        }
    }

    pub fn from_parts(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Self::try_from_parts(nodes, edges).expect("graph parts must reference existing nodes")
    }

    pub fn try_from_parts(nodes: Vec<Node>, edges: Vec<Edge>) -> Result<Self, GraphBuildError> {
        let mut graph = Self::new();

        for node in nodes {
            graph.add_node(node);
        }

        for edge in edges {
            if !graph.add_edge(edge.clone()) {
                return Err(GraphBuildError::MissingEdgeEndpoint {
                    edge_id: edge.id,
                    from: edge.from,
                    to: edge.to,
                });
            }
        }

        Ok(graph)
    }

    pub fn from_json(json: &str) -> Result<Self, GraphJsonError> {
        let graph: GraphJson = serde_json::from_str(json)?;
        Self::try_from_parts(graph.nodes, graph.edges).map_err(GraphJsonError::Build)
    }

    pub fn add_node(&mut self, node: Node) -> NodeIndex {
        self.state.add_node(node)
    }

    pub fn add_edge(&mut self, edge: Edge) -> bool {
        let Some(from) = self.node_graph_index(&edge.from) else {
            return false;
        };
        let Some(to) = self.node_graph_index(&edge.to) else {
            return false;
        };

        self.state.add_edge(from, to, edge);
        true
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> + '_ {
        self.state.node_weights()
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> + '_ {
        self.state.edge_weights()
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

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct GraphJson {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
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
}

impl fmt::Display for GraphJsonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => write!(formatter, "{error}"),
            Self::Build(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for GraphJsonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            Self::Build(error) => Some(error),
        }
    }
}

impl From<serde_json::Error> for GraphJsonError {
    fn from(error: serde_json::Error) -> Self {
        Self::Parse(error)
    }
}

impl Default for Graph {
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
      "shape": {
        "cylinder": {
          "exterior": [
            { "x": -0.5, "y": -0.35 },
            { "x": 0.5, "y": -0.35 },
            { "x": 0.5, "y": 0.35 },
            { "x": -0.5, "y": 0.35 },
            { "x": -0.5, "y": -0.35 }
          ],
          "interiors": []
        }
      },
      "active": true
    },
    {
      "id": "ProbabilitySpace",
      "label": "Probability space",
      "detail": "moments",
      "kind": "state",
      "shape": {
        "ellipse": {
          "exterior": [
            { "x": -0.55, "y": -0.35 },
            { "x": 0.55, "y": -0.35 },
            { "x": 0.55, "y": 0.35 },
            { "x": -0.55, "y": 0.35 },
            { "x": -0.55, "y": -0.35 }
          ],
          "interiors": []
        }
      },
      "active": true
    },
    {
      "id": "SecurityReturnProcesses",
      "label": "Return processes",
      "detail": "state",
      "kind": "state",
      "shape": {
        "oval": {
          "exterior": [
            { "x": -0.55, "y": -0.32 },
            { "x": 0.55, "y": -0.32 },
            { "x": 0.55, "y": 0.32 },
            { "x": -0.55, "y": 0.32 },
            { "x": -0.55, "y": -0.32 }
          ],
          "interiors": []
        }
      },
      "active": true
    },
    {
      "id": "Weights",
      "label": "Decision weights",
      "detail": "allocator",
      "kind": "decision_variable",
      "shape": {
        "diamond": {
          "exterior": [
            { "x": 0.0, "y": 0.5 },
            { "x": 0.5, "y": 0.0 },
            { "x": 0.0, "y": -0.5 },
            { "x": -0.5, "y": 0.0 },
            { "x": 0.0, "y": 0.5 }
          ],
          "interiors": []
        }
      },
      "active": true
    },
    {
      "id": "PortfolioExpectedReturn",
      "label": "Expected return",
      "detail": "component",
      "kind": "portfolio_component",
      "shape": {
        "box": {
          "min": { "x": -0.5, "y": -0.3 },
          "max": { "x": 0.5, "y": 0.3 }
        }
      },
      "active": true
    },
    {
      "id": "PortfolioVariance",
      "label": "Variance",
      "detail": "risk",
      "kind": "portfolio_risk_component",
      "shape": {
        "hexagon": {
          "exterior": [
            { "x": -0.5, "y": 0.0 },
            { "x": -0.25, "y": 0.43 },
            { "x": 0.25, "y": 0.43 },
            { "x": 0.5, "y": 0.0 },
            { "x": 0.25, "y": -0.43 },
            { "x": -0.25, "y": -0.43 },
            { "x": -0.5, "y": 0.0 }
          ],
          "interiors": []
        }
      },
      "active": true
    },
    {
      "id": "PortfolioSkewness",
      "label": "Skewness",
      "detail": "asymmetry",
      "kind": "portfolio_asymmetry_component",
      "shape": {
        "octagon": {
          "exterior": [
            { "x": -0.2, "y": 0.5 },
            { "x": 0.2, "y": 0.5 },
            { "x": 0.5, "y": 0.2 },
            { "x": 0.5, "y": -0.2 },
            { "x": 0.2, "y": -0.5 },
            { "x": -0.2, "y": -0.5 },
            { "x": -0.5, "y": -0.2 },
            { "x": -0.5, "y": 0.2 },
            { "x": -0.2, "y": 0.5 }
          ],
          "interiors": []
        }
      },
      "active": true
    },
    {
      "id": "Utility",
      "label": "Utility",
      "detail": "objective",
      "kind": "utility",
      "shape": {
        "double_circle": {
          "exterior": [
            { "x": -0.5, "y": -0.5 },
            { "x": 0.5, "y": -0.5 },
            { "x": 0.5, "y": 0.5 },
            { "x": -0.5, "y": 0.5 },
            { "x": -0.5, "y": -0.5 }
          ],
          "interiors": []
        }
      },
      "active": true
    }
  ],
  "edges": [
    {
      "id": "D->ProbabilitySpace",
      "from": "D",
      "to": "ProbabilitySpace",
      "kind": "data",
      "active": true
    },
    {
      "id": "ProbabilitySpace->SecurityReturnProcesses",
      "from": "ProbabilitySpace",
      "to": "SecurityReturnProcesses",
      "kind": "process",
      "active": true
    },
    {
      "id": "SecurityReturnProcesses->Weights",
      "from": "SecurityReturnProcesses",
      "to": "Weights",
      "kind": "process",
      "active": true
    },
    {
      "id": "SecurityReturnProcesses->PortfolioExpectedReturn",
      "from": "SecurityReturnProcesses",
      "to": "PortfolioExpectedReturn",
      "kind": "state_contribution",
      "active": true
    },
    {
      "id": "SecurityReturnProcesses->PortfolioVariance",
      "from": "SecurityReturnProcesses",
      "to": "PortfolioVariance",
      "kind": "state_contribution",
      "active": true
    },
    {
      "id": "SecurityReturnProcesses->PortfolioSkewness",
      "from": "SecurityReturnProcesses",
      "to": "PortfolioSkewness",
      "kind": "state_contribution",
      "active": true
    },
    {
      "id": "Weights->PortfolioExpectedReturn",
      "from": "Weights",
      "to": "PortfolioExpectedReturn",
      "kind": "decision_contribution",
      "active": true
    },
    {
      "id": "Weights->PortfolioVariance",
      "from": "Weights",
      "to": "PortfolioVariance",
      "kind": "decision_contribution",
      "active": true
    },
    {
      "id": "Weights->PortfolioSkewness",
      "from": "Weights",
      "to": "PortfolioSkewness",
      "kind": "decision_contribution",
      "active": true
    },
    {
      "id": "PortfolioExpectedReturn->Utility",
      "from": "PortfolioExpectedReturn",
      "to": "Utility",
      "kind": "utility_aggregation",
      "active": true
    },
    {
      "id": "PortfolioVariance->Utility",
      "from": "PortfolioVariance",
      "to": "Utility",
      "kind": "utility_aggregation",
      "active": true
    },
    {
      "id": "PortfolioSkewness->Utility",
      "from": "PortfolioSkewness",
      "to": "Utility",
      "kind": "utility_aggregation",
      "active": true
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

    #[test]
    fn builds_graph_from_json_with_explicit_shape() {
        let graph = Graph::from_json(
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
                  "active": true
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
                  "active": true
                }
              ],
              "edges": [
                {
                  "id": "A->B",
                  "from": "A",
                  "to": "B",
                  "kind": "data",
                  "active": true
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
    fn from_json_rejects_missing_shape() {
        let error = Graph::from_json(
            r#"
            {
              "nodes": [
                {
                  "id": "A",
                  "label": "Input",
                  "detail": "source",
                  "kind": "input",
                  "active": true
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
