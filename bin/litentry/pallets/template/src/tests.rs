// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use sp_core::H256;

#[test]
fn it_works_for_default_value() {
	ExtBuilder::build().execute_with(|| {
		// Just a dummy test for the dummy function `do_something`
		// calling the `do_something` function with a value 42
		let destination:u64 = 2;
		let dumbData:u64 = 1;
		assert_ok!(LitentryPallet::register_identity(Origin::signed(1)));
		// asserting that the stored value is equal to what we stored
		assert_eq!(StructStorage::identities_count(), 1 as u64);

		let identity_id = H256::from_low_u64_be(16);

		assert_ok!(LitentryPallet::register_identity_with_id(Origin::signed(1), identity_id));

		assert_eq!(StructStorage::identities_count(), 2 as u64);

		assert_eq!(StructStorage::identity_by_index(1 as u64), identity_id);

		assert_ok!(LitentryPallet::issue_token(Origin::signed(1), destination, identity_id, dumbData, dumbData, dumbData, dumbData));

		assert_eq!(StructStorage::tokens_count_of_owner(2), 1 as u64);

		assert_eq!(StructStorage::tokens_count_of_identity(identity_id), 1 as u64);
	});
}
