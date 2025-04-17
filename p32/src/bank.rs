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
    pub users: Vec<User>,
    pub name: String,
    pub credit_interest: u64,
    pub debit_interest: u64,
}

impl Bank {
    /// Creates a new bank with the given name, credit interest, and debit interest.
    pub fn new(name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Bank {
            users: Vec::new(),
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
        self.users.push(user);
    }

    /// Gets a user by their name.
    pub fn get_user(&self, name: &str) -> Option<&User> {
        self.users.iter().find(|user| user.name == name)
    }

    /// Gets a mutable reference to a user by their name.
    pub fn get_user_mut(&mut self, name: &str) -> Option<&mut User> {
        self.users.iter_mut().find(|user| user.name == name)
    }

    /// Gets the total assets and total liabilities for the bank.
    pub fn calc_balance(&self) -> (i64, i64) {
        let mut total_liabilities = 0;
        let mut total_assets = 0;

        for user in &self.users {
            if user.balance < 0 {
                total_assets -= user.balance;
            } else {
                total_liabilities += user.balance;
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

        if from.balance + (from.credit_line as i64) < (amount as i64) {
            return Err("Insufficient credit limit".to_string());
        }

        let to_mut = self.get_user_mut(to_user).ok_or("To user not found")?;
        to_mut.balance += amount as i64;
        let from_mut = self
            .get_user_mut(from_user)
            .expect("This should not be reached, because this user was checked earlier");
        from_mut.balance -= amount as i64;

        Ok(())
    }

    /// Accrues interest on the user balances.
    pub fn accrue_interest(&mut self) {
        let credit_interest = self.credit_interest;
        let debit_interest = self.debit_interest;
        let mut interest: i64;
        for user in &mut self.users {
            interest = if user.balance < 0 {
                credit_interest
            } else {
                debit_interest
            } as i64;
            user.balance += user.balance * interest / 10000;
        }
    }

    /// Merges two banks into one.
    pub fn merge_bank(&mut self, other: Bank) {
        // Check if user has account in both banks.
        for user in other.users {
            if let Some(existing_user) = self.get_user_mut(&user.name) {
                existing_user.balance += user.balance;
                existing_user.credit_line += user.credit_line;
            } else {
                self.users.push(user);
            }
        }
    }
}
