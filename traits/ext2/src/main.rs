use std::fmt::{Error, Formatter};
impl std::fmt::Debug for () {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
