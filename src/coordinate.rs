/// Represents a coordinate in either 2D or 3D space.
pub enum Cooordinate {
    /// Represents a 2D coordinate with x and y values.
    D2 { x: f64, y: f64 },
    /// Represents a 3D coordinate with x, y, and z values.
    D3 { x: f64, y: f64, z: f64 }
}

impl Cooordinate {

    /// Returns the x value of the coordinate.
    ///
    /// # Arguments
    ///
    /// * `self` - The coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// assert_eq!(coordinate.x(), 1.0);
    ///
    /// let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
    /// assert_eq!(coordinate.x(), 3.0);
    /// ```
    pub fn x(&self) -> f64 {
        match self {
            Cooordinate::D2 { x, .. } => *x,
            Cooordinate::D3 { x, .. } => *x
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// assert_eq!(coordinate.y(), 2.0);
    ///
    /// let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
    /// assert_eq!(coordinate.y(), 4.0);
    /// ```
    pub fn y(&self) -> f64 {
        match self {
            Cooordinate::D2 { y, .. } => *y,
            Cooordinate::D3 { y, .. } => *y
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// assert_eq!(coordinate.z(), 0.0);
    ///
    /// let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
    /// assert_eq!(coordinate.z(), 5.0);
    /// ```
    pub fn z(&self) -> f64 {
        match self {
            Cooordinate::D3 { z, .. } => *z,
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// assert_eq!(coordinate.get_ordinate(0), 1.0);
    /// assert_eq!(coordinate.get_ordinate(1), 2.0);
    ///
    /// let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
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

    /// Checks if the coordinate values are valid, meaning they are finite.
    ///
    /// # Arguments
    ///
    /// * `self` - The coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// assert!(coordinate.is_valid());
    ///
    /// let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
    /// assert!(coordinate.is_valid());
    ///
    /// let coordinate = Cooordinate::D2 { x: f64::INFINITY, y: 2.0 };
    /// assert!(!coordinate.is_valid());
    /// 
    /// let coordinate = Cooordinate::D3 { x: 3.0, y: f64::NAN, z: 5.0 };
    /// assert!(!coordinate.is_valid());    
    /// ```
    pub fn is_valid(&self) -> bool {
        match self {
            Cooordinate::D2 { x, y } => x.is_finite() && y.is_finite(),
            Cooordinate::D3 { x, y, z } => x.is_finite() && y.is_finite() && z.is_finite()
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate1 = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// let coordinate2 = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// assert!(coordinate1.equals_2d(&coordinate2));
    ///
    /// let coordinate1 = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// let coordinate2 = Cooordinate::D2 { x: 3.0, y: 4.0 };
    /// assert!(!coordinate1.equals_2d(&coordinate2));
    ///
    /// let coordinate1 = Cooordinate::D3 { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Cooordinate::D3 { x: 1.0, y: 2.0, z: 4.0 };
    /// assert!(coordinate1.equals_2d(&coordinate2));
    /// ```
    pub fn equals_2d(&self, other: &Cooordinate) -> bool {
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate1 = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// let coordinate2 = Cooordinate::D2 { x: 1.01, y: 2.02 };
    /// assert!(coordinate1.equals_2d_with_tolerance(&coordinate2, 0.1));
    ///
    /// let coordinate1 = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// let coordinate2 = Cooordinate::D2 { x: 3.0, y: 4.0 };
    /// assert!(!coordinate1.equals_2d_with_tolerance(&coordinate2, 0.1));
    /// 
    /// let coordinate1 = Cooordinate::D3 { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Cooordinate::D3 { x: 1.0, y: 2.0, z: 3.1 };
    /// assert!(coordinate1.equals_2d_with_tolerance(&coordinate2, 0.1));
    /// ```
    pub fn equals_2d_with_tolerance(&self, other: &Cooordinate, tolerance: f64) -> bool {
        (self.x() - other.x()).abs() <= tolerance 
        && (self.y() - other.y()).abs() <= tolerance
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate1 = Cooordinate::D3 { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Cooordinate::D3 { x: 1.0, y: 2.0, z: 3.0 };
    /// assert!(coordinate1.equals_3d(&coordinate2));
    ///
    /// let coordinate1 = Cooordinate::D3 { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
    /// assert!(!coordinate1.equals_3d(&coordinate2));
    /// ```
    pub fn equals_3d(&self, other: &Cooordinate) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }

}