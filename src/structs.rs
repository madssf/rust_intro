#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


pub fn run() {
    let rect1 = Rectangle {
        width: 12,
        height: 11,
    };
    println!("The area is {}", rect1.area());
    println!("Sides are {}, {}", rect1.width, rect1.height);
    println!("rect1 is: {:#?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30*scale),
        height: 50,
    };
    dbg!(&rect2);
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

}

