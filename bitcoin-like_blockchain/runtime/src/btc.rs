use sp_core::{H256, H512};
use frame_support:: {
	decl_storage, decl_event, decl_module
};
use sp_runtime::traits::{
	BlackTwo256,
};

pub trait Trait: system::Trait {
	type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

#[cfg_attr(feature="std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default, Encode, Clone, Decode, Hash)]
pub struct TransactionInput {
	pub outpoint: U256,
	pub sigscript: H512, //proof
}

#[cfg_attr(feature="std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default, Encode, Clone, Decode, Hash)]
pub struct TransactionOutput {
	pub value: value, //value associated with this UTXO
	pub pubkey: H256, //key of the owner
}

#[cfg_attr(feature="std", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default, Encode, Clone, Decode, Hash)]
pub struct Transaction {
	pub inputs: Vec<TransactionInput>,
	pub outputs: Vec<TransactionOutput>,
}

decl_storage! {
	trait Store for Module<T: Trait> as Utxo {
		BTCStore build(|config: &GenesisConfig| {
			config.genesis_utxos
				.iter()
				.cloned()
				.map(|u| (BlackTwo256::hash_of(&u), u))
				.collect::<Vec<_>>()
		}): map hasher(identity) H256 => Option<TransactionOutput>;
	}


	// to seed data in the genesis block
	add_extra_genesis { 
		// seed with transaction outputs
		config(genesis_utxos): Vec<TransactionOutput>
	}
}

// External functions: callable by the end user
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn spend(_origin, transaction: Transaction) -> DispatchResult {
			// 1. TODO checks if a transaction is valid

			// 2. write to storage
			Self::update_storage(&transaction)?;

			// 3. emit success/error events
			Ok(());
		}

	}
}

impl<T: Trait> Module<T> {

	fn update_storage(transaction: &Transaction) -> DispatchResult {

		// remove input UTXO from utxostore
		for input in &transcation.inputs {
			// rust turbo fish
			<BTCStore>::remove(input.outpoint);
		}

		// create the new UTXO in BTCStore
		for output in &transaction.outputs {
			// below could cause security problem:
			// 	- cause hash collision 
			// 	- induce replay attack
			// let hash = BlackTwo256::hash_of(&output);
			// <BTCStore>::insert(hash, output);

			// instead, we hash the entire transaction and index of this output
			// index is sort of like erc712's nounce
			let mut index: u64 = 0;
			let hash = BlackTwo256::hash_of(&(&transaction.encode(), index));
			index = index.checked_add(1).ok_or(
				"output index overflow"
			)?;
			<BTCStore>::insert(hash, output);
		}

		Ok(());
	}
}