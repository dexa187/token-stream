use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::v2 as eth;

/**
 * @dev formatting address and txn hash with 0x prefix
 */
pub fn format_with_0x(address: String) -> String {
    format!("0x{}", address)
}

pub fn get_balances(value: BigInt, calls: Vec<eth::Call>) -> (BigInt, BigInt) {
    let mut from_balance = BigInt::from(0);
    let mut to_balance = BigInt::from(0);
    for call in calls {
        for storage_change in call.storage_changes {
            let old_value = BigInt::from_unsigned_bytes_be(&storage_change.old_value);
            let new_value = BigInt::from_unsigned_bytes_be(&storage_change.new_value);
            let mut from = false;
            let mut amount = old_value.clone() - new_value.clone();
            if amount < BigInt::from(0) {
                amount = amount.neg();
                from = true;
            }
            if amount == value {
                if from {
                    from_balance = new_value.clone();
                } else {
                    to_balance = new_value.clone();
                }
            }
        }
    }
    return (from_balance, to_balance);
}
