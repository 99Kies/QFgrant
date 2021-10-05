//! Benchmarking setup for pallet-quadratic-funding

use super::*;

#[allow(unused)]
use crate::Pallet as QuadraticFunding;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	do_something {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}
}

impl_benchmark_test_suite!(QuadraticFunding, crate::mock::new_test_ext(), crate::mock::Test);
