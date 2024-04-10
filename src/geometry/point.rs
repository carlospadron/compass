use crate::coordinate::Coordinate;
use super::Geometry;
use std::any::Any;

//a representation of a point
#[derive(Debug)]
pub struct Point {
    pub coordinate: Coordinate
}

impl Geometry for Point {
    /// Returns a reference to the object as an `Any` type
    /// This is used to downcast the object to a concrete type.
    /// Useful for matching on different geometry types.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use geoms::geometry::point::Point;
    /// use geoms::coordinate::Coordinate;
    /// use geoms::geometry::Geometry;
    /// use geoms::coord;
    /// use std::any::Any;
    /// 
    /// let point = Point { coordinate: coord!(10, 20) };
    /// let any = point.as_any();
    /// 
    /// assert!(any.is::<Point>());
    /// ```
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Returns true if the point is simple.
    /// A geometry is simple if it has no anomalous geometric points, such as self intersection or self tangency.
    /// A point is always simple.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use geoms::geometry::point::Point;
    /// use geoms::coordinate::Coordinate;
    /// use geoms::geometry::Geometry;
    /// 
    /// use geoms::coord;
    /// let point = Point { coordinate: coord!(10, 20) };
    /// assert!(point.is_simple());
    /// ```
    fn is_simple(&self) -> bool {
        true
    }

    /// Returns the boundary of the point which is itself
    /// 
    /// # Examples
    /// 
    /// ```
    /// use geoms::geometry::point::Point;
    /// use geoms::coordinate::Coordinate;
    /// use geoms::geometry::Geometry;
    /// 
    /// use geoms::coord;
    /// let point = Point { coordinate: coord!(10, 20) };
    /// let boundary = point.boundary();
    /// assert_eq!(point, boundary);
    /// ```
    fn boundary(&self) -> &dyn Geometry {
        self
    }

    fn coordinates(&self) -> Vec<Coordinate> {
        unimplemented!();
    }
    fn dimension(&self) -> i32 {
        unimplemented!();
    }

    fn envelope(&self) -> &dyn Geometry {
        unimplemented!();
    }

    fn buffer(&self, distance: f64) -> &dyn Geometry {
        unimplemented!();
    }

    fn centroid(&self) -> Point {
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
        //for geometry types different to point, return false, for point return true if coordinates are equal
        match other.as_any().downcast_ref::<Point>() {
            Some(point) => {
                self.coordinate == point.coordinate
            },
            None => false
        }
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
