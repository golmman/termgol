use crate::common::{args::CellSetup, point::Point};

pub struct World {
    pub cell_setup: CellSetup,
    pub birth_rule: Vec<u32>,
    pub cells: Vec<u32>,
    pub size: Point,
    pub survival_rule: Vec<u32>,
}

impl From<CellSetup> for World {
    fn from(cell_setup: CellSetup) -> Self {
        Self {
            cell_setup,
            birth_rule: vec![3],
            cells: Vec::new(),
            size: Point::new(0, 0),
            survival_rule: vec![2, 3],
        }
    }
}

impl World {
    pub fn resize(&mut self, size: &Point) {
        self.size = size.clone();

        self.setup_cells();
    }

    pub fn setup_cells(&mut self) {
        match self.cell_setup {
            CellSetup::Acorn => self.setup_acorn(),
            CellSetup::Blank => self.setup_blank(),
            CellSetup::RPentonimo => self.setup_r_pentonimo(),
        }
    }

    fn setup_acorn(&mut self) {
        self.setup_blank();

        let center = Point::new(self.size.width() / 2, self.size.height() / 2);

        self.cells[(self.size.width() * (center.y - 1) + (center.x - 2)) as usize] = 1;
        self.cells[(self.size.width() * center.y + center.x) as usize] = 1;
        self.cells[(self.size.width() * (center.y + 1) + (center.x - 3)) as usize] = 1;
        self.cells[(self.size.width() * (center.y + 1) + (center.x - 2)) as usize] = 1;
        self.cells[(self.size.width() * (center.y + 1) + (center.x + 1)) as usize] = 1;
        self.cells[(self.size.width() * (center.y + 1) + (center.x + 2)) as usize] = 1;
        self.cells[(self.size.width() * (center.y + 1) + (center.x + 3)) as usize] = 1;
    }

    fn setup_blank(&mut self) {
        self.cells = vec![0; (self.size.width() * self.size.height()) as usize];
    }

    fn setup_r_pentonimo(&mut self) {
        self.setup_blank();

        let center = Point::new(self.size.width() / 2, self.size.height() / 2);
        self.cells[(self.size.width() * (center.y - 1) + (center.x - 1)) as usize] = 1;
        self.cells[(self.size.width() * (center.y - 1) + center.x) as usize] = 1;
        self.cells[(self.size.width() * center.y + center.x) as usize] = 1;
        self.cells[(self.size.width() * center.y + (center.x + 1)) as usize] = 1;
        self.cells[(self.size.width() * (center.y + 1) + center.x) as usize] = 1;
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
