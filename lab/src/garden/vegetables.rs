enum F_Product<'a> {
    Carrot(&'a str),
    Radish(&'a str),
    Cabbage(&'a str),
}

impl<'a> F_Product<'a> {
    pub fn price(product: &F_Product) -> i32 {
        match product {
            F_Product::Carrot(_) => 10,
            F_Product::Radish(_)=> 20,
            F_Product::Cabbage(_)=> 30,
        }
    }   
}


