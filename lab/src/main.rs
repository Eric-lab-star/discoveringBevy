#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}





fn main() {
    let x = Rect {
        width: 20,
        height: 10,
    };
    let a = x.area();
    println!("{}", a)
}



