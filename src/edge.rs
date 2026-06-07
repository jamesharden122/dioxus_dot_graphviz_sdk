use serde::{Deserialize, Serialize};

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
pub struct Edge {
    pub id: String,
    pub from: String,
    pub to: String,
    pub kind: GraphEdgeKind,
    pub active: bool,
}

impl Edge {
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
        }
    }
}
