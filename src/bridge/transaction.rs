use super::CTransactionId;
use crate::{
    AccountId, Client, PublicKey, SecretKey, Transaction, TransactionCreateAccount,
    TransactionCryptoTransfer, TransactionCryptoUpdate, TransactionResponse,
};
use libc::c_char;
use std::{ffi::CStr, mem};

// Transaction
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_set_operator(
    tx: *mut Transaction<()>,
    operator: AccountId,
) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.operator(operator);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_set_node(tx: *mut Transaction<()>, node: AccountId) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.node(node);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_set_memo(
    tx: *mut Transaction<()>,
    memo: *const c_char,
) {
    debug_assert!(!tx.is_null());
    debug_assert!(!memo.is_null());

    let memo = CStr::from_ptr(memo).to_string_lossy();

    let mut tx = Box::from_raw(tx);
    tx.memo(memo);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_sign(tx: *mut Transaction<()>, secret: SecretKey) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.sign(secret);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_execute(
    tx: *mut Transaction<()>,
    out: *mut CTransactionId,
) -> u64 {
    debug_assert!(!tx.is_null());

    *out = try_ffi!(Box::from_raw(tx).execute()).id.into();

    0
}

// TransactionCreateAccount
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__create_account__new(
    client: *mut Client,
) -> *mut Transaction<TransactionCreateAccount> {
    debug_assert!(!client.is_null());

    let client = Box::from_raw(client);

    let tx = Transaction::create_account(&client);
    let tx = Box::new(tx);

    mem::forget(client);

    Box::into_raw(tx)
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__create_account__set_key(
    tx: *mut Transaction<TransactionCreateAccount>,
    public: PublicKey,
) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.key(public);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__create_account__set_initial_balance(
    tx: *mut Transaction<TransactionCreateAccount>,
    balance: u64,
) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.initial_balance(balance);

    mem::forget(tx);
}

// TransactionCryptoTransfer
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_transfer__new(
    client: *mut Client,
) -> *mut Transaction<TransactionCryptoTransfer> {
    debug_assert!(!client.is_null());

    let client = Box::from_raw(client);

    let tx = Transaction::crypto_transfer(&client);
    let tx = Box::new(tx);

    mem::forget(client);

    Box::into_raw(tx)
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_transfer__add_transfer(
    tx: *mut Transaction<TransactionCryptoTransfer>,
    id: AccountId,
    amount: i64,
) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.transfer(id, amount);

    mem::forget(tx);
}

// TransactionCryptoUpdate
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_update__new(
    client: *mut Client,
) -> *mut Transaction<TransactionCryptoUpdate> {
    debug_assert!(!client.is_null());

    let client = Box::from_raw(client);

    let tx = Transaction::crypto_update(&client);
    let tx = Box::new(tx);

    mem::forget(client);

    Box::into_raw(tx)
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_update__set_account_id_to_update(
    tx: *mut Transaction<TransactionCryptoUpdate>,
    id: AccountId,
) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.account_id_to_update(id);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_update__set_key(
    tx: *mut Transaction<TransactionCryptoUpdate>,
    public: PublicKey,
) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.key(public);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_update__set_proxy_account_id(
    tx: *mut Transaction<TransactionCryptoUpdate>,
    proxy: AccountId,
) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.proxy_account_id(proxy);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_update__set_proxy_fraction(
    tx: *mut Transaction<TransactionCryptoUpdate>,
    fraction: i32,
) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.proxy_fraction(fraction);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_update__set_send_record_threshold(
    tx: *mut Transaction<TransactionCryptoUpdate>,
    threshold: u64,
) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.send_record_threshold(threshold);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_update__set_receive_record_threshold(
    tx: *mut Transaction<TransactionCryptoUpdate>,
    threshold: u64,
) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.receive_record_threshold(threshold);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_update__set_auto_renew_period(
    tx: *mut Transaction<TransactionCryptoUpdate>,
    duration: Duration,
) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.auto_renew_period(duration);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction__crypto_update__set_expiration_time(
    tx: *mut Transaction<TransactionCryptoUpdate>,
    time: Timestamp,
) {
    debug_assert!(!tx.is_null());

    let mut tx = Box::from_raw(tx);
    tx.expiration_time(time);

    mem::forget(tx);
}
