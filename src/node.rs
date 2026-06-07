use crate::shape::GraphNodeShape;
use geo::{Polygon, Rect};
use serde::{Deserialize, Serialize};

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
        match self {
            Self::Input => {
                GraphNodeShape::Cylinder(Rect::new((-0.5, -0.35), (0.5, 0.35)).to_polygon())
            }
            Self::State => {
                GraphNodeShape::Ellipse(Rect::new((-0.55, -0.35), (0.55, 0.35)).to_polygon())
            }
            Self::DecisionVariable => GraphNodeShape::Diamond(Polygon::new(
                vec![(0.0, 0.5), (0.5, 0.0), (0.0, -0.5), (-0.5, 0.0), (0.0, 0.5)].into(),
                vec![],
            )),
            Self::PortfolioComponent => GraphNodeShape::Box(Rect::new((-0.5, -0.3), (0.5, 0.3))),
            Self::PortfolioRiskComponent => GraphNodeShape::Hexagon(Polygon::new(
                vec![
                    (-0.5, 0.0),
                    (-0.25, 0.43),
                    (0.25, 0.43),
                    (0.5, 0.0),
                    (0.25, -0.43),
                    (-0.25, -0.43),
                    (-0.5, 0.0),
                ]
                .into(),
                vec![],
            )),
            Self::PortfolioAsymmetryComponent => GraphNodeShape::Octagon(Polygon::new(
                vec![
                    (-0.2, 0.5),
                    (0.2, 0.5),
                    (0.5, 0.2),
                    (0.5, -0.2),
                    (0.2, -0.5),
                    (-0.2, -0.5),
                    (-0.5, -0.2),
                    (-0.5, 0.2),
                    (-0.2, 0.5),
                ]
                .into(),
                vec![],
            )),
            Self::Utility => {
                GraphNodeShape::DoubleCircle(Rect::new((-0.5, -0.5), (0.5, 0.5)).to_polygon())
            }
        }
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
}
