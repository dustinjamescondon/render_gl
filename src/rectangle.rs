use crate::camera::{Point2f, Vector2f};

#[derive(Clone)]
pub struct Rect {
    pub min: Point2f,
    pub width : f32,
    pub height: f32,
    unset_flag: bool,
}

impl Rect {
    pub fn new(min: Point2f, width: f32, height: f32) -> Self {
        Self {
            min, 
            width, 
            height,
            unset_flag: false,
        }
    }

    pub fn new_min_max(min: Point2f, max: Point2f) -> Self {
        let dim = max - min;
        Self::new(min, dim.x, dim.y)
    }

    pub fn unset() -> Self {
        Self {
            min: Point2f::origin(),
            width: 0_f32,
            height: 0_f32,
            unset_flag: true
        }
    }

    pub fn union(&self, other: &Rect) -> Self {
        match (self.unset_flag, other.unset_flag) {
            (true, true) => {
                let self_max = self.max();
                let other_max = other.max();
                let min_x = self.min.x.min(other.min.x);
                let min_y = self.min.y.min(other.min.y);
                let max_x = self_max.x.max(other_max.x);
                let max_y = self_max.y.max(other_max.y);
                Self::new_min_max(Point2f::new(min_x, min_y), Point2f::new(max_x, max_y))
            },
            (true, false) => self.clone(),
            (false, true) => other.clone(),
            (false, false) => self.clone(),
        }
    }

    pub fn bottom_left(&self) -> Point2f
    {
        self.min
    }
    pub fn bottom_right(&self) -> Point2f
    {
        self.min + self.width * Vector2f::x()
    }
    pub fn top_left(&self) -> Point2f
    {
        self.min + self.height * Vector2f::y()
    }
    pub fn top_right(&self) -> Point2f {self.max()}

    pub fn max(&self) -> Point2f {
        self.min 
        + self.height * Vector2f::y()
        + self.width  * Vector2f::x()
    }

}