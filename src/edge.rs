use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphEdgeShape {
    #[default]
    Straight,
    Curved,
    Orthogonal,
    Stepped,
}

impl GraphEdgeShape {
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Straight => "straight",
            Self::Curved => "curved",
            Self::Orthogonal => "orthogonal",
            Self::Stepped => "stepped",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphEdgeKind {
    Data,
    Process,
    DecisionContribution,
    StateContribution,
    UtilityAggregation,
}

impl GraphEdgeKind {
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Data => "data",
            Self::Process => "process",
            Self::DecisionContribution => "decision",
            Self::StateContribution => "state",
            Self::UtilityAggregation => "utility",
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Edge<Tool = ()> {
    pub id: String,
    pub from: String,
    pub to: String,
    pub kind: GraphEdgeKind,
    pub active: bool,
    pub tool: Tool,
}

impl Edge<()> {
    pub fn new(
        id: impl Into<String>,
        from: impl Into<String>,
        to: impl Into<String>,
        kind: GraphEdgeKind,
    ) -> Self {
        Self {
            id: id.into(),
            from: from.into(),
            to: to.into(),
            kind,
            active: true,
            tool: (),
        }
    }
}

impl<Tool> Edge<Tool> {
    pub fn new_with_tool(
        id: impl Into<String>,
        from: impl Into<String>,
        to: impl Into<String>,
        kind: GraphEdgeKind,
        tool: Tool,
    ) -> Self {
        Self {
            id: id.into(),
            from: from.into(),
            to: to.into(),
            kind,
            active: true,
            tool,
        }
    }

    pub fn with_tool<NextTool>(self, tool: NextTool) -> Edge<NextTool> {
        Edge {
            id: self.id,
            from: self.from,
            to: self.to,
            kind: self.kind,
            active: self.active,
            tool,
        }
    }
}
