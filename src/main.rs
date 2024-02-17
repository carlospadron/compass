use compass::coordinate;

fn main() {
    let d2 = coordinate::Cooordinate::D2 { x: 1.0, y: 2.0 };
    let d3 = coordinate::Cooordinate::D3 { x: 1.0, y: 2.0, z: 3.0 };
    println!("d2: x = {}, y = {}", d2.x(), d2.y());
    println!("d3: x = {}, y = {}, z = {}", d3.x(), d3.y(), d3.z());
}