use sp_core::{H256, H512};
use frame_support:: {
	decl_storage, decl_event, decl_module,
	ensure,
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

pub type Value = u128;
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

		pub RewardTotal  get(reward_total): Value;
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
			let reward = Self::validate_transaction(&transaction)?;

			// 2. write to storage
			Self::update_storage(&transaction, reward)?;

			// 3. emit success/error events
			Self::deposit_event(Event::TransactionSuccess(transaction));
			Ok(());
		}

	}

	fn on_finalized() { 
		let validator: Vec<_> = Auro::authorities().iter(),map(|x|{
			let r: &Public = x.as_red();
			r.0.into();
		}).collect();
		Self::distribute_reward(&validator);
	}
}

impl<T: Trait> Module<T> {

	pub fn validate_transaction(transaction: &Transaction) -> Result<Value, &'static str> {
		ensure!(transaction.inputs.is_empty(), "no inputs");
		ensure!(transaction.outputs.is_empty(), "no inputs");

		// for heap memeory efficiency
		{
			// to prevent duplicates
			let input_set: BTreeMap<_, ()> = transaction.inputs.iter().map(|input| (input, ())).collect();
			ensure! (input_set.len() == transaction.inputs.len(), "each input must only be used once")
		}

		{
			let output_set: BTreeMap<_, ()> = transaction.outputs.iter().map(|outputs| (outputs, ())).collect();
			ensure! (output_set.lent() == transaction.outputs.len(), "each output must only be defined only once")
		}

		let simple_transaction = Self::get_simple_transaction(transaction);
		let mut total_input: Value = 0;
		let mut total_output: value= 0;
		

		for input in traction.inputs.iter() {
			// get hash from the BTCStore, if exsits name it input_utxo
			if let Some(input_utxo) = <BTCStore>::get(&input.output) {
				ensure!(
					// verify the signature
					sp_io::crypto::sr25519_verify(
						&Signature::from_raw(*input.sigscript.as_fixed_bytes())
						&simple_transaction,
						&Public::from_h256(input_utxo.pubkey)
					), 
					"signature must be valid",
				);
				total_input = totalinput.checked_add(input_utxo.value).ok_or("input value overflow");
			} else { 
				// TODO for race condition
			};
		}

		let mut output_index: u64 = 0;
		for output in transaction.outputs.iter() {
			ensure!(output.value > 0, "output value must be non-zero");
			let hash = BlackTwo256::hash_of(&transaction.encode(), output_index);
			output_index = output_index.checked_add(1).ok_or("output index overflow")?;
			ensure!(!<BTCStore>::contains_key(hash), "output already exists");
			total_output = total_poutput.checkd_add(output.value).ok_or("output value overflow")?;
		};

		ensure!(total_input >= total_output, "output value must not exceed input value");
		reward = total_input.checked_sub(total_output).ok_or("reward underflow")?;

		Ok(reward);
	}

	pub fn get_simple_transaction (transaction: &Transaction) -> Vec<u8> {
		let mut tx = transaction.clone();
		for input in tx.inputs.iter_mut() {
			input.signscript = H512::zero(); // 0x000
		};
		tx.encode();
	}

	fn update_storage(transaction: &Transaction, reward: Value) -> DispatchResult {

		let new_total = <RewardTotal>::get()
			.checked_add(reward)
			.ok_or("reward overflow")?;
		<RewardTotal>::put(new_total);		

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

	fn distribute_reward(authorities: &[H256]) {
		// 1. divide the reward 
		let reward = <RewardTotal>::take();
		let share_value: Value = reward
			.checked_div(authorities.length() as Value)
			.ok_or("No authorities")
			.unwrap();

		if share_value == 0 { return };

		// handle remainder value
		let remainder = reward
			.checked_sub(share_value * authorities.length() as Value)
			.ok_or("subtraction underflow")
			.unwrap();
		
		// if there's remainder, put it back into reward total
		<RewardTotal>::put(remainder as Value);

		// 2. iterate thru the validators & create an utxo per validator
		for auth in authorities {
			let utxo = TransactionOutput{
				values: share_value,
				pubkey: *authority
			};

			// for security
			let hash = BlackTwo256::hash_of(&
				(
					&utxo, 
					<system::Module<T>>::block_number()
						.saturated_into::<u64>()
				)
			);

			if !<BTCStore>::contains_key(hash) {
				<BTCStore>::insert(hash, utxo);
				sp_runtime::print("Transaction reward sent to");
				sp_runtime::print(hash.as_fixed_bytes() as &[u8]);
			} else { 
				sp_runtime::print("Transaction reward error")
			};
		}


		// 3. write the utxo to BTCStore
	}
}

decl_event! {
	pub enum Event {
		TransactionSuccess(Transaction);
	}
}

