#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_module, decl_storage, dispatch::DispatchResult, 
	codec::{Decode, Encode},
};
use frame_system::ensure_signed;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty <Hash, Balance> {
	id: Hash,
	dna: Hash,
	price: Balance,
	gen: u64,
}

pub trait Trait: frame_system::Config {}

decl_storage! {
	trait Store for Module<T: Trait> as KittyStorage {
		// Declare storage and gettter function here.
		OwnedMyKitties get(fn get_my_kitties): map hasher(blake2_128_concat) T::AccountId => Kitty<T::Hash, T::Balance>;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		// declare public functions here.
		#[weight = 10_000]
		fn create_kitty(origin, number: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let new_kitty = Kitty {
				id: Hash::hash_of(&0),
				dna: Hash::hash_of(&0),
				price: <T::Balance as As<u64>>::sa(0),
				gen: 1,
			};

			<OwnedMyKitties<T>>::insert(sender, new_kitty);

			Ok(())
		}


	}
}
