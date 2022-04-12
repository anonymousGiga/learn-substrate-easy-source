trait MyTrait {
    fn info(&self);
}

struct PlaceHolder();

impl PlaceHolder {
    fn method(&self) {
        println!("This is method.");
    }
}

impl MyTrait for PlaceHolder {
    fn info(&self) {
        println!("This is info method.");
    }
}

fn main() {
    let p = PlaceHolder();
    p.method();
    p.info();
}
