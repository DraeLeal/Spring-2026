#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount{
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount >= 0.0{
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount >= 0.0 && amount <= self.balance{
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let account = BankAccount::new(100.0);
        account.withdraw(40.0);
        assert_eq!(account.balance(), 60.0);
    }

    // Add more tests here
    #[test]
    fn test_balance(){
        let account = BankAccount::new(75.7);
        assert_eq!(account.balance(), 75.5);
    }

    #[test]
    fn test_edge_cases(){
        let account = BankAccount::new(100.0);

        account.deposit(-50.0);
        assert_eq!(account.balance(), 100.0);

        account.withdraw(-20.0);
        assert_eq!(account.balance(), 100.0);

        account.withdraw(150.0);
        assert_eq!(account.balance(), 100.0);
        
    }
}

fn main(){

    let mut account = BankAccount::new(100.0);
    println!("Initial balance: {}", account.balance());

    account.deposit(50.0);
    println!("After deposit 50: {}", account.balance());

    account.withdraw(30.0);
    println!("After withdrawing 30: {}", account.balance());

    account.withdraw(200.0);
    println!("After attempting to withdraw 200: {}", account.balance());
}