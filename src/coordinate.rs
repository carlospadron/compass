use std::cmp::PartialEq;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Represents a coordinate in 3D space.

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    /// The x value of the coordinate.
    x: f64,
    /// The y value of the coordinate.
    y: f64,
    /// The z value of the coordinate.
    z: f64
}

impl Coordinate {
    /// Creates a new coordinate.
    ///
    /// # Arguments
    ///
    /// * `x` - The x value of the coordinate.
    /// * `y` - The y value of the coordinate.
    /// * `z` - The z value of the coordinate.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use geoms::coordinate::Coordinate;
    /// 
    /// Coordinate::new(3.0, f64::NAN, 5.0);
    /// ```
    ///
    /// ```should_panic
    /// use geoms::coordinate::Coordinate;
    /// 
    /// Coordinate::new(3.0, f64::INFINITY, 5.0);
    /// ```    
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    /// 
    /// Coordinate::new(3.0, 7.0, 5.0);
    /// ```    
    
    pub fn new(x: f64, y: f64, z: f64) -> Coordinate {
        if x.is_nan() || y.is_nan() || z.is_nan() {
            panic!("Coordinate values cannot be NaN");
        }
        else if x.is_infinite() || y.is_infinite() || z.is_infinite() {
            panic!("Coordinate values cannot be infinite");
            
        }
        Coordinate { x, y, z }
    }

    /// Returns the x value of the coordinate.
    ///
    /// # Arguments
    ///
    /// * `self` - The coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coordinate = Coordinate::new(3.0, 4.0, 5.0);
    /// assert_eq!(coordinate.x(), 3.0);
    /// ```
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns the y value of the coordinate.
    ///
    /// # Arguments
    ///
    /// * `self` - The coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coordinate = Coordinate::new(3.0, 4.0, 5.0);
    /// assert_eq!(coordinate.y(), 4.0);
    /// ```
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns the z value of the coordinate.
    /// If the coordinate is 2D, it returns 0.0.
    ///
    /// # Arguments
    ///
    /// * `self` - The coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coordinate = Coordinate::new(3.0, 4.0, 5.0);
    /// assert_eq!(coordinate.z(), 5.0);
    /// ```
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Returns the ordinate value of the coordinate.
    ///
    /// # Arguments
    ///
    /// * `self` - The coordinate.
    /// * `ordinate` - The index of the ordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    ///
    /// let coordinate = Coordinate::new(3.0, 4.0, 5.0);
    /// assert_eq!(coordinate.get_ordinate(0), 3.0);
    /// assert_eq!(coordinate.get_ordinate(1), 4.0);
    /// assert_eq!(coordinate.get_ordinate(2), 5.0);
    /// ```
    pub fn get_ordinate(&self, ordinate: usize) -> f64 {
        match ordinate {
            0 => self.x(),
            1 => self.y(),
            2 => self.z(),
            _ => panic!("Invalid ordinate index")
        }
    }

    /// Returns a new coordinate with the updated x value.
    ///
    /// # Arguments
    ///
    /// * `self` - The coordinate.
    /// * `new_x` - The new x value.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coordinate = Coordinate::new(3.0, 4.0, 5.0);
    /// let new_coordinate = coordinate.set_x(6.0);
    /// 
    /// assert_eq!(new_coordinate.x(), 6.0);
    /// assert_eq!(new_coordinate.y(), 4.0);
    /// assert_eq!(new_coordinate.z(), 5.0);
    /// ```
    pub fn set_x(&self, new_x: f64) -> Coordinate {
        Coordinate::new(new_x, self.y(), self.z())
    }
    
    /// Returns a new coordinate with the updated y value.
    ///
    /// # Arguments
    ///
    /// * `self` - The coordinate.
    /// * `new_y` - The new y value.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coordinate = Coordinate::new(3.0, 4.0, 5.0);
    /// let new_coordinate = coordinate.set_y(6.0);
    /// assert_eq!(new_coordinate.x(), 3.0);
    /// assert_eq!(new_coordinate.y(), 6.0);
    /// assert_eq!(new_coordinate.z(), 5.0);
    /// ```
    pub fn set_y(&self, new_y: f64) -> Coordinate {
        Coordinate::new(self.x(), new_y, self.z())
    }
    
    /// Returns a new coordinate with the updated z value.
    ///
    /// # Arguments
    ///
    /// * `self` - The coordinate.
    /// * `new_z` - The new z value.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coordinate = Coordinate::new(3.0, 4.0, 5.0);
    /// let new_coordinate = coordinate.set_z(4.0);
    /// assert_eq!(new_coordinate.x(), 3.0);
    /// assert_eq!(new_coordinate.y(), 4.0);
    /// assert_eq!(new_coordinate.z(), 4.0);    
    /// ```
    pub fn set_z(&self, new_z: f64) -> Coordinate {
        Coordinate::new(self.x(), self.y(), new_z)
    }

    /// Returns a new coordinate with the updated ordinate value.
    ///
    /// # Arguments
    ///
    /// * `self` - The coordinate.
    /// * `ordinate` - The index of the ordinate.
    /// * `new_value` - The new value of the ordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coordinate = Coordinate::new(3.0, 4.0, 5.0);
    /// let new_coordinate = coordinate.set_ordinate(2, 6.0);
    /// assert_eq!(new_coordinate.x(), 3.0);
    /// assert_eq!(new_coordinate.y(), 4.0);
    /// assert_eq!(new_coordinate.z(), 6.0);
    /// ```
    pub fn set_ordinate(&self, ordinate: usize, new_value: f64) -> Coordinate {
        match ordinate {
            0 => self.set_x(new_value),
            1 => self.set_y(new_value),
            2 => self.set_z(new_value),
            _ => panic!("Invalid ordinate index")
        }
    }    

