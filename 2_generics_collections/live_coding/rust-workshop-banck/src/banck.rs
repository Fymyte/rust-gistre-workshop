use std::{collections::HashMap, fmt};
use uuid::Uuid;

use super::account::*;
use super::money::*;

#[derive(Debug, Clone)]
pub struct NoSuchAccountError {
    id: Uuid,
}

impl NoSuchAccountError {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

impl fmt::Display for NoSuchAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no such account registered in this banck: {}", self.id)
    }
}

pub struct Banck<T> {
    accounts: HashMap<Uuid, Account<T>>,
}

impl<T: Money> Banck<T> {
    fn get_account_mut(
        &mut self,
        account_id: &Uuid,
    ) -> Result<&mut Account<T>, NoSuchAccountError> {
        self.accounts
            .get_mut(account_id)
            .ok_or(NoSuchAccountError::new(*account_id))
    }

    /// Retrieve the account with the given id
    ///
    /// # Arguments
    /// * `account_id` - The id of the account to  retrieve
    ///
    /// # Returns
    /// A result which contains the account if it was found is this banck, an error otherwise.
    pub fn get_account(&self, account_id: &Uuid) -> Result<&Account<T>, NoSuchAccountError> {
        self.accounts
            .get(account_id)
            .ok_or(NoSuchAccountError::new(*account_id))
    }

    /// Create a new banck without any account.
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    /// Add a new account to this banck.
    ///
    /// # Arguments
    /// * `account` - The account to add to the banck
    ///
    /// # Returns
    /// The id of the added account.
    pub fn add_account(&mut self, account: Account<T>) -> Uuid {
        let id = *account.get_id();
        self.accounts.insert(*account.get_id(), account);
        id
    }

    /// Add money to the given account.
    ///
    /// # Arguments
    /// * `account_id` - The id of the account to add money to.
    /// * `amount` - The amount of money to add.
    ///
    /// # Returns
    /// A result with nothing on success, or an error if the account was not found
    /// is this banck.
    pub fn add_account_money(
        &mut self,
        account_id: &Uuid,
        amount: f64,
    ) -> Result<(), NoSuchAccountError> {
        Ok(self.get_account_mut(account_id)?.add_money(amount))
    }

    /// Retrieve money from the given account.
    ///
    /// # Arguments
    /// * `account_id` - The id of the account to take money from.
    /// * `amount` - The amount of money to retrieve from this account.
    ///
    /// # Returns
    /// A result containing the amount of money retrieved, or an error if
    /// the account was not found in this banck.
    pub fn retrieve_account_money(
        &mut self,
        account_id: &Uuid,
        amount: f64,
    ) -> Result<f64, NoSuchAccountError> {
        Ok(self.get_account_mut(account_id)?.retrieve_money(amount))
    }

    /// Get the amount of money storred in the given account.
    ///
    /// # Arguments
    /// * `account_id` - The id of the account to get the money from.
    ///
    /// # Return
    /// A result containing the amount of money from the given account, or and error
    /// if the account was not found in this banck.
    pub fn get_account_money(&self, account_id: &Uuid) -> Result<f64, NoSuchAccountError> {
        Ok(self.get_account(account_id)?.get_value())
    }

    /// Rename the account.
    ///
    /// > Just in case someone wants to change name.
    pub fn rename_account(
        &mut self,
        account_id: &Uuid,
        new_name: &str,
    ) -> Result<(), NoSuchAccountError> {
        Ok(self.get_account_mut(account_id)?.rename(new_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_account() {
        let mut banck: Banck<Euro> = Banck::new();
        let id = Uuid::new_v4();
        banck.add_account(Account::with_id("account", &id));

        assert!(banck.get_account(&id).is_ok());
        assert_eq!(banck.get_account_mut(&id).unwrap().get_name(), "account");
    }

    #[test]
    fn get_fake_account() {
        let mut banck: Banck<Dollar> = Banck::new();
        let id = Uuid::new_v4();
        banck.add_account(Account::with_id("account", &id));

        assert!(banck.get_account(&Uuid::new_v4()).is_err());
        let id = Uuid::new_v4();
        assert_eq!(banck.get_account(&id).err().unwrap().id, id);
    }

    #[test]
    fn add_money_real_account() {
        let mut banck: Banck<Ouguiya> = Banck::new();
        let id = Uuid::new_v4();
        banck.add_account(Account::with_id("account", &id));

        assert!(banck.add_account_money(&id, 10.).is_ok());
        assert_eq!(banck.get_account_money(&id).unwrap(), 10.);
    }

    #[test]
    fn add_money_fake_account() {
        let mut banck = Banck::<Euro>::new();
        assert!(banck.add_account_money(&Uuid::new_v4(), 10.).is_err());
    }

    #[test]
    fn retrieve_money_real_account() {
        let mut banck = Banck::<Euro>::new();
        let id = Uuid::new_v4();
        banck.add_account(Account::with_id("account", &id));

        assert!(banck.add_account_money(&id, 10.).is_ok());
        assert_eq!(banck.retrieve_account_money(&id, 5.).unwrap(), 5.);
        assert_eq!(banck.get_account_money(&id).unwrap(), 5.);
    }

    #[test]
    fn retrieve_money_fake_account() {
        let mut banck: Banck<Dollar> = Banck::new();
        assert!(banck
            .retrieve_account_money(&Uuid::new_v4(), 10.)
            .is_err());
    }

    #[test]
    fn rename_real_account() {
        let mut banck: Banck<Dollar> = Banck::new();
        let id = Uuid::new_v4();
        banck.add_account(Account::with_id("account", &id));

        assert!(banck.rename_account(&id, "hello").is_ok());
        assert_eq!(banck.get_account(&id).unwrap().get_name(), "hello");
    }

    #[test]
    fn rename_fake_account() {
        let mut banck: Banck<Ouguiya> = Banck::new();
        assert!(banck.rename_account(&Uuid::new_v4(), "hello").is_err());
    }
}
