use super::money::*;
use uuid::Uuid;

/// Represent an account with `T` as its currency
pub struct Account<T> {
    /// The unique id of this account
    id: Uuid,
    /// The name of the owner of this account
    name: String,
    /// The amount of money storred in this account,
    /// in the currency of this account
    money: T,
}

/// Object representation of an account.
/// Every transactions should be done using dollar exchange_rate (1).
/// What tha means is the account assume that the amount given in argument
/// is always in dollar, and the returned value is also in dollar.
impl<T: Money> Account<T> {
    /// Create a new account
    ///
    /// # Argument
    /// * `name` - the name of the owner for this new account
    pub fn new(name: &str) -> Self {
        Self::with_id(name, &Uuid::new_v4())
    }
    /// Create a new account with a given id
    ///
    /// # Arguments
    /// * `name` - the name of the owner for this new account
    /// * `id` - the id which the new account will be using
    pub fn with_id(name: &str, id: &Uuid) -> Self {
        Self::with_amount_and_id(name, 0., id)
    }
    /// Create a new account with a given amount of money
    ///
    /// # Arguments
    /// * `name` - the name of the owner for this new account
    /// * `money` - an amount of money which will be converted into the currency of this account
    pub fn with_amount(name: &str, money: f64) -> Self {
        Self::with_amount_and_id(name, money, &Uuid::new_v4())
    }
    /// Create a new account with a given id and amount of money
    ///
    /// # Arguments
    /// * `name` - the name of the owner for this new account
    /// * `id` - the id which the new account will be using
    /// * `money` - an amount of money which will be converted into the currency of this account
    pub fn with_amount_and_id(name: &str, money: f64, id: &Uuid) -> Self {
        Self {
            id: *id,
            name: name.to_string(),
            money: T::from(money * T::exchange_rate()),
        }
    }

    /// Retrieve the amount of money storred in the account is the currency of the account.
    /// /!\ This is not the same as `get_value` as `get_value` retrieve the amount of money
    /// storred in this account converted into dollar.
    pub fn get_amount(&self) -> f64 {
        self.money.amount()
    }

    /// Get the amount of money stored in this account converted into american dollar
    pub fn get_value(&self) -> f64 {
        self.money.value()
    }
    /// Get the name of the owner of the account
    pub fn get_name(&self) -> &str {
        &self.name
    }
    /// Get the id of the account
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    /// Add the given amount of money into this account. 
    /// The amount is assumed to be in dollar.
    ///
    /// # Arguments
    /// * `amount` - an amount of money in dollar which will be converted into the currency of this account
    pub fn add_money(&mut self, amount: f64) {
        self.money.add(amount);
    }

    /// Retrieve money from this account. The passed amount should be in dollar.
    /// The returned value is also in dollar.
    ///
    /// # Arguments
    /// * `amount` - an amount of money to retrieve from this acount.
    ///
    /// # Returns
    /// The effective amount of money retrieved from this account.
    pub fn retrieve_money(&mut self, amount: f64) -> f64 {
        self.money.remove(amount);
        amount
    }

    /// Rename the account.
    /// In case someone want to change it's name ;)
    pub fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

#[cfg(test)]
mod account_tests {
    use super::*;
    #[test]
    fn create_only_name() {
        let account = Account::<Euro>::new("account");
        assert_eq!(account.get_value(), 0.);
        assert_eq!(account.get_name(), "account");
    }

    #[test]
    fn create_with_id() {
        let id = Uuid::new_v4();
        let account = Account::<Euro>::with_id("account", &id);
        assert_eq!(*account.get_id(), id);
        assert_eq!(account.get_value(), 0.);
        assert_eq!(account.get_name(), "account");

    }

    #[test]
    fn transactions() {
        let mut account = Account::<Euro>::new("account");
        let pocket = account.retrieve_money(100.);
        assert_eq!(pocket, 100.);
        assert_eq!(account.get_value(), -100.);
        account.add_money(200.);
        assert_eq!(account.get_value(), 100.);
        let pocket = account.retrieve_money(50.);
        assert_eq!(pocket, 50.);
        assert_eq!(account.get_value(), 50.);
    }

    #[test]
    fn rename() {
        let mut account = Account::<Euro>::new("account");
        account.rename("new_name");
        assert_eq!(account.get_name(), "new_name");
    }
}