    /// Returns whether the planar projections of the two coordinates are equal.
    ///
    /// # Arguments
    ///
    /// * `self` - The first coordinate.
    /// * `other` - The second coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coordinate1 = Coordinate::new(1.0, 2.0, 5.0);
    /// let coordinate2 = Coordinate::new(1.0, 2.0, 3.0);
    /// assert!(coordinate1.equals_2d(&coordinate2));
    ///
    /// let coordinate1 = Coordinate::new(1.0, 2.0, 3.0);
    /// let coordinate2 = Coordinate::new(3.0, 4.0, 5.0);
    /// assert!(!coordinate1.equals_2d(&coordinate2));
    /// ```
    pub fn equals_2d(&self, other: &Coordinate) -> bool {
        self.x() == other.x() && self.y() == other.y()
    }
    
    /// Returns whether the planar projections of the two coordinates are equal within a given tolerance.
    ///
    /// # Arguments
    ///
    /// * `self` - The first coordinate.
    /// * `other` - The second coordinate.
    /// * `tolerance` - The maximum allowed difference between the coordinates' values.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coordinate1 = Coordinate::new(1.0, 2.0, 5.0);
    /// let coordinate2 = Coordinate::new(1.0, 2.02, 3.0);
    /// assert!(coordinate1.equals_2d_with_tolerance(&coordinate2, 0.1));
    ///
    /// let coordinate1 = Coordinate::new(1.0, 2.0, 5.0);
    /// let coordinate2 = Coordinate::new(3.0, 4.0, 5.0);
    /// assert!(!coordinate1.equals_2d_with_tolerance(&coordinate2, 0.1));
    /// ```
    pub fn equals_2d_with_tolerance(&self, other: &Coordinate, tolerance: f64) -> bool {
        (self.x() - other.x()).abs() - tolerance < f64::EPSILON
        && (self.y() - other.y()).abs() - tolerance < f64::EPSILON
    }

    /// Checks if the coordinates are equal within a given tolerance.
    ///
    /// # Arguments
    ///
    /// * `self` - The first coordinate.
    /// * `other` - The second coordinate.
    /// * `tolerance` - The maximum allowed difference between the coordinates' values.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coordinate1 = Coordinate::new(1.0, 2.0, 3.0);
    /// let coordinate2 = Coordinate::new(1.0, 2.02, 3.02);
    /// assert!(coordinate1.equals_with_tolerance(&coordinate2, 0.1));
    ///
    /// let coordinate1 = Coordinate::new(1.0, 2.0, 3.0);
    /// let coordinate2 = Coordinate::new(3.0, 4.0, 5.0);
    /// assert!(!coordinate1.equals_with_tolerance(&coordinate2, 0.1));
    /// ```
    pub fn equals_with_tolerance(&self, other: &Coordinate, tolerance: f64) -> bool {
        self.equals_2d_with_tolerance(other, tolerance) 
        && (self.z() - other.z()).abs() - tolerance < f64::EPSILON
    }

    /// Checks if the z values of the coordinates are equal within a given tolerance.
    ///
    /// # Arguments
    ///
    /// * `self` - The first coordinate.
    /// * `other` - The second coordinate.
    /// * `tolerance` - The maximum allowed difference between the coordinates' z values.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coordinate1 = Coordinate::new(1.0, 2.0, 3.0);
    /// let coordinate2 = Coordinate::new(1.0, 2.0, 3.1);
    /// assert!(coordinate1.equals_in_z_with_tolerance(&coordinate2, 0.1));
    ///
    /// let coordinate1 = Coordinate::new(1.0, 2.0, 3.0);
    /// let coordinate2 = Coordinate::new(1.0, 2.0, 4.0);
    /// assert!(!coordinate1.equals_in_z_with_tolerance(&coordinate2, 0.1));
    /// ```
    pub fn equals_in_z_with_tolerance(&self, other: &Coordinate, tolerance: f64) -> bool {
        (self.z() - other.z()).abs() - tolerance < f64::EPSILON
    }

 }

 impl fmt::Display for Coordinate {
    /// Returns the string representation of the coordinate.
    ///
    /// # Arguments
    ///
    /// * `self` - The coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coordinate = Coordinate::new(1.0, 2.0, 3.0);
    /// assert_eq!(coordinate.to_string(), "(1, 2, 3)");
    /// ```

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

/// Implements the equality operator for the Coordinate struct.
/// As coordinate does not allow NaN or infinite values, it is safe to compare the coordinates directly.
impl Eq for Coordinate {}

/// Implements the hash trait for the Coordinate struct.
impl Hash for Coordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
    }
}

/// A macro for creating coordinate objects.
///
/// This macro allows you to create coordinate objects in two or three dimensions.
/// It takes in the x, y, and z values and returns a coordinate object.
///
/// # Examples
///
/// ```
/// use geoms::{coord, coordinate::{Coordinate}};
/// let coord = coord!(10, 20.0);
/// assert_eq!(coord, Coordinate::new(10.0, 20.0, 0.0));
///
/// let coord = coord!(10.0, 20, 30.0 );
/// assert_eq!(coord, Coordinate::new(10.0, 20.0, 30.0));
/// ```

#[macro_export]
macro_rules! coord {    
    ( $x:expr, $y:expr ) => {
        Coordinate::new($x as f64, $y as f64, 0.0)
    };
    ( $x:expr, $y:expr, $z:expr ) => {
        Coordinate::new($x as f64, $y as f64, $z as f64)
    };
}
 