use super::Money;

pub struct Ouguiya {
    amount: f64,
}
impl Ouguiya {
    const RATE: f64 = 0.03;
    pub fn new() -> Self {
        Self::with_amount(0.)
    }

    pub fn with_amount(amount: f64) -> Self {
        Self { amount: amount / Self::RATE }
    }
}

impl Money for Ouguiya {
    fn exchange_rate() -> f64 {
        Self::RATE
    }

    fn amount(&self) -> f64 {
        self.amount
    }

    fn value(&self) -> f64 {
        self.amount * Self::RATE
    }

    fn add(&mut self, other: f64) {
        self.amount += other / Self::exchange_rate();
    }

    fn remove(&mut self, other: f64) {
        self.amount -= other / Self::exchange_rate();
    }

    fn from(other: f64) -> Self {
        Self::with_amount(other)
    }
}
