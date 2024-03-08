use crate::coordinate::Coordinate;
use crate::geometry::point::Point;
use crate::geometry::linestring::LineString;

pub mod point;
pub mod linestring;
pub mod polygon;
pub mod multipoint;
pub mod multilinestring;
pub mod multipolygon;
pub mod geometrycollection;


//set of possible geometries
pub enum Geometry {
    Point(Point),
    LineString(LineString),
    Polygon { exterior: Vec<Coordinate>, interior: Vec<Vec<Coordinate>>},
    MultiPoint(Vec<Point>),
    MultiLineString(Vec<LineString>),
    MultiPolygon { polygons: Vec<(Vec<Coordinate>, Vec<Vec<Coordinate>>)>},
    GeometryCollection { geometries: Vec<Geometry>}
}