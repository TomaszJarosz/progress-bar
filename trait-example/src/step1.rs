use std::collections::HashMap;

pub struct BalanceModule {
    balances: HashMap<u32, u32>,
}

impl BalanceModule {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new()
        }
    }

    pub fn set_balances(&mut self, who: u32, amount: u32) {
        self.balances.insert(who, amount);
    }

    pub fn balance(&self, who: u32) -> u32 {
        *self.balances.get(&who).unwrap_or(&0)
    }
}
