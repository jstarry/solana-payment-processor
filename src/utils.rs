const FEE: u64 = 3;

/// Given the expected amount, calculate the fee and take home amount
/// Currently fee is 0.3% with a minimum fee of 1 lamport
/// If the amount is less than 100 lamports the fee is 0
pub fn get_amounts(amount: u64) -> (u64, u64) {
    let mut fee_amount: u64 = 0;
    let mut take_home_amount: u64 = amount;

    if amount >= 100 {
        let possible_fee_amount: u128 = (amount as u128 * FEE as u128) / 1000;
        fee_amount = 1;
        if possible_fee_amount > 0 {
            fee_amount = possible_fee_amount as u64;
        }
        take_home_amount = amount - fee_amount;
    }

    (take_home_amount, fee_amount)
}

#[cfg(test)]
mod test {
    use {super::*, solana_program_test::*};

    #[tokio::test]
    async fn test_get_amounts() {
        assert_eq!((997000000, 3000000), get_amounts(1000000000));
        assert_eq!((1994000, 6000), get_amounts(2000000));
        assert_eq!((1994, 6), get_amounts(2000));
        assert_eq!((100, 1), get_amounts(101));
        assert_eq!((99, 1), get_amounts(100));
        assert_eq!((99, 0), get_amounts(99));
        assert_eq!((80, 0), get_amounts(80));
        assert_eq!((0, 0), get_amounts(0));
    }
}