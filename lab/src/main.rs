#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn new() -> Rect{
        Rect {
            width: 9,
            height: 10
        }
    }
}



static QUANTITY: Rect = Rect { width: 10, height: 10 };
fn dangle() -> &'static Rect{
    &QUANTITY
}


fn main() {
    dangle();
}



