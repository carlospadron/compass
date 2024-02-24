/// Represents a coordinate in either 2D or 3D space.
pub enum Cooordinate {
    /// Represents a 2D coordinate with x and y values.
    D2 { x: f32, y: f32 },
    /// Represents a 3D coordinate with x, y, and z values.
    D3 { x: f32, y: f32, z: f32 }
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
    pub fn x(&self) -> f32 {
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
    pub fn y(&self) -> f32 {
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
    pub fn z(&self) -> f32 {
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
    pub fn get_ordinate(&self, ordinate: usize) -> f32 {
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// let new_coordinate = coordinate.set_x(3.0);
    /// assert_eq!(new_coordinate.x(), 3.0);
    /// assert_eq!(new_coordinate.y(), 2.0);
    ///
    /// let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
    /// let new_coordinate = coordinate.set_x(6.0);
    /// assert_eq!(new_coordinate.x(), 6.0);
    /// assert_eq!(new_coordinate.y(), 4.0);
    /// assert_eq!(new_coordinate.z(), 5.0);
    /// ```
    pub fn set_x(&self, new_x: f32) -> Cooordinate {
        match self {
            Cooordinate::D2 { y, .. } => Cooordinate::D2 { x: new_x, y: *y },
            Cooordinate::D3 { y, z, .. } => Cooordinate::D3 { x: new_x, y: *y, z: *z }
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// let new_coordinate = coordinate.set_y(3.0);
    /// assert_eq!(new_coordinate.x(), 1.0);
    /// assert_eq!(new_coordinate.y(), 3.0);
    ///
    /// let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
    /// let new_coordinate = coordinate.set_y(6.0);
    /// assert_eq!(new_coordinate.x(), 3.0);
    /// assert_eq!(new_coordinate.y(), 6.0);
    /// assert_eq!(new_coordinate.z(), 5.0);
    /// ```
    pub fn set_y(&self, new_y: f32) -> Cooordinate {
        match self {
            Cooordinate::D2 { x, .. } => Cooordinate::D2 { x: *x, y: new_y },
            Cooordinate::D3 { x, z, .. } => Cooordinate::D3 { x: *x, y: new_y, z: *z }
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate = Cooordinate::D3 { x: 1.0, y: 2.0, z: 3.0 };
    /// let new_coordinate = coordinate.set_z(4.0);
    /// assert_eq!(new_coordinate.x(), 1.0);
    /// assert_eq!(new_coordinate.y(), 2.0);
    /// assert_eq!(new_coordinate.z(), 4.0);
    ///
    /// let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// let new_coordinate = coordinate.set_z(4.0);
    /// assert_eq!(new_coordinate.x(), 1.0);
    /// assert_eq!(new_coordinate.y(), 2.0);
    /// assert_eq!(new_coordinate.z(), 4.0);    
    /// ```
    pub fn set_z(&self, new_z: f32) -> Cooordinate {
        match self {
            Cooordinate::D2 { x, y } => Cooordinate::D3 { x: *x, y: *y, z: new_z },
            Cooordinate::D3 { x, y, .. } => Cooordinate::D3 { x: *x, y: *y, z: new_z },
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// let new_coordinate = coordinate.set_ordinate(0, 3.0);
    /// assert_eq!(new_coordinate.x(), 3.0);
    /// assert_eq!(new_coordinate.y(), 2.0);
    ///
    /// let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
    /// let new_coordinate = coordinate.set_ordinate(2, 6.0);
    /// assert_eq!(new_coordinate.x(), 3.0);
    /// assert_eq!(new_coordinate.y(), 4.0);
    /// assert_eq!(new_coordinate.z(), 6.0);
    /// ```
    pub fn set_ordinate(&self, ordinate: usize, new_value: f32) -> Cooordinate {
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
    /// assert!(coordinate.is_valid());
    ///
    /// let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
    /// assert!(coordinate.is_valid());
    ///
    /// let coordinate = Cooordinate::D2 { x: f32::INFINITY, y: 2.0 };
    /// assert!(!coordinate.is_valid());
    /// 
    /// let coordinate = Cooordinate::D3 { x: 3.0, y: f32::NAN, z: 5.0 };
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
    pub fn equals_2d_with_tolerance(&self, other: &Cooordinate, tolerance: f32) -> bool {
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate1 = Cooordinate::D3 { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Cooordinate::D3 { x: 1.01, y: 2.02, z: 3.03 };
    /// assert!(coordinate1.equals_3d_with_tolerance(&coordinate2, 0.1));
    ///
    /// let coordinate1 = Cooordinate::D3 { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
    /// assert!(!coordinate1.equals_3d_with_tolerance(&coordinate2, 0.1));
    /// ```
    pub fn equals_3d_with_tolerance(&self, other: &Cooordinate, tolerance: f32) -> bool {
        self.equals_2d_with_tolerance(other, tolerance) && (self.z() - other.z()).abs() <= tolerance
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
    /// use geoms::coordinate::Cooordinate;
    ///
    /// let coordinate1 = Cooordinate::D3 { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Cooordinate::D3 { x: 2.0, y: 1.0, z: 3.1 };
    /// assert!(coordinate1.equals_in_z(&coordinate2, 0.1));
    ///
    /// let coordinate1 = Cooordinate::D3 { x: 1.0, y: 2.0, z: 3.0 };
    /// let coordinate2 = Cooordinate::D3 { x: 2.0, y: 1.0, z: 4.0 };
    /// assert!(!coordinate1.equals_in_z(&coordinate2, 0.1));
    /// ```
    pub fn equals_in_z(&self, other: &Cooordinate, tolerance: f32) -> bool {
        (self.z() - other.z()).abs() <= tolerance
    }
}