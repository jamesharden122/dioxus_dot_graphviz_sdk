use geo::{BoundingRect, Geometry, Point, Polygon, Rect, Triangle};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GraphNodeGeometry {
    pub shape: Geometry<f64>,
    pub row: usize,
    pub width: f64,
    pub height: f64,
}

impl GraphNodeGeometry {
    pub fn new(shape: Geometry<f64>, row: usize) -> Self {
        let (width, height) = shape
            .bounding_rect()
            .map(|bounds| (bounds.width(), bounds.height()))
            .unwrap_or((0.0, 0.0));

        Self {
            shape,
            row,
            width,
            height,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphNodeShape {
    Box(Rect<f64>),
    Polygon(Polygon<f64>),
    Ellipse(Polygon<f64>),
    Oval(Polygon<f64>),
    Circle(Polygon<f64>),
    Point(Point<f64>),
    Egg(Polygon<f64>),
    Triangle(Triangle<f64>),
    Plaintext(Rect<f64>),
    Plain(Rect<f64>),
    Diamond(Polygon<f64>),
    Trapezium(Polygon<f64>),
    Parallelogram(Polygon<f64>),
    House(Polygon<f64>),
    Pentagon(Polygon<f64>),
    Hexagon(Polygon<f64>),
    Septagon(Polygon<f64>),
    Octagon(Polygon<f64>),
    DoubleCircle(Polygon<f64>),
    DoubleOctagon(Polygon<f64>),
    TripleOctagon(Polygon<f64>),
    InvTriangle(Triangle<f64>),
    InvTrapezium(Polygon<f64>),
    InvHouse(Polygon<f64>),
    MDiamond(Polygon<f64>),
    MSquare(Rect<f64>),
    MCircle(Polygon<f64>),
    Rect(Rect<f64>),
    Rectangle(Rect<f64>),
    Square(Rect<f64>),
    Star(Polygon<f64>),
    None,
    Underline(Rect<f64>),
    Cylinder(Polygon<f64>),
    Note(Polygon<f64>),
    Tab(Polygon<f64>),
    Folder(Polygon<f64>),
    Box3d(Polygon<f64>),
    Component(Polygon<f64>),
    Promoter(Polygon<f64>),
    Cds(Polygon<f64>),
    Terminator(Polygon<f64>),
    Utr(Polygon<f64>),
    PrimerSite(Polygon<f64>),
    RestrictionSite(Polygon<f64>),
    FivePOverhang(Polygon<f64>),
    ThreePOverhang(Polygon<f64>),
    NOverhang(Polygon<f64>),
}

impl GraphNodeShape {
    pub fn geometry(&self) -> Option<GraphNodeGeometry> {
        self.geometry_for_row(0)
    }

    pub fn geometry_for_row(&self, row: usize) -> Option<GraphNodeGeometry> {
        match self {
            Self::Box(geometry)
            | Self::Plaintext(geometry)
            | Self::Plain(geometry)
            | Self::MSquare(geometry)
            | Self::Rect(geometry)
            | Self::Rectangle(geometry)
            | Self::Square(geometry)
            | Self::Underline(geometry) => {
                Some(GraphNodeGeometry::new(Geometry::Rect(*geometry), row))
            }
            Self::Polygon(geometry)
            | Self::Ellipse(geometry)
            | Self::Oval(geometry)
            | Self::Circle(geometry)
            | Self::Egg(geometry)
            | Self::Diamond(geometry)
            | Self::Trapezium(geometry)
            | Self::Parallelogram(geometry)
            | Self::House(geometry)
            | Self::Pentagon(geometry)
            | Self::Hexagon(geometry)
            | Self::Septagon(geometry)
            | Self::Octagon(geometry)
            | Self::DoubleCircle(geometry)
            | Self::DoubleOctagon(geometry)
            | Self::TripleOctagon(geometry)
            | Self::InvTrapezium(geometry)
            | Self::InvHouse(geometry)
            | Self::MDiamond(geometry)
            | Self::MCircle(geometry)
            | Self::Star(geometry)
            | Self::Cylinder(geometry)
            | Self::Note(geometry)
            | Self::Tab(geometry)
            | Self::Folder(geometry)
            | Self::Box3d(geometry)
            | Self::Component(geometry)
            | Self::Promoter(geometry)
            | Self::Cds(geometry)
            | Self::Terminator(geometry)
            | Self::Utr(geometry)
            | Self::PrimerSite(geometry)
            | Self::RestrictionSite(geometry)
            | Self::FivePOverhang(geometry)
            | Self::ThreePOverhang(geometry)
            | Self::NOverhang(geometry) => Some(GraphNodeGeometry::new(
                Geometry::Polygon(geometry.clone()),
                row,
            )),
            Self::Point(geometry) => Some(GraphNodeGeometry::new(Geometry::Point(*geometry), row)),
            Self::Triangle(geometry) | Self::InvTriangle(geometry) => {
                Some(GraphNodeGeometry::new(Geometry::Triangle(*geometry), row))
            }
            Self::None => None,
        }
    }

    pub fn dot_name(&self) -> &'static str {
        match self {
            Self::Box(_) => "box",
            Self::Polygon(_) => "polygon",
            Self::Ellipse(_) => "ellipse",
            Self::Oval(_) => "oval",
            Self::Circle(_) => "circle",
            Self::Point(_) => "point",
            Self::Egg(_) => "egg",
            Self::Triangle(_) => "triangle",
            Self::Plaintext(_) => "plaintext",
            Self::Plain(_) => "plain",
            Self::Diamond(_) => "diamond",
            Self::Trapezium(_) => "trapezium",
            Self::Parallelogram(_) => "parallelogram",
            Self::House(_) => "house",
            Self::Pentagon(_) => "pentagon",
            Self::Hexagon(_) => "hexagon",
            Self::Septagon(_) => "septagon",
            Self::Octagon(_) => "octagon",
            Self::DoubleCircle(_) => "doublecircle",
            Self::DoubleOctagon(_) => "doubleoctagon",
            Self::TripleOctagon(_) => "tripleoctagon",
            Self::InvTriangle(_) => "invtriangle",
            Self::InvTrapezium(_) => "invtrapezium",
            Self::InvHouse(_) => "invhouse",
            Self::MDiamond(_) => "Mdiamond",
            Self::MSquare(_) => "Msquare",
            Self::MCircle(_) => "Mcircle",
            Self::Rect(_) => "rect",
            Self::Rectangle(_) => "rectangle",
            Self::Square(_) => "square",
            Self::Star(_) => "star",
            Self::None => "none",
            Self::Underline(_) => "underline",
            Self::Cylinder(_) => "cylinder",
            Self::Note(_) => "note",
            Self::Tab(_) => "tab",
            Self::Folder(_) => "folder",
            Self::Box3d(_) => "box3d",
            Self::Component(_) => "component",
            Self::Promoter(_) => "promoter",
            Self::Cds(_) => "cds",
            Self::Terminator(_) => "terminator",
            Self::Utr(_) => "utr",
            Self::PrimerSite(_) => "primersite",
            Self::RestrictionSite(_) => "restrictionsite",
            Self::FivePOverhang(_) => "fivepoverhang",
            Self::ThreePOverhang(_) => "threepoverhang",
            Self::NOverhang(_) => "noverhang",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            Self::MDiamond(_) => "mdiamond",
            Self::MSquare(_) => "msquare",
            Self::MCircle(_) => "mcircle",
            _ => self.dot_name(),
        }
    }
}

impl Default for GraphNodeShape {
    fn default() -> Self {
        Self::Box(Rect::new((-0.5, -0.3), (0.5, 0.3)))
    }
}
