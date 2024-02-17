pub enum Cooordinate {
    D2 { x: f64, y: f64 },
    D3 { x: f64, y: f64, z: f64 }
}
impl  Cooordinate {
    //returns the x value of the coordinate
    pub fn x(&self) -> f64 {
        match self {
            Cooordinate::D2 { x, .. } => *x,
            Cooordinate::D3 { x, .. } => *x
        }
    }
    //returns the y value of the coordinate
    pub fn y(&self) -> f64 {
        match self {
            Cooordinate::D2 { y, .. } => *y,
            Cooordinate::D3 { y, .. } => *y
        }
    }
    //returns the z value of the coordinate
    pub fn z(&self) -> f64 {
        match self {
            Cooordinate::D3 { z, .. } => *z,
            _ => 0.0
        }
    }
    //checks the coordinate values are valid, meaning they are finite
    pub fn is_valid(&self) -> bool {
        match self {
            Cooordinate::D2 { x, y } => x.is_finite() && y.is_finite(),
            Cooordinate::D3 { x, y, z } => x.is_finite() && y.is_finite() && z.is_finite()
        }
    }

}

