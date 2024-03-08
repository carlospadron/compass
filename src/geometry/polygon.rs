use crate::geometry::linestring::LineString;
use crate::geometry::multilinestring::MultiLineString;

//a representation of a polygon
pub struct Polygon {
    pub exterior: LineString,
    pub interior: MultiLineString
}