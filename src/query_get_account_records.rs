use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::QueryInner,
    AccountId, Client, ErrorKind, PreCheckCode, Query,
};
use failure::Error;

pub type QueryGetAccountRecordsAnswer = u64;

pub struct QueryGetAccountRecords {
    account: AccountId,
}

impl Query<u64> {
    pub fn get_account_records(client: &Client, account: AccountId) -> Self {
        Self::new(client, QueryGetAccountRecords { account })
    }
}

impl QueryInner for QueryGetAccountRecords {
    type Answer = QueryGetAccountRecordsAnswer;

    fn answer(&self, mut response: proto::Response::Response) -> Result<Self::Answer, Error> {
        let mut response = response.take_cryptogetAccountRecords();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(response.get_balance()),
            code => Err(ErrorKind::PreCheck(code))?,
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetAccountRecords::CryptoGetAccountRecordsQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);

        Ok(Query_oneof_query::cryptogetAccountRecords(query))
    }
}
