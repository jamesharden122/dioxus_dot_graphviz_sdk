#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub id: &'static str,
    pub label: &'static str,
    pub detail: &'static str,
    pub kind: GraphNodeKind,
    pub active: bool,
}

impl Node {
    pub const fn new(
        id: &'static str,
        label: &'static str,
        detail: &'static str,
        kind: GraphNodeKind,
    ) -> Self {
        Self {
            id,
            label,
            detail,
            kind,
            active: true,
        }
    }
}
