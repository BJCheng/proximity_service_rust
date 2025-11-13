use crate::data::business::Business;
use crate::data::business::Direction;

pub struct Quadtree {
    pub businesses: Vec<Business>,
    pub north_west: Option<Box<Quadtree>>,
    pub north_east: Option<Box<Quadtree>>,
    pub south_west: Option<Box<Quadtree>>,
    pub south_east: Option<Box<Quadtree>>,
}

impl Quadtree {
    const MAX_NUMBER_BUSINESS: usize = 1;

    pub fn new() -> Self {
        Quadtree {
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

    pub fn put(self: &mut Self, businesses: &mut Vec<Business>) {
        if businesses.is_empty() {
            return;
        }

        let next_business = businesses.pop().unwrap();

        if !self.is_full() {
            self.businesses.push(next_business);
            self.put(businesses);
            return;
        }

        let next_business_direction = match next_business.get_direction() {
            Direction::NorthWest => &mut self.north_west,
            Direction::NorthEast => &mut self.north_east,
            Direction::SouthWest => &mut self.south_west,
            Direction::SouthEast => &mut self.south_east,
        };

        next_business_direction
            .get_or_insert_with(|| Box::new(Quadtree::new()))
            .put(businesses)
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
}
