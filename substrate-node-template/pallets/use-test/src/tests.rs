use super::pallet::Class;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn test_set_class_info() {
	new_test_ext().execute_with(|| {
		assert_noop!(UseTestDemo::set_class_info(Origin::signed(1), 42), BadOrigin);
		assert_ok!(UseTestDemo::set_class_info(Origin::root(), 42));
		assert_eq!(Class::<Test>::get(), 42);
	});
}
