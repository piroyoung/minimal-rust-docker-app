use shape::Polygon;
use shape::Rectangle;

mod shape;

fn main() {
    let rect = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };
    rect.get_vertices().iter().for_each(|point| {
        println!("{0},{1}", point[0], point[1])
    });
}
