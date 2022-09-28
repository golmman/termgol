use crate::common::{
    args::Args,
    cell_setup::CellSetup,
    color::{Color, Rgb},
    point::Point,
};

#[derive(Clone)]
pub struct Cell {
    pub alive: bool,
    pub color: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            alive: false,
            color: Color::default(),
        }
    }
}

pub struct World {
    pub birth_rule: Vec<u32>,
    pub cell_setup: CellSetup,
    pub cells: Vec<Cell>,
    pub color_bg_alive: Rgb,
    pub color_bg_dead: Rgb,
    pub size: Point,
    pub survival_rule: Vec<u32>,
}

impl From<Args> for World {
    fn from(args: Args) -> Self {
        Self {
            birth_rule: args.rules.birth.clone(),
            cell_setup: args.cell_setup,
            cells: Vec::new(),
            color_bg_alive: args.color_bg_alive,
            color_bg_dead: args.color_bg_dead,
            size: Point::new(0, 0),
            survival_rule: args.rules.survival.clone(),
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
            CellSetup::Termgol => self.setup_termgol(),
        }
    }

    pub fn set_alive(&mut self, i: usize) {
        let cell = Cell {
            alive: true,
            color: Color {
                fg: Rgb::default(),
                bg: self.color_bg_alive,
            },
        };

        self.cells[i] = cell;
    }

    fn set_alive_p(&mut self, p: Point) {
        self.set_alive((self.size.width() * p.y + p.x) as usize);
    }

    pub fn set_dead(&mut self, i: usize) {
        let cell = Cell {
            alive: false,
            color: Color {
                fg: Rgb::default(),
                bg: self.color_bg_dead,
            },
        };

        self.cells[i] = cell;
    }

    fn setup_acorn(&mut self) {
        self.setup_blank();

        let center = Point::new(self.size.width() / 2, self.size.height() / 2);

        self.set_alive_p(Point::new(center.x - 2, center.y - 1));
        self.set_alive_p(Point::new(center.x + 0, center.y + 0));
        self.set_alive_p(Point::new(center.x - 3, center.y + 1));
        self.set_alive_p(Point::new(center.x - 2, center.y + 1));
        self.set_alive_p(Point::new(center.x + 1, center.y + 1));
        self.set_alive_p(Point::new(center.x + 2, center.y + 1));
        self.set_alive_p(Point::new(center.x + 3, center.y + 1));
    }

    fn setup_blank(&mut self) {
        let dead_cell = Cell {
            alive: false,
            color: Color {
                fg: Rgb::default(),
                bg: self.color_bg_dead,
            },
        };
        self.cells = vec![dead_cell; (self.size.width() * self.size.height()) as usize];
    }

    fn setup_r_pentonimo(&mut self) {
        self.setup_blank();

        let center = Point::new(self.size.width() / 2, self.size.height() / 2);

        self.set_alive_p(Point::new(center.x - 1, center.y - 1));
        self.set_alive_p(Point::new(center.x + 0, center.y - 1));
        self.set_alive_p(Point::new(center.x + 0, center.y + 0));
        self.set_alive_p(Point::new(center.x + 1, center.y + 0));
        self.set_alive_p(Point::new(center.x + 0, center.y + 1));
    }

