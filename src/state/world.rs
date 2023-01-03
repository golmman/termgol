use nanorand::Rng;
use nanorand::WyRand;

use crate::common::args::Args;
use crate::common::rainbow::Rainbow;
use term2d::model::color::Color;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;

use super::cell::Cell;
use super::cell_image::CellImage;
use super::cell_setup::CellSetup;

pub struct World {
    pub birth_rule: Vec<u32>,
    pub cell_setup: CellSetup,
    pub cells: Vec<Cell>,
    // TODO
    pub color_alpha: u8,
    pub color_bg_alive: Rgba,
    pub color_bg_dead: Rgba,
    pub fading_speed: i32,
    pub rainbow: Option<Rainbow>,
    pub size: Point,
    pub survival_rule: Vec<u32>,
}

impl From<Args> for World {
    fn from(args: Args) -> Self {
        let (color_alpha, cell_setup) = if args.screen_saver.is_some() {
            let mut rng = WyRand::new();
            let width = rng.generate_range(15_i32..=75);
            let height = rng.generate_range(5_i32..=25);
            (0, CellSetup::rect_soup(width, height))
        } else {
            (255, args.cell_setup)
        };

        let rainbow = if args.rainbow {
            Some(Rainbow::new(vec![
                Rgba::red(),
                Rgba::yellow(),
                Rgba::green(),
                Rgba::cyan(),
                Rgba::blue(),
                Rgba::violet(),
            ]))
        } else {
            None
        };

        Self {
            birth_rule: args.rules.birth.clone(),
            cell_setup,
            cells: Vec::new(),
            color_alpha,
            color_bg_alive: args.color_bg_alive,
            color_bg_dead: args.color_bg_dead,
            fading_speed: args.fading_speed,
            rainbow,
            size: Point::new(0, 0),
            survival_rule: args.rules.survival.clone(),
        }
    }
}

impl World {
    pub fn resize(&mut self, size: &Point) {
        self.size = size.clone();

        if let Some(rainbow) = &mut self.rainbow {
            rainbow.resize(size);
        }
        self.setup_cells();
    }

    pub fn setup_cells(&mut self) {
        self.setup_blank();

        let cell_image = CellImage::from(self.cell_setup.clone());
        let cell_image_pos = self.size.half() - cell_image.size.half();

        for p in &cell_image.living_points {
            let point = &cell_image_pos + p;
            if point.is_contained(&self.size) {
                self.set_alive_p(point);
            }
        }
    }

    pub fn set_alive(&mut self, i: usize) {
        let bg = if let Some(rainbow) = &self.rainbow {
            let point = Point::new(i as i32 % self.size.width(), i as i32 / self.size.width());
            rainbow.at(point)
        } else {
            self.color_bg_alive.clone()
        };

        let cell = Cell {
            alive: true,
            color: Color {
                fg: Rgba::default(),
                bg,
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
                fg: Rgba::default(),
                bg: self.color_bg_dead.clone(),
            },
        };

        self.cells[i] = cell;
    }

    pub fn set_dead_fading(&mut self, i: usize) {
        self.cells[i].alive = false;
        self.cells[i]
            .color
            .bg
            .fade(&self.color_bg_dead, self.fading_speed);
    }

    fn setup_blank(&mut self) {
        let dead_cell = Cell {
            alive: false,
            color: Color {
                fg: Rgba::default(),
                bg: self.color_bg_dead.clone(),
            },
        };
        self.cells = vec![dead_cell; (self.size.width() * self.size.height()) as usize];
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
                self.set_dead_fading(i);
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
