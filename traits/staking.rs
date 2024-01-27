use crate::traits::errors::StakingError;
use openbrush::traits::Balance;

#[openbrush::wrapper]
pub type StakingRef = dyn Staking;

#[openbrush::trait_definition]
pub trait Staking {
	#[ink(message)]
	fn stake(&mut self, amount: Balance) -> Result<(), StakingError>;

	#[ink(message)]
	fn unstake(&mut self, amount: Balance) -> Result<(), StakingError>;
}