    fn setup_termgol(&mut self) {
        self.setup_blank();

        let center = Point::new(self.size.width() / 2, self.size.height() / 2);

        self.set_alive_p(Point::new(center.x - 20, center.y - 2));
        self.set_alive_p(Point::new(center.x - 19, center.y - 2));
        self.set_alive_p(Point::new(center.x - 18, center.y - 2));
        self.set_alive_p(Point::new(center.x - 17, center.y - 2));
        self.set_alive_p(Point::new(center.x - 16, center.y - 2));
        self.set_alive_p(Point::new(center.x - 18, center.y - 1));
        self.set_alive_p(Point::new(center.x - 18, center.y - 0));
        self.set_alive_p(Point::new(center.x - 18, center.y + 1));
        self.set_alive_p(Point::new(center.x - 18, center.y + 2));
        self.set_alive_p(Point::new(center.x - 14, center.y - 2));
        self.set_alive_p(Point::new(center.x - 13, center.y - 2));
        self.set_alive_p(Point::new(center.x - 12, center.y - 2));
        self.set_alive_p(Point::new(center.x - 11, center.y - 2));
        self.set_alive_p(Point::new(center.x - 10, center.y - 2));
        self.set_alive_p(Point::new(center.x - 14, center.y - 1));
        self.set_alive_p(Point::new(center.x - 14, center.y - 0));
        self.set_alive_p(Point::new(center.x - 13, center.y - 0));
        self.set_alive_p(Point::new(center.x - 12, center.y - 0));
        self.set_alive_p(Point::new(center.x - 11, center.y - 0));
        self.set_alive_p(Point::new(center.x - 10, center.y - 0));
        self.set_alive_p(Point::new(center.x - 14, center.y + 1));
        self.set_alive_p(Point::new(center.x - 14, center.y + 2));
        self.set_alive_p(Point::new(center.x - 13, center.y + 2));
        self.set_alive_p(Point::new(center.x - 12, center.y + 2));
        self.set_alive_p(Point::new(center.x - 11, center.y + 2));
        self.set_alive_p(Point::new(center.x - 10, center.y + 2));
        self.set_alive_p(Point::new(center.x - 8, center.y - 2));
        self.set_alive_p(Point::new(center.x - 7, center.y - 2));
        self.set_alive_p(Point::new(center.x - 6, center.y - 2));
        self.set_alive_p(Point::new(center.x - 5, center.y - 2));
        self.set_alive_p(Point::new(center.x - 8, center.y - 1));
        self.set_alive_p(Point::new(center.x - 4, center.y - 1));
        self.set_alive_p(Point::new(center.x - 8, center.y - 0));
        self.set_alive_p(Point::new(center.x - 7, center.y - 0));
        self.set_alive_p(Point::new(center.x - 6, center.y - 0));
        self.set_alive_p(Point::new(center.x - 5, center.y - 0));
        self.set_alive_p(Point::new(center.x - 8, center.y + 1));
        self.set_alive_p(Point::new(center.x - 6, center.y + 1));
        self.set_alive_p(Point::new(center.x - 8, center.y + 2));
        self.set_alive_p(Point::new(center.x - 5, center.y + 2));
        self.set_alive_p(Point::new(center.x - 2, center.y - 2));
        self.set_alive_p(Point::new(center.x + 2, center.y - 2));
        self.set_alive_p(Point::new(center.x - 2, center.y - 1));
        self.set_alive_p(Point::new(center.x - 1, center.y - 1));
        self.set_alive_p(Point::new(center.x + 1, center.y - 1));
        self.set_alive_p(Point::new(center.x + 2, center.y - 1));
        self.set_alive_p(Point::new(center.x - 2, center.y - 0));
        self.set_alive_p(Point::new(center.x + 0, center.y - 0));
        self.set_alive_p(Point::new(center.x + 2, center.y - 0));
        self.set_alive_p(Point::new(center.x - 2, center.y + 1));
        self.set_alive_p(Point::new(center.x + 2, center.y + 1));
        self.set_alive_p(Point::new(center.x - 2, center.y + 2));
        self.set_alive_p(Point::new(center.x + 2, center.y + 2));
        self.set_alive_p(Point::new(center.x + 5, center.y - 2));
        self.set_alive_p(Point::new(center.x + 6, center.y - 2));
        self.set_alive_p(Point::new(center.x + 7, center.y - 2));
        self.set_alive_p(Point::new(center.x + 8, center.y - 2));
        self.set_alive_p(Point::new(center.x + 4, center.y - 1));
        self.set_alive_p(Point::new(center.x + 4, center.y - 0));
        self.set_alive_p(Point::new(center.x + 7, center.y - 0));
        self.set_alive_p(Point::new(center.x + 8, center.y - 0));
        self.set_alive_p(Point::new(center.x + 4, center.y + 1));
        self.set_alive_p(Point::new(center.x + 8, center.y + 1));
        self.set_alive_p(Point::new(center.x + 5, center.y + 2));
        self.set_alive_p(Point::new(center.x + 6, center.y + 2));
        self.set_alive_p(Point::new(center.x + 7, center.y + 2));
        self.set_alive_p(Point::new(center.x + 8, center.y + 2));
        self.set_alive_p(Point::new(center.x + 11, center.y - 2));
        self.set_alive_p(Point::new(center.x + 12, center.y - 2));
        self.set_alive_p(Point::new(center.x + 13, center.y - 2));
        self.set_alive_p(Point::new(center.x + 10, center.y - 1));
        self.set_alive_p(Point::new(center.x + 14, center.y - 1));
        self.set_alive_p(Point::new(center.x + 10, center.y - 0));
        self.set_alive_p(Point::new(center.x + 14, center.y - 0));
        self.set_alive_p(Point::new(center.x + 10, center.y + 1));
        self.set_alive_p(Point::new(center.x + 14, center.y + 1));
        self.set_alive_p(Point::new(center.x + 11, center.y + 2));
        self.set_alive_p(Point::new(center.x + 12, center.y + 2));
        self.set_alive_p(Point::new(center.x + 13, center.y + 2));
        self.set_alive_p(Point::new(center.x + 16, center.y - 2));
        self.set_alive_p(Point::new(center.x + 16, center.y - 1));
        self.set_alive_p(Point::new(center.x + 16, center.y - 0));
        self.set_alive_p(Point::new(center.x + 16, center.y + 1));
        self.set_alive_p(Point::new(center.x + 16, center.y + 2));
        self.set_alive_p(Point::new(center.x + 17, center.y + 2));
        self.set_alive_p(Point::new(center.x + 18, center.y + 2));
        self.set_alive_p(Point::new(center.x + 19, center.y + 2));
        self.set_alive_p(Point::new(center.x + 20, center.y + 2));
    }

    pub fn update(&mut self) {
        let mut is_alive = vec![false; (self.size.width() * self.size.height()) as usize];

        for y in 0..self.size.height() {
            for x in 0..self.size.width() {
                let i = (self.size.width() * y + x) as usize;
                let neighbour_cell_count = self.count_neighbor_cells(&Point::new(x, y));

                if self.cells[i].alive {
                    if self
                        .survival_rule
                        .iter()
                        .find(|&&count| count == neighbour_cell_count)
                        .is_some()
                    {
                        is_alive[i] = true;
                    } else {
                        is_alive[i] = false;
                    }
                } else {
                    if self
                        .birth_rule
                        .iter()
                        .find(|&&count| count == neighbour_cell_count)
                        .is_some()
                    {
                        is_alive[i] = true;
                    } else {
                        is_alive[i] = false;
                    }
                }
            }
        }

        for i in 0..self.cells.len() {
            if is_alive[i] {
                self.set_alive(i);
            } else {
                self.set_dead(i);
            }
        }
    }

    fn count_neighbor_cells(&self, p: &Point) -> u32 {
        let mut count = 0;
        let neighbour_indices = self.get_neighbour_indices(p);

        for index in neighbour_indices {
            if self.cells[index].alive {
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
