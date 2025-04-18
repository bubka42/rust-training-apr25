use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A struct representing a user with a name, credit line, and balance.
pub struct User {
    pub name: String,
    pub credit_line: u64,
    pub balance: i64,
}

#[derive(Debug, Clone)]
/// A struct representing a bank with a list of users, a name, and interest rates.
pub struct Bank {
    pub users: HashMap<String, User>,
    pub name: String,
    pub credit_interest: u64,
    pub debit_interest: u64,
}

impl Bank {
    /// Creates a new bank with the given name, credit interest, and debit interest.
    pub fn new(name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Bank {
            users: HashMap::new(),
            name,
            credit_interest,
            debit_interest,
        }
    }

    /// Adds a user to the bank with the given name and credit line.
    pub fn add_user(&mut self, name: String, credit_line: u64) {
        let user = User {
            name,
            credit_line,
            balance: 0,
        };
        self.users.insert(user.name.clone(), user);
    }

    /// Gets a user by their name.
    pub fn get_user(&self, name: &str) -> Option<&User> {
        self.users.get(name)
    }

    /// Gets a mutable reference to a user by their name.
    pub fn get_user_mut(&mut self, name: &str) -> Option<&mut User> {
        self.users.get_mut(name)
    }

    /// Gets the total assets and total liabilities for the bank.
    pub fn calc_balance(&self) -> (i64, i64) {
        let mut total_liabilities = 0i64;
        let mut total_assets = 0i64;

        for user in self.users.values() {
            if user.balance < 0 {
                total_assets = total_assets
                    .checked_add(user.balance)
                    .ok_or("Overflow in total assets")
                    .unwrap();
            } else {
                total_liabilities = total_liabilities
                    .checked_add(user.balance)
                    .ok_or("Overflow in total liabilities")
                    .unwrap();
            }
        }

        (total_liabilities, total_assets)
    }

    /// Transfers amount from one user to another.
    pub fn transfer_funds(
        &mut self,
        from_user: &str,
        to_user: &str,
        amount: u64,
    ) -> Result<(), String> {
        let from = self.get_user(from_user).ok_or("From user not found")?;

        if from.balance + i64::try_from(from.credit_line).unwrap() < i64::try_from(amount).unwrap()
        {
            return Err("Insufficient credit limit".to_string());
        }
        let from_balance = from
            .balance
            .checked_sub(i64::try_from(amount).unwrap())
            .ok_or("Underflow in balance")
            .unwrap();

        let to_mut = self.get_user_mut(to_user).ok_or("To user not found")?;
        let to_balance = to_mut
            .balance
            .checked_add(i64::try_from(amount).unwrap())
            .ok_or("Overflow in balance")
            .unwrap();
        to_mut.balance = to_balance;

        let from_mut = self
            .get_user_mut(from_user)
            .expect("This should not be reached, because this user was checked earlier");
        from_mut.balance = from_balance;

        Ok(())
    }

    /// Accrues interest on the user balances.
    pub fn accrue_interest(&mut self) {
        let credit_interest = self.credit_interest;
        let debit_interest = self.debit_interest;
        for user in self.users.values_mut() {
            let interest = if user.balance < 0 {
                credit_interest
            } else {
                debit_interest
            };
            let new_balance = user
                .balance
                .checked_add(user.balance * i64::try_from(interest).unwrap() / 10000)
                .ok_or("Overflow/Underflow in balance")
                .unwrap();
            user.balance = new_balance;
        }
    }

    /// Merges two banks into one.
    pub fn merge_bank(&mut self, other: Bank) {
        // Check if user has account in both banks.
        for user in other.users.values() {
            if let Some(existing_user) = self.get_user_mut(&user.name) {
                let new_balance = existing_user
                    .balance
                    .checked_add(user.balance)
                    .ok_or("Overflow in balance")
                    .unwrap();
                existing_user.balance = new_balance;
                let new_credit_line = existing_user
                    .credit_line
                    .checked_add(user.credit_line)
                    .ok_or("Overflow in credit line")
                    .unwrap();
                existing_user.credit_line = new_credit_line;
            } else {
                self.users.insert(user.name.clone(), user.clone());
            }
        }
    }
}
