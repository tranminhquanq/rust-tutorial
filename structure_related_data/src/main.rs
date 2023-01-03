struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_account: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user_one = User {
        email: String::from("j2team.tranminhquang@gmail.com"),
        name: String::from("Minh Quang"),
        active: true,
        sign_in_account: 12121,
    };

    println!("Email: {}", user_one.email);
    let rect_1 = Rectangle{
        width: 10,
        height: 20
    };
    let rect_2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect_3 = Rectangle {
        width: 60,
        height: 45,
    };
    let square = Rectangle::square(32);
    println!("Square {:?}", square);

    // println!("Width of renct {:?}", rect_1);
    // println!("The area of the rectangle is {} square pixels.", rect_1.area());
    println!("Can rect1 hold rect2? {}", rect_1.can_hold(&rect_2));
    println!("Can rect1 hold rect3? {}", rect_1.can_hold(&rect_3));
}

// fn area(rectangle: &Rectangle) -> u32 {
//     return rectangle.width * rectangle.height;
// }