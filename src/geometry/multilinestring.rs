use crate::geometry::linestring::LineString;

//a representation of a multilinestring
pub struct MultiLineString {
    pub linestrings: Vec<LineString>
}