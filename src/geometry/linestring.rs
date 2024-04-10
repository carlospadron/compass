use crate::coordinate::Coordinate;
use std::any::Any;
use super::Geometry;

//a representation of a linestring
#[derive(Debug)]
pub struct LineString {
    pub coordinates: Vec<Coordinate>
}

impl Geometry for LineString {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_simple(&self) -> bool {
        unimplemented!();
    }

    fn boundary(&self) -> &dyn Geometry {
        unimplemented!();
    }

    fn coordinates(&self) -> Vec<Coordinate> {
        unimplemented!()
    }

    fn dimension(&self) -> i32 {
        1
    }

    fn envelope(&self) -> &dyn Geometry {
        unimplemented!();
    }

    fn buffer(&self, distance: f64) -> &dyn Geometry {
        unimplemented!();
    }

    fn centroid(&self) -> super::point::Point {
        unimplemented!();
    }

    fn difference(&self, other: &dyn Geometry) -> &dyn Geometry {
        unimplemented!();
    }

    fn concave_hull(&self, tolerance: f64) -> &dyn Geometry {
        unimplemented!();
    }

    fn convex_hull(&self) -> &dyn Geometry {
        unimplemented!();
    }

    fn intersection(&self, other: &dyn Geometry) -> &dyn Geometry {
        unimplemented!();
    }

    fn reverse(&self) -> &dyn Geometry {
        unimplemented!();
    }

    fn simplify(&self, tolerance: f64) -> &dyn Geometry {
        unimplemented!();
    }

    fn sym_difference(&self, other: &dyn Geometry) -> &dyn Geometry {
        unimplemented!();
    }

    fn union(&self, other: &dyn Geometry) -> &dyn Geometry {
        unimplemented!();
    }

    fn normalize(&self) -> &dyn Geometry {
        unimplemented!();
    }

    fn snap(&self, other: &dyn Geometry, tolerance: f64) -> &dyn Geometry {
        unimplemented!();
    }

    fn snap_to_grid(&self, size: f64) -> &dyn Geometry {
        unimplemented!();
    }

    fn area(&self) -> f64 {
        unimplemented!();
    }

    fn distance(&self, other: &dyn Geometry) -> f64 {
        unimplemented!();
    }

    fn length(&self) -> f64 {
        unimplemented!();
    }

    fn is_within_distance(&self, other: &dyn Geometry, distance: f64) -> bool {
        unimplemented!();
    }

    fn set_srid(&self, srid: i32) -> &dyn Geometry {
        unimplemented!();
    }

    fn srid(&self) -> i32 {
        unimplemented!();
    }

    fn transform(&self, srid: i32) -> &dyn Geometry {
        unimplemented!();
    }

    fn contains(&self, other: &dyn Geometry) -> bool {
        unimplemented!();
    }

    fn covers(&self, other: &dyn Geometry) -> bool {
        unimplemented!();
    }

    fn covered_by(&self, other: &dyn Geometry) -> bool {
        unimplemented!();
    }

    fn crosses(&self, other: &dyn Geometry) -> bool {
        unimplemented!();
    }

    fn disjoint(&self, other: &dyn Geometry) -> bool {
        unimplemented!();
    }

    fn equals(&self, other: &dyn Geometry) -> bool {
        unimplemented!();
    }

    fn intersects(&self, other: &dyn Geometry) -> bool {
        unimplemented!();
    }

    fn overlaps(&self, other: &dyn Geometry) -> bool {
        unimplemented!();
    }

    fn relate(&self, other: &dyn Geometry, matrix: &str) -> bool {
        unimplemented!();
    }

    fn touches(&self, other: &dyn Geometry) -> bool {
        unimplemented!();
    }

    fn within(&self, other: &dyn Geometry) -> bool {
        unimplemented!();
    }

    fn is_valid(&self) -> bool {
        unimplemented!();
    }

    fn make_valid(&self) -> &dyn Geometry {
        unimplemented!();
    }

    fn as_text(&self) -> String {
        unimplemented!();
    }

    fn from_text(&self, wkt: &str) -> &dyn Geometry {
        unimplemented!();
    }

    fn as_binary(&self) -> Vec<u8> {
        unimplemented!();
    }

    fn from_binary(&self, wkb: &[u8]) -> &dyn Geometry {
        unimplemented!();
    }

    fn as_geojson(&self) -> String {
        unimplemented!();
    }

    fn from_geojson(&self, geojson: &str) -> &dyn Geometry {
        unimplemented!();
    }

    fn as_svg(&self) -> String {
        unimplemented!();
    }

    fn from_svg(&self, svg: &str) -> &dyn Geometry {
        unimplemented!();
    }

    fn as_kml(&self) -> String {
        unimplemented!();
    }

    fn from_kml(&self, kml: &str) -> &dyn Geometry {
        unimplemented!();
    }

    fn as_gml(&self) -> String {
        unimplemented!();
    }

    fn from_gml(&self, gml: &str) -> &dyn Geometry {
        unimplemented!();
    }

}
    