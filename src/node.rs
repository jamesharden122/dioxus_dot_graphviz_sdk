use crate::shape::GraphNodeShape;
use serde::{Deserialize, Serialize};

pub trait ToolArgs: Send + Sync {}

impl ToolArgs for () {}

impl<T: ToolArgs + ?Sized> ToolArgs for &T {}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphNodeKind {
    Input,
    State,
    DecisionVariable,
    PortfolioComponent,
    PortfolioRiskComponent,
    PortfolioAsymmetryComponent,
    Utility,
}

impl GraphNodeKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Input => "Input",
            Self::State => "State",
            Self::DecisionVariable => "Decision",
            Self::PortfolioComponent => "Return",
            Self::PortfolioRiskComponent => "Risk",
            Self::PortfolioAsymmetryComponent => "Asymmetry",
            Self::Utility => "Utility",
        }
    }

    pub fn css_class(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::State => "state",
            Self::DecisionVariable => "decision-variable",
            Self::PortfolioComponent => "portfolio-component",
            Self::PortfolioRiskComponent => "portfolio-risk-component",
            Self::PortfolioAsymmetryComponent => "portfolio-asymmetry-component",
            Self::Utility => "utility",
        }
    }

    pub fn default_shape(self) -> GraphNodeShape {
        let shape_name = match self {
            Self::Input => "cylinder",
            Self::State => "ellipse",
            Self::DecisionVariable => "diamond",
            Self::PortfolioComponent => "box",
            Self::PortfolioRiskComponent => "hexagon",
            Self::PortfolioAsymmetryComponent => "octagon",
            Self::Utility => "double_circle",
        };

        GraphNodeShape::default_for_name(shape_name).expect("node kind default shape must exist")
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Node<Tool = ()> {
    pub id: String,
    pub label: String,
    pub detail: String,
    pub kind: GraphNodeKind,
    pub shape: GraphNodeShape,
    pub active: bool,
    pub tool: Tool,
}

impl Node<()> {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        detail: impl Into<String>,
        kind: GraphNodeKind,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            detail: detail.into(),
            kind,
            shape: kind.default_shape(),
            active: true,
            tool: (),
        }
    }
}

impl<Tool> Node<Tool> {
    pub fn new_with_tool(
        id: impl Into<String>,
        label: impl Into<String>,
        detail: impl Into<String>,
        kind: GraphNodeKind,
        tool: Tool,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            detail: detail.into(),
            kind,
            shape: kind.default_shape(),
            active: true,
            tool,
        }
    }

    pub fn with_shape(mut self, shape: GraphNodeShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn with_tool<NextTool>(self, tool: NextTool) -> Node<NextTool> {
        Node {
            id: self.id,
            label: self.label,
            detail: self.detail,
            kind: self.kind,
            shape: self.shape,
            active: self.active,
            tool,
        }
    }

    pub fn tool(&self, args: impl ToolArgs) -> &Self
    where
        Tool: Fn(&dyn ToolArgs),
    {
        (self.tool)(&args);
        self
    }
}
