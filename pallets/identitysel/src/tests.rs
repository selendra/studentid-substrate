// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Tests for Identity Pallet

use super::*;
use crate as pallet_studentid;
use frame_support::{
	assert_noop, assert_ok, ord_parameter_types, parameter_types, BoundedVec
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;


frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Identity: pallet_studentid::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type PalletInfo = PalletInfo;
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	//type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}
parameter_types! {
	pub const BasicDeposit: u64 = 10;
	pub const FieldDeposit: u64 = 10;
	pub const SubAccountDeposit: u64 = 10;
	pub const MaxSubAccounts: u32 = 2;
	pub const MaxUseridentities: u32 = 2;
	pub const MaxAdditionalFields: u32 = 2;
	pub const MaxRegistrars: u32 = 20;
	pub const MaxEmailsize: u32 = 30;
    pub const MaxTokenid: u32 = 30;

}
ord_parameter_types! {
	pub const One: u64 = 1;
	pub const Two: u64 = 2;
    pub const MaxAccessTokenMetadata: u32 = 15;

}
//type EnsureOneOrRoot = EnsureOneOf<EnsureRoot<u64>, EnsureSignedBy<One, u64>>;
//type EnsureTwoOrRoot = EnsureOneOf<EnsureRoot<u64>, EnsureSignedBy<Two, u64>>;
impl pallet_studentid::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type Slashed = ();
	type BasicDeposit = BasicDeposit;
	type FieldDeposit = FieldDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type MaxUseridentities = MaxUseridentities;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type MaxEmailsize = MaxEmailsize;
    type MaxAccessTokenMetadata = MaxAccessTokenMetadata;
    type MaxTokenid = MaxTokenid;

	type RegistrarOrigin = frame_system::EnsureRoot<Self::AccountId> ;
	type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>  ;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 10), (2, 10), (3, 10), (10, 100), (20, 100), (30, 100)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}


#[test]
fn userregistration_should_work_sela() {
	new_test_ext().execute_with(|| {
		let _data = |x| Data::Raw(vec![x; 1].try_into().unwrap());
        let user: Vec<u8> = b"a@b.com".to_vec().try_into().unwrap();
        let pass: Vec<u8> = b"hello123".to_vec().try_into().unwrap();
        let wrongpass: Vec<u8> = b"xxhello123".to_vec().try_into().unwrap();
        let wronguser: Vec<u8> = b"a@wrong.com".to_vec().try_into().unwrap();
		assert_ok!(Identity::request_registration_sel11(Origin::signed(10),user.clone(), pass.clone()  ) );
		assert_ok!(Identity::login_access_sel12(Origin::signed(10),user.clone(), pass.clone(), ) );
		assert_noop!(Identity::login_access_sel12(Origin::signed(10), wronguser, pass.clone(), ), Error::<Test>::NoIdentity);
		assert_noop!(Identity::login_access_sel12(Origin::signed(10), user.clone(), wrongpass, ), Error::<Test>::LoginFailed);

	});
}


#[test]
fn changepassword_should_work_sela() {
	new_test_ext().execute_with(|| {
		let _data = |x| Data::Raw(vec![x; 1].try_into().unwrap());
        let user: Vec<u8> = b"a@b.com".to_vec().try_into().unwrap();
        let pass: Vec<u8> = b"hello123".to_vec().try_into().unwrap();
        let changedpass: Vec<u8> = b"welcome123".to_vec().try_into().unwrap();
        let _wrongpass: Vec<u8> = b"xxhello123".to_vec().try_into().unwrap();
        let _wronguser: Vec<u8> = b"a@wrong.com".to_vec().try_into().unwrap();
		assert_ok!(Identity::request_registration_sel11(Origin::signed(10),user.clone(), pass.clone()  ) );
		assert_ok!(Identity::login_access_sel12(Origin::signed(10),user.clone(), pass.clone()   ) );
		assert_ok!(Identity::change_password_sel13(Origin::signed(10),user.clone(), changedpass.clone()  ) );
		assert_noop!(Identity::login_access_sel12(Origin::signed(10), user.clone(), pass ), Error::<Test>::LoginFailed);
		assert_ok!(Identity::login_access_sel12(Origin::signed(10),user.clone(), changedpass.clone(),    ) );


	});
}

#[test]
fn referal_setup_andusing_sela() {
	new_test_ext().execute_with(|| {
		let _data = |x| Data::Raw(vec![x; 1].try_into().unwrap());
        let user: Vec<u8> = b"a@b.com".to_vec().try_into().unwrap();
        let pass: Vec<u8> = b"hello123".to_vec().try_into().unwrap();
        let referal: Vec<u8> = b"referABCD".to_vec().try_into().unwrap();
        let wrongreferal: Vec<u8> = b"wrongreferABCD".to_vec().try_into().unwrap();
		assert_ok!(Identity::request_registration_sel11(Origin::signed(10),user.clone(), pass.clone()  ) );
		assert_ok!(Identity::login_access_sel12(Origin::signed(10),user.clone(), pass.clone()    ) );
		assert_ok!(Identity::set_referal_sel12(Origin::signed(10),user.clone(),referal.clone() ) );
		assert_noop!(Identity::create_web3link_sel15(Origin::signed(10),user.clone(),10, wrongreferal.clone() ),  Error::<Test>::ReferalFailed );
		assert_noop!(Identity::create_web3link_sel15(Origin::signed(10),user.clone(),9, referal.clone() ),  Error::<Test>::SignerNotmatching );
		assert_ok!(Identity::create_web3link_sel15(Origin::signed(10),user.clone(),10, referal.clone() ) );
	});
}


