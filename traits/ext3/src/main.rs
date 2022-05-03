trait Drive{
    fn drive(&self);
}

struct Truck;

impl Drive for Truck {
    fn drive(&self) {
        println!("Truck run!");
    }
}


struct MotorCycle;

impl Drive for MotorCycle {
    fn drive(&self) {
        println!("MotorCycle run!");
    }
}

fn use_transportation(t: Box<dyn Drive>) {
   t.drive(); 
}

fn main() {
    let truck = Truck;
    use_transportation(Box::new(truck));

    let moto = MotorCycle;
    use_transportation(Box::new(moto));
}