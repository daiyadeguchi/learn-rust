#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // using tuple
    // let rect1 = (30, 50);
    // println!("The are of rect. is {} square pixels", area(rect1));
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // println!("rect1 is {:?}", rect1);
    // println!("area of rect is {} square pixels", area(&rect1));
    dbg!(&rect1);

}

// tuple area
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}