pub mod dollar;
pub mod euro;
pub mod ouguiya;

pub use dollar::Dollar;
pub use euro::Euro;
pub use ouguiya::Ouguiya;

/// Represent a currency
pub trait Money: From<f64> {
    /// Get the exchange rate from this currency into american dollar
    fn exchange_rate() -> f64;
    /// Retrieve the amount of money storred in this currency
    fn amount(&self) -> f64;
    /// Return the value of money storred in this currency, converted into american dollar
    ///
    /// # Example
    /// This function basically perform
    /// ```
    /// self.amount * Self::exchange_rate
    /// ```
    fn value(&self) -> f64;
    /// Add the amount of money stored in `other` converted into this currency
    ///
    /// # Example
    /// This function basically perform
    /// ```
    /// self.amount += other.value() * Self::exchange_rate();
    /// ```
    fn add(&mut self, other: f64);
    /// Remove the amount of money stored in `other` converted into this currency
    ///
    /// # Example
    /// This function basically perform
    /// ```
    /// self.amount -= other.value() * Self::exchange_rate();
    /// ```
    fn remove(&mut self, other: f64);
}
