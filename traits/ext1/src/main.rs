//例子1
pub trait MyTrait {
    fn print(&self);
}

pub struct MyType;

impl MyTrait for MyType {
    fn print(&self) {
        println!("This is ok.");
    }
}

fn main() {
    let t = MyType;
    t.print();
    println!("Hello, world!");
}
