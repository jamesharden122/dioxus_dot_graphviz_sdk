use crate::{
    edge::{Edge, GraphEdgeKind},
    node::{GraphNodeKind, Node},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Self { nodes, edges }
    }

    pub fn node_index(&self, id: &str) -> Option<usize> {
        self.nodes.iter().position(|node| node.id == id)
    }
}

pub fn example_graph() -> Graph {
    Graph::new(
        vec![
            Node::new("D", "ReadBin data", "input panel", GraphNodeKind::Input),
            Node::new(
                "ProbabilitySpace",
                "Probability space",
                "moments",
                GraphNodeKind::State,
            ),
            Node::new(
                "SecurityReturnProcesses",
                "Return processes",
                "state",
                GraphNodeKind::State,
            ),
            Node::new(
                "Weights",
                "Decision weights",
                "allocator",
                GraphNodeKind::DecisionVariable,
            ),
            Node::new(
                "PortfolioExpectedReturn",
                "Expected return",
                "component",
                GraphNodeKind::PortfolioComponent,
            ),
            Node::new(
                "PortfolioVariance",
                "Variance",
                "risk",
                GraphNodeKind::PortfolioRiskComponent,
            ),
            Node::new(
                "PortfolioSkewness",
                "Skewness",
                "asymmetry",
                GraphNodeKind::PortfolioAsymmetryComponent,
            ),
            Node::new("Utility", "Utility", "objective", GraphNodeKind::Utility),
        ],
        vec![
            Edge::new(
                "D->ProbabilitySpace",
                "D",
                "ProbabilitySpace",
                GraphEdgeKind::Data,
            ),
            Edge::new(
                "ProbabilitySpace->SecurityReturnProcesses",
                "ProbabilitySpace",
                "SecurityReturnProcesses",
                GraphEdgeKind::Process,
            ),
            Edge::new(
                "SecurityReturnProcesses->PortfolioExpectedReturn",
                "SecurityReturnProcesses",
                "PortfolioExpectedReturn",
                GraphEdgeKind::StateContribution,
            ),
            Edge::new(
                "SecurityReturnProcesses->PortfolioVariance",
                "SecurityReturnProcesses",
                "PortfolioVariance",
                GraphEdgeKind::StateContribution,
            ),
            Edge::new(
                "SecurityReturnProcesses->PortfolioSkewness",
                "SecurityReturnProcesses",
                "PortfolioSkewness",
                GraphEdgeKind::StateContribution,
            ),
            Edge::new(
                "Weights->PortfolioExpectedReturn",
                "Weights",
                "PortfolioExpectedReturn",
                GraphEdgeKind::DecisionContribution,
            ),
            Edge::new(
                "Weights->PortfolioVariance",
                "Weights",
                "PortfolioVariance",
                GraphEdgeKind::DecisionContribution,
            ),
            Edge::new(
                "Weights->PortfolioSkewness",
                "Weights",
                "PortfolioSkewness",
                GraphEdgeKind::DecisionContribution,
            ),
            Edge::new(
                "PortfolioExpectedReturn->Utility",
                "PortfolioExpectedReturn",
                "Utility",
                GraphEdgeKind::UtilityAggregation,
            ),
            Edge::new(
                "PortfolioVariance->Utility",
                "PortfolioVariance",
                "Utility",
                GraphEdgeKind::UtilityAggregation,
            ),
            Edge::new(
                "PortfolioSkewness->Utility",
                "PortfolioSkewness",
                "Utility",
                GraphEdgeKind::UtilityAggregation,
            ),
        ],
    )
}
