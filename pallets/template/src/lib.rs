#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	decl_module, decl_storage, decl_event, 
	dispatch::{DispatchResult, Vec},
	ensure,
};
use sp_core::{H256, H512};
use frame_system::{ensure_signed,};
use serde::{Deserialize, Serialize};
use sp_core::sr25519::{Public, Signature};
use sp_runtime::traits::{BlakeTwo256, Hash, SaturatedConversion};
use sp_std::collections::btree_map::BTreeMap;
use sp_runtime::transaction_validity::{TransactionLongevity, ValidTransaction};

pub trait Trait: frame_system::Config {
	type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
}

pub type Value = u128;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Default, Clone, Encode, Decode, Hash, Debug)]
pub struct TransactionInput {
	pub output: H256, // reference to a UTXO to be spent 
	pub sigscript: H512, // proof
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Default, Clone, Encode, Decode, Hash, Debug)]
pub struct TransactionOutput {
	pub value: Value, // value associated with this UTXO 
	pub pubkey: H256, // public key assocaited with this output, key at the UTXO's owner.
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Default, Clone, Encode, Decode, Hash, Debug)]
pub struct Transaction {
	pub inputs: Vec<TransactionInput>, 
	pub outputs: Vec<TransactionOutput>,
}

decl_storage! {
	trait Store for Module<T: Trait> as Utxo {
		UtxoStore build(|config: &GenesisConfig| {
			config.genesis_utxos
				.iter()
				.cloned()
				.map(|u| (BlakeTwo256::hash_of(&u), u))
				.collect::<Vec<_>>()
		}) : map hasher(identity) H256 => Option<TransactionOutput>;
	}

	add_extra_genesis {
		config(genesis_utxos): Vec<TransactionOutput>;
	}
}

// external functions: callable by the end user.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		// declare public functions here.
		#[weight = 10_000]
		fn spend(_origin, transaction: Transaction) -> DispatchResult {
			// 1, check that the trx is valid 
			// let sender = ensure_signed(_origin)?;

			// 2, writes to storage
			Self::update_storage(&transaction)?;

			// 3, emit success event. 
			Self::deposit_event(Event::TransactionSuccess(transaction));


			Ok(()) // Error
		}
	}
}

decl_event! {
	pub enum Event {
		TransactionSuccess(Transaction),
	}
}

impl<T: Trait> Module<T> {
	fn update_storage(transaction: &Transaction) -> DispatchResult {
		// 1, remove input UTXO from utxo storers
		for input in &transaction.inputs {
			<UtxoStore>::remove(input.output);
		}
		// 2, creates the new UTXO in utxostore
		let mut index: u64 = 0;
		for output in &transaction.outputs {
			let hash = BlakeTwo256::hash_of( &(&transaction.encode(), index) );
			index = index.checked_add(1).ok_or("output index overflow!")?;
			// 50. 0x0000
			<UtxoStore>::insert(hash, output);
		}

		Ok(())
	}
}
