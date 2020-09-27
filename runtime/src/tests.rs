#![cfg(test)]

use super::*;
use mock::*;

pub const CLV: CurrencyId = CurrencyId::CLV;
pub const CUSD: CurrencyId = CurrencyId::CUSD;
use orml_traits::{MultiCurrency, MultiCurrencyExtended};

#[test]
fn test_balance() {
	ExtBuilder::default()
		.balances(vec![
			(AccountId::from(ALICE), CLV, 1000),
			(AccountId::from(BOB), CUSD, 1000),
		])
		.build()
		.execute_with(|| {
			assert_eq!(<Currencies as MultiCurrency<_>>::free_balance(CLV, &AccountId::from(ALICE)), 500);
			assert_eq!(<Currencies as MultiCurrency<_>>::free_balance(CUSD, &AccountId::from(BOB)), 1000);

			let _ = <Currencies as MultiCurrencyExtended<_>>::update_balance(CLV, &AccountId::from(ALICE), 1000);
			assert_eq!(<Currencies as MultiCurrency<_>>::free_balance(CLV, &AccountId::from(ALICE)), 1500);
		});
}
