#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height
    }

    fn add1(&mut self) {
        self.width += 1;
        self.height += 1;
    }

    fn new_square(size: u32) -> Self {
        return Self { width: size, height: size};
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl  Point {
    fn distance(&self, other:&Point) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2);
        let y_squared = f64::powi(other.y - self.y, 2);
        return f64::sqrt(x_squared + y_squared)
    }
}

fn main() {
    let mut rect1 = Rectangle::new_square(30);
    rect1.add1();
    // let area = get_area(&rect1);
    let area = rect1.area();
    println!("rect1 is {:?}, its area is {area}", rect1);

    let p1 = Point{x: 0., y: 0.};
    let p2 = Point{x: 3., y: 4.};
    let distance = p1.distance(&p2);
    println!("the distance during {:?} and {:?} is {distance}.", p1, p2);
}

// fn get_area(rect:&Rectangle) -> u32 {
//     return rect.height*rect.width
// }