use polygon::Polygon;
use polygon::Rectangle;

mod polygon;

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
