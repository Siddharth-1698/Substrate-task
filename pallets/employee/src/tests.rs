#[cfg(test)]
mod tests {
	use crate as pallet_employee;
	use frame_support::traits::{ConstU16, ConstU64};
	use frame_support::{assert_noop, assert_ok};
	use frame_system as system;
	use sp_core::H256;
	use sp_core::*;
	use sp_runtime::{
		testing::Header,
		traits::{BadOrigin, BlakeTwo256, IdentityLookup},
	};

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
	type Block = frame_system::mocking::MockBlock<Test>;

	// Configure a mock runtime to test the pallet.
	frame_support::construct_runtime!(
		pub enum Test where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system,
			TemplateModule: pallet_employee,
		}
	);

	impl system::Config for Test {
		type BaseCallFilter = frame_support::traits::Everything;
		type BlockWeights = ();
		type BlockLength = ();
		type DbWeight = ();
		type RuntimeOrigin = RuntimeOrigin;
		type RuntimeCall = RuntimeCall;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type RuntimeEvent = RuntimeEvent;
		type BlockHashCount = ConstU64<250>;
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = ();
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ConstU16<42>;
		type OnSetCode = ();
		type MaxConsumers = frame_support::traits::ConstU32<16>;
	}

	impl pallet_employee::Config for Test {
		type RuntimeEvent = RuntimeEvent;
	}

	// Build genesis storage according to the mock runtime.
	pub fn new_test_ext() -> sp_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn create_emp() {
		let name_test: Vec<u8> = "test".as_bytes().to_vec();
		let company_name_test: Vec<u8> = "mfsi".as_bytes().to_vec();
		let dob_test: Vec<u8> = "16-01-98".as_bytes().to_vec();
		new_test_ext().execute_with(|| {
			assert_ok!(TemplateModule::create_employee(
				RuntimeOrigin::root(),
				1,
				name_test,
				company_name_test,
				dob_test
			));
			assert_noop!(
				TemplateModule::update_employee_info(
					RuntimeOrigin::signed(1),
					1,
					"test2".as_bytes().to_vec(),
					"mfsi".as_bytes().to_vec(),
					"16-01-98".as_bytes().to_vec()
				),
				BadOrigin
			);
		});
	}
	#[test]
	fn update_emp() {
		let name_test: Vec<u8> = "test2".as_bytes().to_vec();
		let company_name_test: Vec<u8> = "mfsi".as_bytes().to_vec();
		let dob_test: Vec<u8> = "16-01-98".as_bytes().to_vec();
		new_test_ext().execute_with(|| {
			assert_ok!(TemplateModule::update_employee_info(
				RuntimeOrigin::root(),
				1,
				name_test,
				company_name_test,
				dob_test
			));
			assert_noop!(
				TemplateModule::update_employee_info(
					RuntimeOrigin::signed(1),
					1,
					"test2".as_bytes().to_vec(),
					"mfsi".as_bytes().to_vec(),
					"16-01-98".as_bytes().to_vec()
				),
				BadOrigin
			);
		});
	}
	#[test]
	fn delete_emp() {
		new_test_ext().execute_with(|| {
			assert_noop!(TemplateModule::delete_employee(RuntimeOrigin::signed(1), 1,), BadOrigin);
			assert_ok!(TemplateModule::delete_employee(RuntimeOrigin::root(), 1,));
		});
	}
}
