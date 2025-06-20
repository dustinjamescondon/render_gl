use crate::camera::{Point2f, Vector2f};

pub struct Rect {
    pub min: Point2f,
    pub width : f32,
    pub height: f32,
}

impl Rect {
    pub fn new(min: Point2f, width: f32, height: f32) -> Self {
        Self {
            min, 
            width, 
            height
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
    pub fn top_right(&self) -> Point2f
    {
        self.min + self.height * Vector2f::y()
                    + self.width  * Vector2f::x()
    }

}