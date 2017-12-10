use piston::input::RenderArgs;

pub struct Point2D {
    pub x: f64,
    pub y: f64
}

impl Point2D {
    pub fn scale(&self, width: f64, height: f64) -> Point2D {
        Point2D {
            x: self.x * width,
            y: self.y * height
        }
    }

    pub fn scale_to_args(&self, args: &RenderArgs) -> Point2D {
        self.scale(args.width as f64, args.height as f64)
    }

    pub fn distance_between(p1: &Point2D, p2: &Point2D) -> f64 {
        let (dx, dy) = (p1.x - p2.x, p1.y - p2.y);

        let distances_sqr = dx.powi(2) + dy.powi(2);

        distances_sqr.sqrt()
    }

    pub fn distance_to_point(&self, other: &Point2D) -> f64 {
        Point2D::distance_between(self, other)
    }


}