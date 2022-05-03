trait SystemConfig {
    fn system_configure(&self) {
        println!("system configure.");
    }
}

trait Config: SystemConfig {
    type Event: ToString;
    type Balance: ToString;
    type Currency: ToString;

    fn configure_event(&self, event: Self::Event);
    fn configure_balance(&self, balance: Self::Balance);
    fn configure_currency(&self, currency: Self::Currency);
}

struct Pallet {
    event: u64,
    balance: String,
    currency: String,
}

impl SystemConfig for Pallet {}

impl Config for Pallet {
    type Event = u64;
    type Balance = String;
    type Currency = String;
    fn configure_event(&self, event: Self::Event) {
        println!("configure, event is: {:?}", event);
    }
    fn configure_balance(&self, balance: Self::Balance) {
        println!("configure, balance is: {:?}", balance);
    }
    fn configure_currency(&self, currency: Self::Currency) {
        println!("configure, currency is: {:?}", currency);
    }
}

impl Pallet {
    fn new(event: u64, balance: String, currency: String) -> Self {
        Pallet {event, balance, currency}
    }

    fn init(&self) {
        self.configure_event(self.event);
        self.configure_balance(self.balance.clone());
        self.configure_currency(self.currency.clone());
    }
}

fn main() {
    let my_pallet = Pallet::new(1, "my balance".to_string(), 
    "my currency".to_string());
    my_pallet.init();
}