#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22)]
#[openbrush::contract]
mod staking {
    
    use openbrush::traits::Storage;
    use staking_contract::traits::errors::StakingError;
    use staking_contract::traits::staking::*;
    use staking_contract::impls::staking::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakingContract {
        #[storage_field]
        staking: StakingData,
    }

    impl StakingImpl for StakingContract {}

    impl Staking for StakingContract {
        #[ink(message)]
        fn stake(&mut self, amount: Balance) -> Result<(), StakingError> {
            self.stake_impl(amount)
        }


        #[ink(message)]
        fn unstake(&mut self, amount: Balance) -> Result<(), StakingError> {
            self.unstake_impl(amount)
        }
    }

    impl StakingContract {
        #[ink(constructor)]
        pub fn new(token: AccountId) -> Self {
            let mut contract = Self::default();
            contract.staking.token.set(&token);
            contract
        }
    }
}
