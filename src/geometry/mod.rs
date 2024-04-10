use crate::coordinate::Coordinate;
use crate::geometry::point::Point;
use std::any::Any;
use std::fmt::Debug;

pub mod point;
pub mod linestring;
pub mod linearring;
pub mod polygon;
pub mod multipoint;
pub mod multilinestring;
pub mod multipolygon;
pub mod geometrycollection;


//set of possible geometries
pub trait Geometry: Debug {
    //type methods
    fn as_any(&self) -> &dyn Any;
    //predicates
    fn is_simple(&self) -> bool;
    //accessors
    fn boundary(&self) -> &dyn Geometry;
    fn coordinates(&self) -> Vec<Coordinate>;
    fn dimension(&self) -> i32;
    fn envelope(&self) -> &dyn Geometry;
    //constructive methods
    fn buffer(&self, distance: f64) -> &dyn Geometry;
    fn centroid(&self) -> Point;
    fn difference(&self, other: &dyn Geometry) -> &dyn Geometry;
    fn concave_hull(&self, tolerance: f64) -> &dyn Geometry;
    fn convex_hull(&self) -> &dyn Geometry;
    fn intersection(&self, other: &dyn Geometry) -> &dyn Geometry;
    fn reverse(&self) -> &dyn Geometry;
    fn simplify(&self, tolerance: f64) -> &dyn Geometry;    
    fn sym_difference(&self, other: &dyn Geometry) -> &dyn Geometry;    
    fn union(&self, other: &dyn Geometry) -> &dyn Geometry;
    //editorial methods
    fn normalize(&self) -> &dyn Geometry;
    fn snap(&self, other: &dyn Geometry, tolerance: f64) -> &dyn Geometry;
    fn snap_to_grid(&self, size: f64) -> &dyn Geometry;
    //measuring methods
    fn area(&self) -> f64;
    fn distance(&self, other: &dyn Geometry) -> f64;
    fn length(&self) -> f64;      
    //distance relationships
    fn is_within_distance(&self, other: &dyn Geometry, distance: f64) -> bool;
    //spatial reference system methods
    fn set_srid(&self, srid: i32) -> &dyn Geometry;
    fn srid(&self) -> i32;
    fn transform(&self, srid: i32) -> &dyn Geometry;
    //topological relationships
    fn contains(&self, other: &dyn Geometry) -> bool;
    fn covers(&self, other: &dyn Geometry) -> bool;
    fn covered_by(&self, other: &dyn Geometry) -> bool;
    fn crosses(&self, other: &dyn Geometry) -> bool;
    fn disjoint(&self, other: &dyn Geometry) -> bool;
    fn equals(&self, other: &dyn Geometry) -> bool;
    fn intersects(&self, other: &dyn Geometry) -> bool;
    fn overlaps(&self, other: &dyn Geometry) -> bool;
    fn relate(&self, other: &dyn Geometry, matrix: &str) -> bool;
    fn touches(&self, other: &dyn Geometry) -> bool;
    fn within(&self, other: &dyn Geometry) -> bool;
    //validation methods    
    fn is_valid(&self) -> bool;
    fn make_valid(&self) -> &dyn Geometry;
    //wkt methods
    fn as_text(&self) -> String;
    fn from_text(&self, wkt: &str) -> &dyn Geometry;
    //wkb methods
    fn as_binary(&self) -> Vec<u8>;
    fn from_binary(&self, wkb: &[u8]) -> &dyn Geometry;
    //geojson methods
    fn as_geojson(&self) -> String;
    fn from_geojson(&self, geojson: &str) -> &dyn Geometry;
    //svg methods
    fn as_svg(&self) -> String;
    fn from_svg(&self, svg: &str) -> &dyn Geometry;
    //kml methods
    fn as_kml(&self) -> String;
    fn from_kml(&self, kml: &str) -> &dyn Geometry;
    //gml methods
    fn as_gml(&self) -> String;
    fn from_gml(&self, gml: &str) -> &dyn Geometry;








} 

/// Implement PartialEq for dyn Geometry
/// TODO add examples
impl PartialEq for dyn Geometry {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}
