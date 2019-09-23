pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub trait Polygon {
    fn get_vertices(&self) -> Vec<Vec<f32>>;
}

impl Polygon for Rectangle {
    fn get_vertices(&self) -> Vec<Vec<f32>> {
        let mut points: Vec<Vec<f32>> = Vec::new();
        points.push(vec![self.x, self.y]);
        points.push(vec![self.x + self.width, self.y]);
        points.push(vec![self.x + self.width, self.y + self.height]);
        points.push(vec![self.x, self.y + self.height]);
        points
    }
}
