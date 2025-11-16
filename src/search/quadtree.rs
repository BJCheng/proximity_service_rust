use crate::data::business::Business;
use crate::data::business::Direction;

pub struct Quadtree {
    left_longitude: i32,
    right_longitude: i32,
    top_latitude: i32,
    bottom_latitude: i32,
    pub businesses: Vec<Business>,
    pub north_west: Option<Box<Quadtree>>,
    pub north_east: Option<Box<Quadtree>>,
    pub south_west: Option<Box<Quadtree>>,
    pub south_east: Option<Box<Quadtree>>,
}

impl Quadtree {
    const MAX_NUMBER_BUSINESS: usize = 1;
    const EMPTY: Vec<Business> = Vec::new();

    pub fn new(
        left_longitude: i32,
        right_longitude: i32,
        top_latitude: i32,
        bottom_latitude: i32,
    ) -> Self {
        Quadtree {
            left_longitude: left_longitude,
            right_longitude: right_longitude,
            top_latitude: top_latitude,
            bottom_latitude: bottom_latitude,
            businesses: Vec::new(),
            north_west: None,
            north_east: None,
            south_west: None,
            south_east: None,
        }
    }

    pub fn is_full(&self) -> bool {
        self.businesses.len() >= Self::MAX_NUMBER_BUSINESS
    }

    pub fn find_relative_direction(self: &Self, longitude: i32, latitude: i32) -> Direction {
        let mid_longitude = (self.left_longitude + self.right_longitude) / 2;
        let mid_latitude = (self.top_latitude + self.bottom_latitude) / 2;
        let is_west = longitude - mid_longitude <= 0;
        let is_north = latitude - mid_latitude > 0;

        match (is_west, is_north) {
            (true, true) => Direction::NorthWest,
            (true, false) => Direction::SouthWest,
            (false, true) => Direction::NorthEast,
            (false, false) => Direction::SouthEast,
        }
    }

    pub fn put(self: &mut Self, remaining_businesses: &mut Vec<Business>) {
        if remaining_businesses.is_empty() {
            return;
        }

        let next_business = remaining_businesses.pop().unwrap();

        if !self.is_full() {
            self.businesses.push(next_business);
            self.put(remaining_businesses);
            return;
        }

        let next_quadtree = match next_business.get_direction() {
            Direction::NorthWest => &mut self.north_west,
            Direction::NorthEast => &mut self.north_east,
            Direction::SouthWest => &mut self.south_west,
            Direction::SouthEast => &mut self.south_east,
        };

        next_quadtree
            .get_or_insert_with(|| Box::new(Quadtree::new(-180, 180, 90, -90)))
            .put(remaining_businesses)
    }

    pub fn print(&self) {
        if self.businesses.is_empty() {
            return;
        }
        for business in &self.businesses {
            println!("{}", &business.name);
        }

        if let Some(north_west) = &self.north_west {
            println!("↖");
            north_west.print();
        }
        if let Some(north_east) = &self.north_east {
            println!("↗");
            north_east.print();
        }
        if let Some(south_west) = &self.south_west {
            println!("↙");
            south_west.print();
        }
        if let Some(south_east) = &self.south_east {
            println!("↘");
            south_east.print();
        }
    }

    // fn next_quadtree(latitude: i32, longitude: i32) -> &Quadtree {}

    pub fn search(self: &Self, longitude: i32, latitude: i32) -> &[Business] {
        // todo 1: find the leaf first
        //         otherwise will always return the business in top quadtree node
        // todo 2: will have to store result businesses in a variable
        // return this result variable once the number of businesses meets requirement
        if self.businesses.len() >= 1 {
            return &self.businesses;
        }

        let next_direction = self.find_relative_direction(longitude, latitude);

        let next_quadtree = match next_direction {
            Direction::NorthWest => &self.north_west,
            Direction::NorthEast => &self.north_east,
            Direction::SouthWest => &self.south_west,
            Direction::SouthEast => &self.south_east,
        };

        if let Some(next_quadtree) = next_quadtree {
            return next_quadtree.search(longitude, latitude);
        };

        &[]
    }
}
