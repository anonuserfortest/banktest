use std::{io, num};

use crate::{currency::ParseCurrencyError, transaction::Transaction};

#[derive(Debug)]
pub enum ParseCSVError {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
    ParseCurrencyError(ParseCurrencyError),
    UnknownRecord,
}

impl From<io::Error> for ParseCSVError {
    fn from(error: io::Error) -> Self {
        ParseCSVError::IoError(error)
    }
}

impl From<num::ParseIntError> for ParseCSVError {
    fn from(error: num::ParseIntError) -> Self {
        ParseCSVError::ParseIntError(error)
    }
}

impl From<ParseCurrencyError> for ParseCSVError {
    fn from(error: ParseCurrencyError) -> Self {
        ParseCSVError::ParseCurrencyError(error)
    }
}

impl From<ParseCSVError> for io::Error {
    fn from(error: ParseCSVError) -> Self {
        io::Error::new(io::ErrorKind::InvalidInput, format!("{:?}", error))
    }
}

pub fn parse_line(line: io::Result<String>) -> Result<Transaction, ParseCSVError> {
    let line = line?;
    let mut fields = line.split(',').map(|f| f.trim());
    let transaction_type = fields.next();
    let client = fields.next();
    let tx_id = fields.next();
    let amount = fields.next();
    use Transaction::*;
    match (transaction_type, client, tx_id, amount) {
        (Some("withdrawal"), Some(client), Some(tx_id), Some(amount)) => {
            Ok(Transaction::Withdraw {
                client: client.parse()?,
                tx: tx_id.parse()?,
                amount: amount.parse()?,
            })
        }
        (Some("deposit"), Some(client), Some(tx_id), Some(amount)) => Ok(Deposit {
            client: client.parse()?,
            tx: tx_id.parse()?,
            amount: amount.parse()?,
        }),
        (Some("dispute"), Some(client), Some(tx_id), _) => Ok(Dispute {
            client: client.parse()?,
            tx: tx_id.parse()?,
        }),
        (Some("resolve"), Some(client), Some(tx_id), _) => Ok(Resolve {
            client: client.parse()?,
            tx: tx_id.parse()?,
        }),
        (Some("chargeback"), Some(client), Some(tx_id), _) => Ok(Chargeback {
            client: client.parse()?,
            tx: tx_id.parse()?,
        }),
        _ => Err(ParseCSVError::UnknownRecord),
    }
}
