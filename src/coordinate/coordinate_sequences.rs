use crate::coordinate::Coordinate;

#[derive(Clone)]
pub struct CoordinateSequence {
    coordinates: Vec<Coordinate>,
}

impl CoordinateSequence {
    /// Creates a new CoordinateSequence
    /// 
    /// # Examples
    /// ```
    /// use geoms::coord;
    /// use geoms::coordinate::Coordinate;
    /// use geoms::coordinate::coordinate_sequences::CoordinateSequence;
    /// 
    /// let coordinates = vec![coord!(0, 0), coord!(1, 1), coord!(2, 2)];
    /// let sequence = CoordinateSequence::new(coordinates);
    /// ```
    pub fn new(coordinates: Vec<Coordinate>) -> Self {
        Self { coordinates }
    }

    /// Returns the coordinates of the sequence
    /// 
    /// # Examples
    /// ```
    /// use geoms::coord;
    /// use geoms::coordinate::Coordinate;
    /// use geoms::coordinate::coordinate_sequences::CoordinateSequence;
    /// 
    /// let coordinates = vec![coord!(0, 0), coord!(1, 1), coord!(2, 2)];
    /// let sequence = CoordinateSequence::new(coordinates);
    /// assert_eq!(sequence.get_coordinates(), &vec![coord!(0, 0), coord!(1, 1), coord!(2, 2)]);
    /// ```
    pub fn get_coordinates(&self) -> &Vec<Coordinate> {
        &self.coordinates
    }

    /// Returns the coordinate at the given index
    /// 
    /// # Examples
    /// ```
    /// use geoms::coord;
    /// use geoms::coordinate::Coordinate;
    /// use geoms::coordinate::coordinate_sequences::CoordinateSequence;
    /// 
    /// let coordinates = vec![coord!(0, 0), coord!(1, 1), coord!(2, 2)];
    /// let sequence = CoordinateSequence::new(coordinates);
    /// assert_eq!(sequence.get_coordinate(1), Some(&coord!(1, 1)));
    /// ```
    pub fn get_coordinate(&self, index: usize) -> Option<&Coordinate> {
        self.coordinates.get(index)
    }

    /// Sets the coordinate at the given index
    /// 
    /// # Examples
    /// ```
    /// use geoms::coord;
    /// use geoms::coordinate::Coordinate;
    /// use geoms::coordinate::coordinate_sequences::CoordinateSequence;
    /// 
    /// let coordinates = vec![coord!(0, 0), coord!(1, 1), coord!(2, 2)];
    /// let sequence = CoordinateSequence::new(coordinates);
    /// let new_sequence = sequence.set_coordinate(1, coord!(3, 3));
    /// assert_eq!(new_sequence.get_coordinate(1), Some(&coord!(3, 3)));
    /// ```
    pub fn set_coordinate(&self, index: usize, coordinate: Coordinate) -> Self {
        let mut new_sequence = self.clone();
        new_sequence.coordinates[index] = coordinate;
        new_sequence
    }

    pub fn add_coordinate(&self, coordinate: Coordinate) -> Self {
        let mut new_sequence = self.clone();
        new_sequence.coordinates.push(coordinate);
        new_sequence
    }

    // pub fn remove_coordinate(&mut self, index: usize) -> Option<Coordinate> {
    //     self.coordinates.remove(index)
    // }

    pub fn len(&self) -> usize {
        self.coordinates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.coordinates.is_empty()
    }

    pub fn has_duplicates(&self) -> bool {
        let mut set = std::collections::HashSet::new();
        for coord in &self.coordinates {
            if !set.insert(coord) {
                return true;
            }
        }
        false
    }

    pub fn is_closed(&self) -> bool {
        if self.coordinates.len() < 2 {
            return false;
        }
        self.coordinates[0] == self.coordinates[self.coordinates.len() - 1]
    }
    
}

//todo
//has_duplicates
//is_closed