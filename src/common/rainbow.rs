use term2d::model::point::Point;
use term2d::model::rgba::Rgba;

const CORRECTION_FACTOR_X: f32 = 0.5;
const CORRECTION_FACTOR_Y: f32 = 1.0;

#[derive(Debug)]
pub struct Rainbow {
    colors: Vec<Rgba>,
    normal: (f32, f32),
    normalized_color_distance: f32,
    radius: f32,
    size: Point,
    size_corrected: (f32, f32),
}

impl Rainbow {
    pub fn new(colors: Vec<Rgba>) -> Self {
        let radius = 1_f32;
        let normal = (1_f32, 0_f32);
        let normalized_color_distance = 1_f32 / (colors.len() as f32 - 1_f32);
        let size = Point::new(1, 0);
        let size_corrected = (1_f32, 0_f32);
        Self {
            colors,
            normal,
            normalized_color_distance,
            radius,
            size,
            size_corrected,
        }
    }

    pub fn resize(&mut self, size: &Point) {
        self.size = size.clone();
        self.size_corrected = (
            CORRECTION_FACTOR_X * size.x as f32,
            CORRECTION_FACTOR_Y * size.y as f32,
        );
        self.radius = ((self.size_corrected.0 * self.size_corrected.0
            + self.size_corrected.1 * self.size_corrected.1) as f32)
            .sqrt();
        self.normal = (
            self.size_corrected.0 as f32 / self.radius,
            self.size_corrected.1 as f32 / self.radius,
        );
    }

    pub fn at(&self, point: Point) -> Rgba {
        let point_corrected = (
            CORRECTION_FACTOR_X * point.x as f32,
            CORRECTION_FACTOR_Y * point.y as f32,
        );
        let point_projection = (point_corrected.0 * self.size_corrected.0
            + point_corrected.1 * self.size_corrected.1)
            / (self.radius * self.radius);

        let i = ((self.colors.len() - 1) as f32 * point_projection) as usize;
        let color1 = self.colors[i].clone();
        let color2 = self.colors[i + 1].clone();

        let a = point_projection / self.normalized_color_distance - i as f32;

        let r = ((1_f32 - a) * color1.r as f32 + a * color2.r as f32) as u8;
        let g = ((1_f32 - a) * color1.g as f32 + a * color2.g as f32) as u8;
        let b = ((1_f32 - a) * color1.b as f32 + a * color2.b as f32) as u8;

        Rgba { r, g, b, a: 255 }
    }
}
