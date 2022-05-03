trait MyTrait{
    type MyType;
    fn function(&self, v: Self::MyType) -> Self::MyType;
}

struct MyStruct(u8);

impl MyTrait for MyStruct {
    type MyType = u8;
    fn function(&self, v: Self::MyType) -> Self::MyType {
        v
    }
}

fn main() {
    let a = MyStruct(1);
    let ret = a.function(a.0);
    println!("ret is {:?}", ret);
}
