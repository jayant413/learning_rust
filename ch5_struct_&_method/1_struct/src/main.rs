#[derive(Debug)]
struct Triangle {
    width: u32,
    height: u32,
}

fn main() {
    let triangle = Triangle { width: 10, height: 20 };
    let area = calculate_area(&triangle);
    println!("area of {triangle:#?} is {area}");
    dbg!(&triangle);
}


fn calculate_area(triangle: &Triangle) -> u32 {
    triangle.width * triangle.height / 2
}