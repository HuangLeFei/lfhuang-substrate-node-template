use super::pallet::UserStorage;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn test_do_user_info() {
	new_test_ext().execute_with(|| {
		// LfhuangTestDemo 使用的mock.rs中定义的名字 LfhuangTestDemo: lfhuang_test_pallet_template,

		// 用普通账户提交签名  报错
		assert_noop!(LfhuangTestDemo::do_user_info(RuntimeOrigin::signed(1), 42), BadOrigin);

		// 需要root账户提交签名交易
		assert_ok!(LfhuangTestDemo::do_user_info(RuntimeOrigin::root(), 24));
		let value = UserStorage::<Test>::get();
		println!("###---------------### value:{}", value);
		assert_eq!(value, 24);
		// assert_eq!(value, 25);
	});
}
