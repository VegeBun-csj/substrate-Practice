use super::*;
use crate::{mock::*, Error, Proofs};
use frame_support::{assert_noop, assert_ok};

/// 创建存证的测试用例
#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		// 这里因为AccountId是u64类型，所以这里直接用整数1来表示账户
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

/// 创建存证失败的测试用例1(存证已经存在，不能重复创建)
#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		// 这里因为AccountId是u64类型，所以这里直接用整数1来表示账户
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

/// 创建存证失败的测试用力(存证claim超过了一定长度)
#[test]
fn create_claim_failed_when_claim_outlength() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 2, 3, 4];
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimOutLength
		);
	})
}

/// 注销存证的测试用例
#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), None);
	})
}

/// 注册存证失败的测试用例1（不存在这个存证）
#[test]
fn revoke_claim_works_failed_when_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

/// 注销存证失败的测试用例2（不是存证的拥有者）
#[test]
fn revoke_claim_works_failed_when_claim_not_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		// 使用账户1进行创建存证
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			// 使用账户2注销存证，报错
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

/// 转移存证的测试用例
#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		// 将账户1创建的存证转移给账户2
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

/// 转移存证失败的测试用例1（用户没有存证进行转移）
#[test]
fn transfer_claim_failed_when_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		// 账户1创建存证
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		// 账户1注销存证,此时账户1的存证为None，是无法进行转移存证的
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), None);
		// 此时如果再想转移存证会出现不是存证拥有者的错误
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	})
}

/// 转移存证失败的测试用例2（用户不是存证的拥有者就进行转移）
#[test]
fn transfer_claim_failed_when_claim_not_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		// 账户1是claim的所有者
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			// 账户2试图将账户1的claim转移给账户3,就会出现错误，因为不是claim的所有者
			PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
			Error::<Test>::NotClaimOwner
		);
	})
}
