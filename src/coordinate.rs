/// Represents a coordinate in either 2D or 3D space.
#[derive(Debug)]
pub enum Coordinate {
    /// Represents a 2D coordinate with x and y values.
    TwoDim { x: f64, y: f64 },
    /// Represents a 3D coordinate with x, y, and z values.
    ThreeDim { x: f64, y: f64, z: f64 }
}

impl Coordinate {

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
    /// let coordinate = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// assert_eq!(coordinate.x(), 1.0);
    ///
    /// let coordinate = Coordinate::ThreeDim { x: 3.0, y: 4.0, z: 5.0 };
    /// assert_eq!(coordinate.x(), 3.0);
    /// ```
    pub fn x(&self) -> f64 {
        match self {
            Coordinate::TwoDim { x, .. } => *x,
            Coordinate::ThreeDim { x, .. } => *x
        }
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
    /// let coordinate = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// assert_eq!(coordinate.y(), 2.0);
    ///
    /// let coordinate = Coordinate::ThreeDim { x: 3.0, y: 4.0, z: 5.0 };
    /// assert_eq!(coordinate.y(), 4.0);
    /// ```
    pub fn y(&self) -> f64 {
        match self {
            Coordinate::TwoDim { y, .. } => *y,
            Coordinate::ThreeDim { y, .. } => *y
        }
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
    /// let coordinate = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// assert_eq!(coordinate.z(), 0.0);
    ///
    /// let coordinate = Coordinate::ThreeDim { x: 3.0, y: 4.0, z: 5.0 };
    /// assert_eq!(coordinate.z(), 5.0);
    /// ```
    pub fn z(&self) -> f64 {
        match self {
            Coordinate::ThreeDim { z, .. } => *z,
            _ => 0.0
        }
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
    /// let coordinate = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// assert_eq!(coordinate.get_ordinate(0), 1.0);
    /// assert_eq!(coordinate.get_ordinate(1), 2.0);
    ///
    /// let coordinate = Coordinate::ThreeDim { x: 3.0, y: 4.0, z: 5.0 };
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
    /// let coordinate = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// let new_coordinate = coordinate.set_x(3.0);
    /// assert_eq!(new_coordinate.x(), 3.0);
    /// assert_eq!(new_coordinate.y(), 2.0);
    ///
    /// let coordinate = Coordinate::ThreeDim { x: 3.0, y: 4.0, z: 5.0 };
    /// let new_coordinate = coordinate.set_x(6.0);
    /// assert_eq!(new_coordinate.x(), 6.0);
    /// assert_eq!(new_coordinate.y(), 4.0);
    /// assert_eq!(new_coordinate.z(), 5.0);
    /// ```
    pub fn set_x(&self, new_x: f64) -> Coordinate {
        match self {
            Coordinate::TwoDim { y, .. } => Coordinate::TwoDim { x: new_x, y: *y },
            Coordinate::ThreeDim { y, z, .. } => Coordinate::ThreeDim { x: new_x, y: *y, z: *z }
        }
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
    /// let coordinate = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// let new_coordinate = coordinate.set_y(3.0);
    /// assert_eq!(new_coordinate.x(), 1.0);
    /// assert_eq!(new_coordinate.y(), 3.0);
    ///
    /// let coordinate = Coordinate::ThreeDim { x: 3.0, y: 4.0, z: 5.0 };
    /// let new_coordinate = coordinate.set_y(6.0);
    /// assert_eq!(new_coordinate.x(), 3.0);
    /// assert_eq!(new_coordinate.y(), 6.0);
    /// assert_eq!(new_coordinate.z(), 5.0);
    /// ```
    pub fn set_y(&self, new_y: f64) -> Coordinate {
        match self {
            Coordinate::TwoDim { x, .. } => Coordinate::TwoDim { x: *x, y: new_y },
            Coordinate::ThreeDim { x, z, .. } => Coordinate::ThreeDim { x: *x, y: new_y, z: *z }
        }
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
    /// let coordinate = Coordinate::ThreeDim { x: 1.0, y: 2.0, z: 3.0 };
    /// let new_coordinate = coordinate.set_z(4.0);
    /// assert_eq!(new_coordinate.x(), 1.0);
    /// assert_eq!(new_coordinate.y(), 2.0);
    /// assert_eq!(new_coordinate.z(), 4.0);
    ///
    /// let coordinate = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// let new_coordinate = coordinate.set_z(4.0);
    /// assert_eq!(new_coordinate.x(), 1.0);
    /// assert_eq!(new_coordinate.y(), 2.0);
    /// assert_eq!(new_coordinate.z(), 4.0);    
    /// ```
    pub fn set_z(&self, new_z: f64) -> Coordinate {
        match self {
            Coordinate::TwoDim { x, y } => Coordinate::ThreeDim { x: *x, y: *y, z: new_z },
            Coordinate::ThreeDim { x, y, .. } => Coordinate::ThreeDim { x: *x, y: *y, z: new_z },
        }
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
    /// let coordinate = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// let new_coordinate = coordinate.set_ordinate(0, 3.0);
    /// assert_eq!(new_coordinate.x(), 3.0);
    /// assert_eq!(new_coordinate.y(), 2.0);
    ///
    /// let coordinate = Coordinate::ThreeDim { x: 3.0, y: 4.0, z: 5.0 };
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

    /// Checks if the coordinate values are valid, meaning they are finite.
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
    /// let coordinate = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// assert!(coordinate.is_valid());
    ///
    /// let coordinate = Coordinate::ThreeDim { x: 3.0, y: 4.0, z: 5.0 };
    /// assert!(coordinate.is_valid());
    ///
    /// let coordinate = Coordinate::TwoDim { x: f64::INFINITY, y: 2.0 };
    /// assert!(!coordinate.is_valid());
    /// 
    /// let coordinate = Coordinate::ThreeDim { x: 3.0, y: f64::NAN, z: 5.0 };
    /// assert!(!coordinate.is_valid());    
    /// ```
    pub fn is_valid(&self) -> bool {
        match self {
            Coordinate::TwoDim { x, y } => x.is_finite() && y.is_finite(),
            Coordinate::ThreeDim { x, y, z } => x.is_finite() && y.is_finite() && z.is_finite()
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
    /// let coordinate1 = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// let coordinate2 = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// assert!(coordinate1.equals_2d(&coordinate2));
    ///
    /// let coordinate1 = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// let coordinate2 = Coordinate::TwoDim { x: 3.0, y: 4.0 };
    /// assert!(!coordinate1.equals_2d(&coordinate2));
    ///
    /// let coordinate1 = Coordinate::ThreeDim { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Coordinate::ThreeDim { x: 1.0, y: 2.0, z: 4.0 };
    /// assert!(coordinate1.equals_2d(&coordinate2));
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
    /// let coordinate1 = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// let coordinate2 = Coordinate::TwoDim { x: 1.01, y: 2.02 };
    /// assert!(coordinate1.equals_2d_with_tolerance(&coordinate2, 0.1));
    ///
    /// let coordinate1 = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// let coordinate2 = Coordinate::TwoDim { x: 3.0, y: 4.0 };
    /// assert!(!coordinate1.equals_2d_with_tolerance(&coordinate2, 0.1));
    /// 
    /// let coordinate1 = Coordinate::ThreeDim { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Coordinate::ThreeDim { x: 1.0, y: 2.0, z: 3.1 };
    /// assert!(coordinate1.equals_2d_with_tolerance(&coordinate2, 0.1));
    /// ```
    pub fn equals_2d_with_tolerance(&self, other: &Coordinate, tolerance: f64) -> bool {
        (self.x() - other.x()).abs() - tolerance < f64::EPSILON
        && (self.y() - other.y()).abs() - tolerance < f64::EPSILON
    }

    /// Checks if the 3D coordinates are equal.
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
    /// let coordinate1 = Coordinate::ThreeDim { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Coordinate::ThreeDim { x: 1.0, y: 2.0, z: 3.0 };
    /// assert!(coordinate1.equals_3d(&coordinate2));
    ///
    /// let coordinate1 = Coordinate::ThreeDim { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Coordinate::ThreeDim { x: 3.0, y: 4.0, z: 5.0 };
    /// assert!(!coordinate1.equals_3d(&coordinate2));
    /// ```
    pub fn equals_3d(&self, other: &Coordinate) -> bool {
        self.equals_2d(other) && self.z() == other.z()
    }

    /// Checks if the 3D coordinates are equal within a given tolerance.
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
    /// let coordinate1 = Coordinate::ThreeDim { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Coordinate::ThreeDim { x: 1.01, y: 2.02, z: 3.03 };
    /// assert!(coordinate1.equals_3d_with_tolerance(&coordinate2, 0.1));
    ///
    /// let coordinate1 = Coordinate::ThreeDim { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Coordinate::ThreeDim { x: 3.0, y: 4.0, z: 5.0 };
    /// assert!(!coordinate1.equals_3d_with_tolerance(&coordinate2, 0.1));
    /// ```
    pub fn equals_3d_with_tolerance(&self, other: &Coordinate, tolerance: f64) -> bool {
        self.equals_2d_with_tolerance(other, tolerance) 
        && (self.z() - other.z()).abs() - tolerance < f64::EPSILON
    }

    /// Checks if the z values of the 3D coordinates are equal within a given tolerance.
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
    /// let coordinate1 = Coordinate::ThreeDim { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Coordinate::ThreeDim { x: 2.0, y: 1.0, z: 3.1 };
    /// assert!(coordinate1.equals_in_z(&coordinate2, 0.1));
    ///
    /// let coordinate1 = Coordinate::ThreeDim { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Coordinate::ThreeDim { x: 2.0, y: 1.0, z: 4.0 };
    /// assert!(!coordinate1.equals_in_z(&coordinate2, 0.1));
    /// ```
    pub fn equals_in_z(&self, other: &Coordinate, tolerance: f64) -> bool {
        (self.z() - other.z()).abs() - tolerance < f64::EPSILON
    }

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
    /// let coordinate = Coordinate::TwoDim { x: 1.0, y: 2.0 };
    /// assert_eq!(coordinate.to_string(), "(1, 2, 0)");
    ///
    /// let coordinate = Coordinate::ThreeDim { x: 3.0, y: 4.0, z: 5.0 };
    /// assert_eq!(coordinate.to_string(), "(3, 4, 5)");
    /// ```
    pub fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x(), self.y(), self.z())

    }
 }


use std::cmp::PartialEq;
/// Implements the `PartialEq` trait for the `Coordinate` struct.
/// This allows for comparing two `Coordinate` instances for equality.
impl PartialEq for Coordinate {
    /// Compares two `Coordinate` instances for equality.
    ///
    /// # Arguments
    ///
    /// * `other` - The other `Coordinate` instance to compare with.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coord1 = Coordinate::ThreeDim {x: 1.0, y: 2.0, z: 3.0};
    /// let coord2 = Coordinate::ThreeDim {x: 1.0, y: 2.0, z: 3.0};
    ///
    /// assert!(coord1 == coord2);
    ///
    /// let coord1 = Coordinate::ThreeDim {x: 1.0, y: 2.0, z: 0.0};
    /// let coord2 = Coordinate::TwoDim {x: 1.0, y: 2.0};
    ///
    /// assert!(coord1 == coord2);    
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.equals_3d(other)
    }

    /// Compares two `Coordinate` instances for inequality.
    ///
    /// # Arguments
    ///
    /// * `other` - The other `Coordinate` instance to compare with.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Coordinate;
    ///
    /// let coord1 = Coordinate::ThreeDim {x: 1.0, y: 2.0, z: 3.0};
    /// let coord2 = Coordinate::ThreeDim {x: 1.0, y: 5.0, z: 3.0};
    ///
    /// assert!(coord1 != coord2);
    /// ```
    fn ne(&self, other: &Self) -> bool {
        !self.equals_3d(other)
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
/// let coord = coord!(10.0, 20.0);
/// assert_eq!(coord, Coordinate::TwoDim { x: 10.0, y: 20.0 });
///
/// let coord = coord!(10.0, 20.0, 30.0 );
/// assert_eq!(coord, Coordinate::ThreeDim { x: 10.0, y: 20.0, z: 30.0 });
/// ```
#[macro_export]
macro_rules! coord {    
    ( $x:expr, $y:expr ) => {
        Coordinate::TwoDim { x: $x, y: $y }
    };
    ( $x:expr, $y:expr, $z:expr ) => {
        Coordinate::ThreeDim { x: $x, y: $y, z: $z }
    };
}
 