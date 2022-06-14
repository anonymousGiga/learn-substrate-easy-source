use super::pallet::{Class, DormInfo, StudentsInfo};
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn test_set_class_info() {
	new_test_ext().execute_with(|| {
		assert_noop!(UseTestDemo::set_class_info(Origin::signed(1), 42), BadOrigin);
		assert_ok!(UseTestDemo::set_class_info(Origin::root(), 42));
		assert_noop!(
			UseTestDemo::set_class_info(Origin::root(), 2),
			Error::<Test>::SetClassDuplicate
		);
		assert_eq!(Class::<Test>::get(), 42);
	});
}

#[test]
fn test_set_student_info() {
	new_test_ext().execute_with(|| {
		assert_ok!(UseTestDemo::set_student_info(Origin::signed(1), 1, 123));
		assert_eq!(StudentsInfo::<Test>::get(1), 123);
		assert_noop!(
			UseTestDemo::set_student_info(Origin::signed(1), 1, 234),
			Error::<Test>::SetStudentsInfoDuplicate
		);
	});
}

#[test]
fn test_set_dorm_info() {
	new_test_ext().execute_with(|| {
		assert_ok!(UseTestDemo::set_dorm_info(Origin::signed(1), 1, 1, 1));
		assert_eq!(DormInfo::<Test>::get(1, 1), 1);
		assert_noop!(
			UseTestDemo::set_dorm_info(Origin::signed(1), 1, 1, 2),
			Error::<Test>::SetDormInfoDuplicate
		);
	});
}
