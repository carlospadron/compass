use crate::coordinate::Coordinate;
use crate::geometry::point::Point;

pub mod point;
pub mod linestring;
pub mod linearring;
pub mod polygon;
pub mod multipoint;
pub mod multilinestring;
pub mod multipolygon;
pub mod geometrycollection;


//set of possible geometries
pub trait Geometry {
    //predicates
    fn is_simple(&self) -> bool;
    fn is_valid(&self) -> bool;
    //accessors
    fn area(&self) -> f64;
    fn boundary(&self) -> &dyn Geometry;
    fn coordinates(&self) -> Vec<Coordinate>;
    fn dimension(&self) -> i32;
    fn envelope(&self) -> &dyn Geometry;
    fn length(&self) -> f64;       
    fn srid(&self) -> i32;
    //constructive methods
    fn buffer(&self, distance: f64) -> &dyn Geometry;
    fn centroid(&self) -> Point;
    fn difference(&self, other: &dyn Geometry) -> &dyn Geometry;
    fn convex_hull(&self) -> &dyn Geometry;
    fn intersection(&self, other: &dyn Geometry) -> &dyn Geometry;
    fn reverse(&self) -> &dyn Geometry;
    fn simplify(&self, tolerance: f64) -> &dyn Geometry;    
    fn sym_difference(&self, other: &dyn Geometry) -> &dyn Geometry;    
    fn union(&self, other: &dyn Geometry) -> &dyn Geometry;
    //measuring methods
    fn distance(&self, other: &dyn Geometry) -> f64;
    //distance relationships
    fn is_within_distance(&self, other: &dyn Geometry, distance: f64) -> bool;
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




} 