#[test]
fn login_web3_method_sela() {
	new_test_ext().execute_with(|| {
		let _data = |x| Data::Raw(vec![x; 1].try_into().unwrap());
        let user: Vec<u8> = b"a@b.com".to_vec().try_into().unwrap();
        let pass: Vec<u8> = b"hello123".to_vec().try_into().unwrap();
        let referal: Vec<u8> = b"referABCD".to_vec().try_into().unwrap();
        let challenge: Vec<u8> = b"random27363".to_vec().try_into().unwrap();
		assert_ok!(Identity::request_registration_sel11(Origin::signed(10),user.clone(), pass.clone()  ) );
		assert_ok!(Identity::login_access_sel12(Origin::signed(10),user.clone(), pass.clone()  ) );
		assert_ok!(Identity::set_referal_sel12(Origin::signed(10),user.clone(),referal.clone() ) );
		assert_ok!(Identity::create_web3link_sel15(Origin::signed(10),user.clone(),10, referal.clone() ) );
		assert_noop!(Identity::login_web3_sel16(Origin::signed(9), challenge.clone()  ), Error::<Test>::LoginFailed );
		assert_ok!(Identity::login_web3_sel16(Origin::signed(10), challenge.clone() ) );
	});
}

#[test]
fn access_web3_method_sela() {
	new_test_ext().execute_with(|| {
		let _data = |x| Data::Raw(vec![x; 1].try_into().unwrap());
        let user: Vec<u8> = b"a@b.com".to_vec().try_into().unwrap();
        let pass: Vec<u8> = b"hello123".to_vec().try_into().unwrap();
        let referal: Vec<u8> = b"referABCD".to_vec().try_into().unwrap();
        let service: Vec<u8> = b"docsystem".to_vec().try_into().unwrap();
        let challenge: Vec<u8> = b"random27363".to_vec().try_into().unwrap();
        let wrongchallenge: Vec<u8> = b"random75553".to_vec().try_into().unwrap();
        let manager = 11;
        let staff = 10;
		assert_ok!(Identity::request_registration_sel11(Origin::signed(staff),user.clone(), pass.clone()  ) );
		assert_ok!(Identity::login_access_sel12(Origin::signed(staff),user.clone(), pass.clone()  ) );
		assert_ok!(Identity::set_referal_sel12(Origin::signed(staff),user.clone(),referal.clone() ) );
		assert_ok!(Identity::create_web3link_sel15(Origin::signed(staff),user.clone(),10, referal.clone() ) );
		assert_ok!(Identity::login_web3_sel16(Origin::signed(staff), challenge.clone()) );

 
        let tokeninfo1 = <Tokens<Test>>::get(BoundedVec::try_from(challenge).unwrap()).unwrap();

//		assert_eq!(tokenid, challenge );
		assert_eq!(tokeninfo1.data,   Data::Raw(b"allowed".to_vec().try_into().unwrap()) );


       let tokeninfo2 = <Tokens<Test>>::get(BoundedVec::try_from(wrongchallenge).unwrap());

	   assert_eq!(tokeninfo2,   None);


	});
}



#[test]
fn logout_web3_method_sela() {
	new_test_ext().execute_with(|| {
		let _data = |x| Data::Raw(vec![x; 1].try_into().unwrap());
        let user: Vec<u8> = b"a@b.com".to_vec().try_into().unwrap();
        let pass: Vec<u8> = b"hello123".to_vec().try_into().unwrap();
        let referal: Vec<u8> = b"referABCD".to_vec().try_into().unwrap();
        let service: Vec<u8> = b"docsystem".to_vec().try_into().unwrap();
        let challenge: Vec<u8> = b"random27363".to_vec().try_into().unwrap();
        let wrongchallenge: Vec<u8> = b"random75553".to_vec().try_into().unwrap();
        let manager = 11;
        let staff = 10;
		assert_ok!(Identity::request_registration_sel11(Origin::signed(staff),user.clone(), pass.clone()  ) );
		assert_ok!(Identity::login_access_sel12(Origin::signed(staff),user.clone(), pass.clone()  ) );
		assert_ok!(Identity::set_referal_sel12(Origin::signed(staff),user.clone(),referal.clone() ) );
		assert_ok!(Identity::create_web3link_sel15(Origin::signed(staff),user.clone(),10, referal.clone() ) );
		assert_ok!(Identity::login_web3_sel16(Origin::signed(staff), challenge.clone()) );

 
        let tokeninfo1 = <Tokens<Test>>::get(BoundedVec::try_from(challenge.clone()).unwrap()).unwrap();

		assert_eq!(tokeninfo1.data,   Data::Raw(b"allowed".to_vec().try_into().unwrap()) );


	   assert_ok!(Identity::logout_web3_sel19(Origin::signed(staff) ));

        let tokeninfo2 = <Tokens<Test>>::get(BoundedVec::try_from(challenge).unwrap()).unwrap();

        // This is not correct. Need to fix it
		assert_eq!(tokeninfo2.data,   Data::Raw(b"allowed".to_vec().try_into().unwrap()) );



	});
}

