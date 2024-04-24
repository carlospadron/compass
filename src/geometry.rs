use std::collections::HashSet;

use crate::coordinate::Coordinate;

pub enum Geometry {
    Point { coordinates: Coordinate },
    LineString { coordinates: Vec<Coordinate> },
    LinearRing { coordinates: Vec<Coordinate> },
    Polygon { coordinates: Vec<Vec<Coordinate>> },
    MultiPoint { coordinates: Vec<Coordinate> },
    MultiLineString { coordinates: Vec<Vec<Coordinate>> },
    MultiPolygon { coordinates: Vec<Vec<Vec<Coordinate>>> },
    GeometryCollection { geometries: Vec<Geometry> },    
}

//set of possible geometries
impl Geometry {
    //predicates

    /// Returns true if the geometry is a simple geometry
    /// A simple geometry is one that has no anomalous geometric points, such as self intersection or self tangency.
    ///
    /// # Examples
    /// ```
    /// use geoms::geometry::Geometry;
    /// use geoms::coordinate::Coordinate;
    /// use geoms::coord;
    /// 
    /// let point = Geometry::Point { coordinates: coord!(0, 0) };
    /// assert!(point.is_simple());
    /// 
    /// let line = Geometry::LineString { coordinates: vec![coord!(0, 0), coord!(1, 1), coord!(2, 2)] };
    /// assert!(line.is_simple());
    /// 
    /// let line = Geometry::LineString { coordinates: vec![coord!(0, 0), coord!(1, 1), coord!(0, 0)] };
    /// assert!(line.is_simple());
    /// 
    /// let line = Geometry::LineString { coordinates: vec![coord!(0, 0), coord!(1, 1), coord!(1, 1)] };
    /// assert!(!line.is_simple());
    /// 
    /// let line = Geometry::LineString { coordinates: vec![coord!(1, 1), coord!(1, 1), coord!(0, 0)] };
    /// assert!(!line.is_simple());
    /// 
    ///  //very very long line with no self intersection
    /// let coordinates: Vec<Coordinate> = (0..=100_000_000).map(|i| coord!(i, i)).collect();
    /// let line = Geometry::LineString { coordinates };
    /// assert!(line.is_simple());
    /// ```
    pub fn is_simple(&self) -> bool {
    
        //helper function that checks if a set of coordinates are simple
        fn is_simple_coordinates(coordinates: &Vec<Coordinate>) -> bool {
            let mut coords = coordinates.iter();
            let initial = coords.next().unwrap();
            let mut set: HashSet<&Coordinate> = HashSet::new();
            let mut initial_state = true;
            let mut state = true;

            //check if it is contained in the set, if it is, state is false
            loop {
                match coords.next() {
                    Some(coordinate) => {
                        //if the previous state was false, and it is not the end of the loop, return false
                        if !state || !initial_state { 
                            break false;
                        }
                        //check against the initial coordinate
                        else if initial == coordinate {
                            initial_state = false;
                        }                       
                        //add the coordinate to the set,if it is already in the set, state is false
                        else if !set.insert(coordinate) {
                            state = false;
                        }
                    },
                    //if it reaches the end, and state is false, check if the first and last coordinates are the same
                    None => {
                        if !state {
                            break false;
                        } else {
                            break true;
                        }
                    }           
                }
            }

        }

        match self {
            //points are always simple
            Geometry::Point { .. } => true,
            //lines are simple if they have no self intersections. 
            Geometry::LineString { coordinates } => is_simple_coordinates(&coordinates),
            //Meaning, the only duplicate coordinates are the first and last.
            _ => false,
        }

    }
    // //accessors
    // fn boundary(&self) -> &dyn Geometry;
    // fn coordinates(&self) -> Vec<Coordinate>; //this might not be needed as every type has a different construct of coordinates
    // fn dimension(&self) -> i32;
    // fn envelope(&self) -> &dyn Geometry;
    // //constructive methods
    // fn buffer(&self, distance: f64) -> &dyn Geometry;
    // fn centroid(&self) -> Point;
    // fn difference(&self, other: &dyn Geometry) -> &dyn Geometry;
    // fn concave_hull(&self, tolerance: f64) -> &dyn Geometry;
    // fn convex_hull(&self) -> &dyn Geometry;
    // fn intersection(&self, other: &dyn Geometry) -> &dyn Geometry;
    // fn reverse(&self) -> &dyn Geometry;
    // fn simplify(&self, tolerance: f64) -> &dyn Geometry;    
    // fn sym_difference(&self, other: &dyn Geometry) -> &dyn Geometry;    
    // fn union(&self, other: &dyn Geometry) -> &dyn Geometry;
    // //editorial methods
    // fn normalize(&self) -> &dyn Geometry;
    // fn snap(&self, other: &dyn Geometry, tolerance: f64) -> &dyn Geometry;
    // fn snap_to_grid(&self, size: f64) -> &dyn Geometry;
    // //measuring methods
    // fn area(&self) -> f64;
    // fn distance(&self, other: &dyn Geometry) -> f64;
    // fn length(&self) -> f64;      
    // //distance relationships
    // fn is_within_distance(&self, other: &dyn Geometry, distance: f64) -> bool;
    // //spatial reference system methods
    // fn set_srid(&self, srid: i32) -> &dyn Geometry;
    // fn srid(&self) -> i32;
    // fn transform(&self, srid: i32) -> &dyn Geometry;
    // //topological relationships
    // fn contains(&self, other: &dyn Geometry) -> bool;
    // fn covers(&self, other: &dyn Geometry) -> bool;
    // fn covered_by(&self, other: &dyn Geometry) -> bool;
    // fn crosses(&self, other: &dyn Geometry) -> bool;
    // fn disjoint(&self, other: &dyn Geometry) -> bool;
    // fn equals(&self, other: &dyn Geometry) -> bool;
    // fn intersects(&self, other: &dyn Geometry) -> bool;
    // fn overlaps(&self, other: &dyn Geometry) -> bool;
    // fn relate(&self, other: &dyn Geometry, matrix: &str) -> bool;
    // fn touches(&self, other: &dyn Geometry) -> bool;
    // fn within(&self, other: &dyn Geometry) -> bool;
    // //validation methods    
    // fn is_valid(&self) -> bool;
    // fn make_valid(&self) -> &dyn Geometry;
    // //wkt methods
    // fn as_text(&self) -> String;
    // fn from_text(&self, wkt: &str) -> &dyn Geometry;
    // //wkb methods
    // fn as_binary(&self) -> Vec<u8>;
    // fn from_binary(&self, wkb: &[u8]) -> &dyn Geometry;
    // //geojson methods
    // fn as_geojson(&self) -> String;
    // fn from_geojson(&self, geojson: &str) -> &dyn Geometry;
    // //svg methods
    // fn as_svg(&self) -> String;
    // fn from_svg(&self, svg: &str) -> &dyn Geometry;
    // //kml methods
    // fn as_kml(&self) -> String;
    // fn from_kml(&self, kml: &str) -> &dyn Geometry;
    // //gml methods
    // fn as_gml(&self) -> String;
    // fn from_gml(&self, gml: &str) -> &dyn Geometry;
}

//tests
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::coord;

    #[test]
    pub fn test_is_simple() {
        let point = Geometry::Point { coordinates: coord!(0, 0) };
        assert!(point.is_simple());

        //no self intersections
        let line = Geometry::LineString { coordinates: vec![coord!(0, 0), coord!(1, 1), coord!(2, 2)] };
        assert!(line.is_simple());

        //no self intersections, closed line
        let line = Geometry::LineString { coordinates: vec![coord!(0, 0), coord!(1, 1), coord!(0, 0)] };
        assert!(line.is_simple());

        //self intersection at the end
        let line = Geometry::LineString { coordinates: vec![coord!(0, 0), coord!(1, 1), coord!(1, 1)] };
        assert!(!line.is_simple());

        //self intersection at the beginning
        let line = Geometry::LineString { coordinates: vec![coord!(1, 1), coord!(1, 1), coord!(0, 0)] };
        assert!(!line.is_simple());

        //very very long line with no self intersection
        let coordinates: Vec<Coordinate> = (0..=10_000_000).map(|i| coord!(i, i)).collect();
        let line = Geometry::LineString { coordinates };
        assert!(line.is_simple());


    }
}