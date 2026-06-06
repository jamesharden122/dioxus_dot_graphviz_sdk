#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub id: &'static str,
    pub from: &'static str,
    pub to: &'static str,
    pub kind: GraphEdgeKind,
    pub active: bool,
}

impl Edge {
    pub const fn new(
        id: &'static str,
        from: &'static str,
        to: &'static str,
        kind: GraphEdgeKind,
    ) -> Self {
        Self {
            id,
            from,
            to,
            kind,
            active: true,
        }
    }
}
