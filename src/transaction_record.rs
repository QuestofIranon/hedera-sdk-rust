use std::{fmt, str::FromStr};

use chrono::{DateTime, Duration, Utc};
use failure::Error;
use itertools::Itertools;

use crate::{
    error::ErrorKind,
    proto::{self, ToProto},
    AccountId,
    timestamp::Timestamp
};
use crate::TransactionId;
use query_interface::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionRecord {
    pub account_id: AccountId,
    pub transaction_hash: Vec<u8>,
    pub consensus_timestamp: Option<Timestamp>,
    pub transaction_id: Option<TransactionId>,
    pub memo: Option<String>,
    pub transaction_fee: u64,
    pub(crate) inner: Box<dyn Object>,
    phantom: PhantomDate<T>
}

/*
id: Option<TransactionId>,
client: Arc<grpc::Client>,
node: Option<AccountId>,
secrets: Vec<SecretKey>,
memo: Option<String>,
pub(crate) inner: Box<dyn Object>,
phantom: PhantomData<T>,
*/

/*
pub struct TransactionRecord {
    // message fields
    pub receipt: ::protobuf::SingularPtrField<super::TransactionReceipt::TransactionReceipt>,
    pub transactionHash: ::std::vec::Vec<u8>,
    pub consensusTimestamp: ::protobuf::SingularPtrField<super::Timestamp::Timestamp>,
    pub transactionID: ::protobuf::SingularPtrField<super::BasicTypes::TransactionID>,
    pub memo: ::std::string::String,
    pub transactionFee: u64,
    // message oneof groups
    pub body: ::std::option::Option<TransactionRecord_oneof_body>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
*/

/*
    // message fields
    pub transactionID: ::protobuf::SingularPtrField<super::BasicTypes::TransactionID>,
    pub nodeAccountID: ::protobuf::SingularPtrField<super::BasicTypes::AccountID>,
    pub transactionFee: u64,
    pub transactionValidDuration: ::protobuf::SingularPtrField<super::Duration::Duration>,
    pub generateRecord: bool,
    pub memo: ::std::string::String,
    // message oneof groups
    pub data: ::std::option::Option<TransactionBody_oneof_data>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
*/

impl fmt::Display for TransactionRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}->{}#{}::{}@{}.{}",
            self.account_id,
            self.transaction_id,
            self.transaction_hash,
            self.memo,
            self.consensus_timestamp,
            self.transaction_fee
        )
    }
}

impl FromStr for TransactionRecord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::timestamp::Timestamp;

        if let Some((account_id, timestamp)) = s.split('@').next_tuple() {
            Ok(Self {
                account_id: account_id.parse()?,
                transaction_valid_start: Timestamp::from_str(timestamp)?.into(),
            })
        } else {
            let b = hex::decode(s)?;

            let mut pb: crate::proto::BasicTypes::TransactionID =
                protobuf::parse_from_bytes(b.as_slice())
                    .map_err(|_| ErrorKind::Parse("{realm}:{shard}:{account}@{seconds}.{nanos}"))?;

            Ok(Self {
                account_id: pb.take_accountID().into(),
                transaction_valid_start: pb.take_transactionValidStart().into(),
            })
        }
    }
}

impl ToProto<proto::TransactionRecord::TransactionRecord> for TransactionRecord {
    fn to_proto(&self) -> Result<proto::TransactionRecord::TransactionRecord, Error> {
        let mut transaction_record = proto::TransactionRecord::TransactionRecord::new();
        transaction_record.set_transactionValidStart(self.transaction_valid_start.to_proto()?);
        transaction_record.set_accountID(self.account_id.to_proto()?);

        Ok(transaction_record)
    }
}
