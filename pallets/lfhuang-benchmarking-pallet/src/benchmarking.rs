use super::*;

#[allow(unused)]
use crate::Pallet as LfhuangBenchmarking; //依赖于当前的pallet 取个名字
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	do_something {// 基准的名称是do_something
		// 变量b存储用于测试函数执行时间的输入do_user_info
		// 变量值b在 1 到 100 之间变化，因此您可以重复运行基准测试以使用不同的输入值来测量执行时间。
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: {
		LfhuangBenchmarking::<T>::do_user_info(RawOrigin::Signed(caller).into(), s.into()) // 执行调用函数  需要root权限执行
	}
	verify {//验证值
		assert_eq!(UserStorage::<T>::get(), s.into());
	}
	// 使用mock中的new_test_ext
	impl_benchmark_test_suite!(LfhuangBenchmarking, crate::mock::new_test_ext(), crate::mock::Test);
}
