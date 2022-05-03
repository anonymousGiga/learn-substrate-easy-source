trait Parent{
    fn parent_function(&self);
}

trait Son: Parent{
    fn son_function(&self);
}

struct MyType;

impl Son for MyType {
    fn son_function(&self) {
        println!("Son function!");
    }
}

impl Parent for MyType {
    fn parent_function(&self) {
        println!("Parent function!");
    }
}

fn main() {
    let s = MyType;
    s.son_function();
}
