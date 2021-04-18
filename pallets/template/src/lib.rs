#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_storage, decl_module};

pub trait Trait: frame_system::Config {}

decl_storage! {
	trait Store for Module<T: Trait> as KittyStroy {
		// Declare storage and gettter function here.
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// declare public functions here. 
		
	}
}
