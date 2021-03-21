use crate::currency::Currency;

pub type ClientId = u16;
pub type TxId = u32;

pub enum Transaction {
    Withdraw {
        client: ClientId,
        tx: TxId,
        amount: Currency,
    },
    Deposit {
        client: ClientId,
        tx: TxId,
        amount: Currency,
    },
    Dispute {
        client: ClientId,
        tx: TxId,
    },
    Resolve {
        client: ClientId,
        tx: TxId,
    },
    Chargeback {
        client: ClientId,
        tx: TxId,
    },
}
