pub mod dollar;
pub mod euro;
pub mod ouguiya;

pub use dollar::Dollar;
pub use euro::Euro;
pub use ouguiya::Ouguiya;

/// Represent a currency
pub trait Money {
    /// Get the exchange rate from this currency into american dollar
    fn exchange_rate() -> f64 where Self: Sized;
    /// Retrieve the amount of money storred in this currency
    fn amount(&self) -> f64;
    /// Return the value of money storred in this currency, converted into american dollar
    ///
    /// # Example
    /// ```
    /// let rate = 1.17; // Dollar to Euro rate
    /// let amount = 12.1; // Euro value
    /// let value = amount * rate; // amount Euro = value dollar  
    /// ```
    fn value(&self) -> f64;
    /// Add the amount of money stored in `other` converted into this currency
    fn add(&mut self, other: f64);
    /// Remove the amount of money stored in `other` converted into this currency
    fn remove(&mut self, other: f64);

    /// Allow construction of a money object from a dollar value.
    fn from(other: f64) -> Self where Self: Sized;
}
