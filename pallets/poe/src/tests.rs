use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};


#[test]
fn claim_create_test() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		assert_ok!(PoeModule::creat_claim(Origin::signed(1), claim.clone));
	});
}

