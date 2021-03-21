use std::fmt;

use crate::{
    client_info::{ClientInfo, TransactionError},
    transaction::{ClientId, Transaction},
};

/// Since there are so few possible client ids due to the assumption that clients are valid u16's
/// It makes much more sense to simply use a vector instead of using a HashMap for performance
pub struct ClientTable {
    clients: Vec<ClientInfo>,
}

impl ClientTable {
    pub fn new() -> Self {
        Self {
            clients: vec![Default::default(); ClientId::MAX.into()],
        }
    }

    pub fn handle_transaction(&mut self, tx: Transaction) -> Result<(), TransactionError> {
        use Transaction::*;
        #[allow(clippy::unit_arg)]
        match tx {
            Withdraw { client, tx, amount } => self.clients[client as usize].withdraw(amount, tx),
            Deposit { client, tx, amount } => Ok(self.clients[client as usize].deposit(amount, tx)),
            Dispute { client, tx } => self.clients[client as usize].dispute(tx),
            Resolve { client, tx } => self.clients[client as usize].resolve(tx),
            Chargeback { client, tx } => self.clients[client as usize].chargeback(tx),
        }
    }
}

impl fmt::Debug for ClientTable {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_list()
            .entries(self.clients.iter().filter(|c| c.exists()))
            .finish()
    }
}

impl fmt::Display for ClientTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "client, available, held, total, locked")?;
        for c in 0..self.clients.len() {
            if self.clients[c].exists() {
                writeln!(f, "{}, {}", c, self.clients[c])?;
            }
        }
        Ok(())
    }
}
