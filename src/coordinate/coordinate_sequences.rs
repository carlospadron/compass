use crate::coordinate::Coordinate;

pub struct CoordinateSequence {
    coordinates: Vec<Coordinate>,
}

impl CoordinateSequence {

    pub fn new(coordinates: Vec<Coordinate>) -> Self {
        Self { coordinates }
    }

    pub fn get_coordinates(&self) -> &Vec<Coordinate> {
        &self.coordinates
    }

    pub fn get_coordinates_mut(&mut self) -> &mut Vec<Coordinate> {
        &mut self.coordinates
    }

    pub fn get_coordinate(&self, index: usize) -> Option<&Coordinate> {
        self.coordinates.get(index)
    }

    pub fn get_coordinate_mut(&mut self, index: usize) -> Option<&mut Coordinate> {
        self.coordinates.get_mut(index)
    }

    pub fn set_coordinate(&mut self, index: usize, coordinate: Coordinate) {
        self.coordinates[index] = coordinate;
    }

    pub fn add_coordinate(&mut self, coordinate: Coordinate) {
        self.coordinates.push(coordinate);
    }

    pub fn remove_coordinate(&mut self, index: usize) -> Option<Coordinate> {
        self.coordinates.remove(index)
    }

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