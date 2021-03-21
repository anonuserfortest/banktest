use std::fmt;

use crate::{currency::Currency, transaction::TxId};

/// ClientInfo is optimized around the assumption that disputes are a lot rarer than normal transactions
/// Thus it uses vectors instead of hashmaps to achieve fast insertions for the common transactions
/// This does means that a dispute takes longer to execute than what might be expected due to having to search the entire vector
/// Dispute follow up transactions(resolve/chargeback) are reletivley cheap as the amount of dispute to search through should be very short
/// If disputes becomes an issue one could dynamically "upgrade" from a vector to a hashmap once some threshhold has been reached
#[derive(Default, Clone, Debug)]
pub struct ClientInfo {
    available_funds: Currency,
    held_funds: Currency,
    locked: bool,
    transfers: Vec<ClientTransaction>,
    disputes: Vec<ClientTransaction>,
}

impl ClientInfo {
    pub fn deposit(&mut self, amount: Currency, tx: TxId) {
        self.available_funds += amount;
        self.transfers.push(ClientTransaction::new(amount, tx));
    }

    pub fn withdraw(&mut self, amount: Currency, tx: TxId) -> Result<(), TransactionError> {
        if self.available_funds <= amount {
            return Err(TransactionError::Overdraw);
        }
        self.available_funds -= amount;
        self.transfers.push(ClientTransaction::new(-amount, tx));
        Ok(())
    }

    pub fn dispute(&mut self, tx: TxId) -> Result<(), TransactionError> {
        for t in &self.transfers {
            if t.tx == tx {
                self.available_funds -= t.amount;
                self.held_funds += t.amount;
                self.disputes.push(ClientTransaction::new(t.amount, t.tx));
                return Ok(());
            }
        }
        Err(TransactionError::InvalidTxId)
    }

    pub fn resolve(&mut self, dispute_tx: TxId) -> Result<(), TransactionError> {
        for d in &self.disputes {
            if d.tx == dispute_tx {
                self.available_funds += d.amount;
                self.held_funds -= d.amount;
                return Ok(());
            }
        }
        Err(TransactionError::InvalidTxId)
    }

    pub fn chargeback(&mut self, dispute_tx: TxId) -> Result<(), TransactionError> {
        for d in &self.disputes {
            if d.tx == dispute_tx {
                self.held_funds -= d.amount;
                self.locked = true;
                return Ok(());
            }
        }
        Err(TransactionError::InvalidTxId)
    }

    pub fn exists(&self) -> bool {
        !self.transfers.is_empty()
    }

    fn total_funds(&self) -> Currency {
        self.available_funds + self.held_funds
    }
}

impl fmt::Display for ClientInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.available_funds,
            self.held_funds,
            self.total_funds(),
            self.locked
        )
    }
}

#[derive(Debug)]
pub enum TransactionError {
    Overdraw,
    InvalidTxId,
}

#[derive(Clone, Copy, Debug)]
pub struct ClientTransaction {
    tx: TxId,
    amount: Currency,
}

impl ClientTransaction {
    fn new(amount: Currency, tx: TxId) -> Self {
        Self { tx, amount }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_deposit() {
        let amount = Currency::new(5000);
        let mut clinfo = ClientInfo::default();
        clinfo.deposit(amount, 1);
        assert_eq!(clinfo.available_funds, amount);
        assert_eq!(clinfo.transfers[0].amount, amount);
        assert_eq!(clinfo.transfers[0].tx, 1);
    }

    #[test]
    fn handle_withdraw() {
        let amount = Currency::new(5000);
        let amount2 = Currency::new(1000);
        let amount3 = Currency::new(4000);
        let mut clinfo = ClientInfo::default();
        clinfo.deposit(amount, 1);
        clinfo.withdraw(amount2, 2).unwrap();
        assert_eq!(clinfo.available_funds, amount3);
        assert_eq!(clinfo.transfers[1].amount, -amount2);
        assert_eq!(clinfo.transfers[1].tx, 2);
    }

    #[test]
    fn handle_withdraw_not_enough_money() {
        let amount = Currency::new(5000);
        let amount2 = Currency::new(6000);
        let mut clinfo = ClientInfo::default();
        clinfo.deposit(amount, 1);
        assert!(clinfo.withdraw(amount2, 2).is_err());
        assert_eq!(clinfo.available_funds, amount);
        assert_eq!(clinfo.transfers.len(), 1);
    }

    #[test]
    fn handle_dispute() {
        let amount = Currency::new(5000);
        let amount0 = Currency::new(0);
        let mut clinfo = ClientInfo::default();
        clinfo.deposit(amount, 1);
        clinfo.dispute(1).unwrap();
        assert_eq!(clinfo.available_funds, amount0);
        assert_eq!(clinfo.held_funds, amount);
        assert_eq!(clinfo.total_funds(), amount);
        assert_eq!(clinfo.disputes[0].amount, amount);
        assert_eq!(clinfo.disputes[0].tx, 1);
    }

    #[test]
    fn handle_resolve() {
        let amount = Currency::new(5000);
        let amount0 = Currency::new(0);
        let mut clinfo = ClientInfo::default();
        clinfo.deposit(amount, 1);
        clinfo.dispute(1).unwrap();
        clinfo.resolve(1).unwrap();
        assert_eq!(clinfo.available_funds, amount);
        assert_eq!(clinfo.held_funds, amount0);
        assert_eq!(clinfo.total_funds(), amount);
    }

    #[test]
    fn handle_chargeback() {
        let amount = Currency::new(5000);
        let amount0 = Currency::new(0);
        let mut clinfo = ClientInfo::default();
        clinfo.deposit(amount, 1);
        clinfo.dispute(1).unwrap();
        clinfo.chargeback(1).unwrap();
        assert_eq!(clinfo.available_funds, amount0);
        assert_eq!(clinfo.held_funds, amount0);
        assert_eq!(clinfo.total_funds(), amount0);
    }
}
