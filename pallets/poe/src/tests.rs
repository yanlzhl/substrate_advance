use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok,BoundedVec};


#[test]
fn test_claim_create_test() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		assert_ok!(PoeMoudle::creat_claim(Origin::signed(1), claim.clone()));

		let bound_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&bound_claim),
			Some((1,frame_system::Pallet::<Test>::block_number()))
		)
	});
}

#[test]
fn test_claim_create_failed_when_clain_alread_exist(){
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		assert_ok!(PoeMoudle::creat_claim(Origin::signed(1), claim.clone()));

		assert_noop!(
            PoeMoudle::creat_claim(Origin::signed(1), claim),
            Error::<Test>::ProofAlreadyExist
        );
	});
}

#[test]
fn test_create_claim_failed_when_claim_is_too_long() {
    new_test_ext().execute_with(|| {
        let claim = vec![0; 999];

        assert_noop!(
            PoeMoudle::creat_claim(Origin::signed(1), claim),
            Error::<Test>::ClaimTooLong
        );
    })
}


#[test]
fn test_revoke_claim() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeMoudle::creat_claim(Origin::signed(1), claim.clone()));

        assert_ok!(PoeMoudle::revoke_claim(Origin::signed(1), claim));
    })
}

#[test]
fn test_transfer_claim() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeMoudle::creat_claim(Origin::signed(1), claim.clone()));
        let bounded_claim = BoundedVec::<u8,<Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

        assert_ok!(PoeMoudle::transfer_claim(Origin::signed(1), 2, claim.clone()));
        assert_eq!(Proofs::<Test>::get(&bounded_claim), Some((2, frame_system::Pallet::<Test>::block_number())));
    })
}

