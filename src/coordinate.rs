/// Represents a coordinate in either 2D or 3D space.
pub enum Cooordinate {
    /// Represents a 2D coordinate with x and y values.
    D2 { x: f64, y: f64 },
    /// Represents a 3D coordinate with x, y, and z values.
    D3 { x: f64, y: f64, z: f64 }
}

impl Cooordinate {
    /// Returns the x value of the coordinate.
    pub fn x(&self) -> f64 {
        match self {
            Cooordinate::D2 { x, .. } => *x,
            Cooordinate::D3 { x, .. } => *x
        }
    }

    /// Returns the y value of the coordinate.
    pub fn y(&self) -> f64 {
        match self {
            Cooordinate::D2 { y, .. } => *y,
            Cooordinate::D3 { y, .. } => *y
        }
    }

    /// Returns the z value of the coordinate.
    /// If the coordinate is 2D, it returns 0.0.
    pub fn z(&self) -> f64 {
        match self {
            Cooordinate::D3 { z, .. } => *z,
            _ => 0.0
        }
    }

    /// Checks if the coordinate values are valid, meaning they are finite.
    pub fn is_valid(&self) -> bool {
        match self {
            Cooordinate::D2 { x, y } => x.is_finite() && y.is_finite(),
            Cooordinate::D3 { x, y, z } => x.is_finite() && y.is_finite() && z.is_finite()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_x() {
        let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
        assert_eq!(coordinate.x(), 1.0);

        let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
        assert_eq!(coordinate.x(), 3.0);
    }

    #[test]
    fn test_coordinate_y() {
        let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
        assert_eq!(coordinate.y(), 2.0);

        let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
        assert_eq!(coordinate.y(), 4.0);
    }

    #[test]
    fn test_coordinate_z() {
        let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
        assert_eq!(coordinate.z(), 0.0);

        let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
        assert_eq!(coordinate.z(), 5.0);
    }

    #[test]
    fn test_coordinate_is_valid() {
        let coordinate = Cooordinate::D2 { x: 1.0, y: 2.0 };
        assert!(coordinate.is_valid());

        let coordinate = Cooordinate::D3 { x: 3.0, y: 4.0, z: 5.0 };
        assert!(coordinate.is_valid());

        let coordinate = Cooordinate::D2 { x: f64::INFINITY, y: 2.0 };
        assert!(!coordinate.is_valid());

        let coordinate = Cooordinate::D3 { x: 3.0, y: f64::NAN, z: 5.0 };
        assert!(!coordinate.is_valid());
    }
}
