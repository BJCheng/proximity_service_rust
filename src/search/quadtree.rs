use crate::data::business::Business;
use crate::data::business::Direction;

pub struct Quadtree {
    pub businesses: Vec<Business>,
    pub north_west: Box<Quadtree>,
    pub north_east: Box<Quadtree>,
    pub south_west: Box<Quadtree>,
    pub south_east: Box<Quadtree>,
}

impl Quadtree {
    const MAX_NUMBER_BUSINESS: usize = 4;

    // pub fn new() -> Self {}

    pub fn is_full(self: &Self) -> bool {
        self.businesses.capacity() >= Self::MAX_NUMBER_BUSINESS
    }

    pub fn put(self: &mut Self, mut businesses: Vec<Business>) {
        if businesses.is_empty() {
            return;
        }

        let next_business = businesses.pop().unwrap();

        if !self.is_full() {
            self.businesses.push(next_business);
            self.put(businesses);
            return;
        }

        match next_business.get_direction() {
            Direction::NorthWest => &self.north_west.put(businesses),
            Direction::NorthEast => &self.north_east.put(businesses),
            Direction::SouthWest => &self.south_west.put(businesses),
            Direction::SouthEast => &self.south_east.put(businesses),
        };
    }
}
