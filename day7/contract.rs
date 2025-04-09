struct Contract {
    owner: String,
    balance: u64,
}

impl Contract {
    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: u64) -> Result<(), &'static str> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds")
        }
    }
}
