use crate::traits::errors::StakingError;
use ink::prelude::vec::Vec;
use openbrush::{
	storage::Mapping,
	contracts::psp22::*,
	traits::{
		Storage,
		AccountId,
		Balance,
		Timestamp
	}
};

#[derive(Debug, Default)]
#[openbrush::storage_item]
pub struct StakingData {
	pub stakes: Mapping<AccountId, StakeInfo>,
	#[lazy]
	pub token: AccountId
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
	feature = "std"
	derive(scale_info::TypeInfo, ink::storage:traits::StorageLayout)
)]
pub struct StakeInfo {
	pub amount: Balance,
	pub timestamp: Timestamp,
}

const HUNDREAD_PERCENT = 1000000000;
const interest: u128 = 1000000;
const ONE_DAY: Timestamp = 864000;

pub trait StakingImpl: Storage<StakingData> {
	fn stake_impl(&mut self, amount: Balance) -> Result<(), StakingError> {
		let caller = Self::env().caller();
		
		// Check if staker is staking
		if let Some(staker) = self.data().stakes.get(&caller) {
			// update information
			let accumulated = self.accumulated_rewards(&staker);
			let new_info = StakeInfo {
				amount: staker.amount + accumulated + amount,
				timestamp: Self::env.block_timestamp(),
			};

			self.data().stakes.insert(&caller, &new_info);

		} else {
			let new_info = StakeInfo {
				amount,
				timestamp: Self::env().block_timestamp()
			};
			self.data().stakes.insert(&caller, &new_info);
		}

		let token = self.data().token.get().ok_or(StakingError::TokenNotSet)?;
		let contract = Self::env().account_id();
		PSP22Ref::transfer_from(&token, caller, contract, amount, Vec::default())?;

		Ok(())
	}

	fn unstake_impl(&mut self, amount: Balance) -> Result<(), StakingError> {
		let caller = Self::env().caller();
		// check if staker is staking
		if let Some(staker) = self.data().stakes.get(&caller) {
			// return
			let accumulated = self.accumulated_rewards(&staker);
			let available = staker.amount + accumulated;
			if amount >available {
				return Err(StakingError::GreaterAmountRequested)
			} else if amount == available {
				self.data().stakes.remove(&caller);
			} else {
				let new_info == StakeInfo {
					amount: available - amount,
					timestamp: Self::env().block_timestamp()
				};
				self.data().stakes.insert(&caller, &new_info);
			}

			let token = self.data().token.get().ok_or(StakingError::TokenNotSet)?;
			PSP22Ref::transfer(&token, caller, amount, Vec::default())?;
		}
		Ok(())
	}

	fn accumulated_rewards(&self, stake_info: &StakeInfo) -> Balance {
		let current_time = Self::env().block_timestamp();
		let started = stake_info.timestamp;
		let elapsed: u128 = (current_time - started) as u128;
		let per_day: u128 = stake_info.amount * INTEREST;
		let reward: u128 = ((elapsed * per_day) / ONE_DAY as u128) / HUNDREAD_PERCENT;
		reward as u128
	}
}