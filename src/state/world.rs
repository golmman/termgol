use crate::common::point::Point;

pub struct World {
    pub size: Point,
    pub cells: Vec<u32>,
    pub birth_rule: Vec<u32>,
    pub survival_rule: Vec<u32>,
}

impl World {
    pub fn new() -> Self {
        Self {
            size: Point::new(0, 0),
            cells: Vec::new(),
            birth_rule: vec![3],
            survival_rule: vec![2, 3],
        }
    }

    pub fn resize(&mut self, size: &Point) {
        self.cells = vec![0; (size.width() * size.height()) as usize];

        self.cells[(size.width() * 5 + 3) as usize] = 1;
        self.cells[(size.width() * 5 + 4) as usize] = 1;
        self.cells[(size.width() * 6 + 4) as usize] = 1;
        self.cells[(size.width() * 6 + 5) as usize] = 1;
        self.cells[(size.width() * 7 + 4) as usize] = 1;

        self.size = size.clone();
    }

    pub fn update(&mut self) {
        let mut new_cells = vec![0; (self.size.width() * self.size.height()) as usize];

        for y in 0..self.size.height() {
            for x in 0..self.size.width() {
                let i = (self.size.width() * y + x) as usize;
                let cell = self.cells[i];
                let neighbour_cell_count = self.count_neighbor_cells(&Point::new(x, y));

                if cell == 0 {
                    if self
                        .birth_rule
                        .iter()
                        .find(|&&count| count == neighbour_cell_count)
                        .is_some()
                    {
                        new_cells[i] = 1;
                    } else {
                        new_cells[i] = 0;
                    }
                } else {
                    if self
                        .survival_rule
                        .iter()
                        .find(|&&count| count == neighbour_cell_count)
                        .is_some()
                    {
                        new_cells[i] = 1;
                    } else {
                        new_cells[i] = 0;
                    }
                }
            }
        }

        self.cells = new_cells;
    }

    fn count_neighbor_cells(&self, p: &Point) -> u32 {
        let mut count = 0;
        let neighbour_indices = self.get_neighbour_indices(p);

        for index in neighbour_indices {
            if self.cells[index] != 0 {
                count += 1;
            }
        }

        count
    }

    fn get_neighbour_indices(&self, p: &Point) -> Vec<usize> {
        vec![
            self.transform_toroidal_point(&p.left()),
            self.transform_toroidal_point(&p.right()),
            self.transform_toroidal_point(&p.up()),
            self.transform_toroidal_point(&p.up_left()),
            self.transform_toroidal_point(&p.up_right()),
            self.transform_toroidal_point(&p.down()),
            self.transform_toroidal_point(&p.down_left()),
            self.transform_toroidal_point(&p.down_right()),
        ]
        .iter()
        .map(|p| (self.size.width() * p.y + p.x) as usize)
        .collect()
    }

    fn transform_toroidal_point(&self, p: &Point) -> Point {
        let new_x = Self::toroidal_mod(p.x, self.size.width());
        let new_y = Self::toroidal_mod(p.y, self.size.height());
        Point::new(new_x, new_y)
    }

    fn toroidal_mod(n: i32, m: i32) -> i32 {
        let modulus = n % m;

        if modulus < 0 {
            return m + modulus;
        }
        modulus
    }
}
