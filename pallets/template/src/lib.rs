#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch, ensure};
use frame_system::{ensure_signed};

pub trait Trait: frame_system::Config {}

decl_storage! {
	trait Store for Module<T: Trait> as KittyStorage {
		// Declare storage and gettter function here.
		Value: map hasher(blake2_128_concat) T::AccountId => u64;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		// declare public functions here. 
		#[weight = 10_000]
		fn set_value(origin, value: u64) -> dispatch::DispatchResult {
			let _sender = ensure_signed(origin)?;

			// ensure!(!<Value<T>>::contains_key(value), "Key already exists!");

			<Value<T>>::insert(_sender, value);

			Ok(())
		}


	}
}
