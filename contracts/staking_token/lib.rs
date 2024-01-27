#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22)]
#[openbrush::contract]
mod stake_token {

    use openbrush::contracts::psp22::*;
    use openbrush::traits::{ Storage, String };
    use openbrush::test_utils::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakeToken {
       #[storage_field]
       psp22: psp22::Data
    }

    #[overrider(psp22::Internal)]
    fn _before_token_transfer(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        amount: &Balance
    ) -> Result<(), PSP22Error> {
        if from == Some(&[0; 32].into()) {
            return Err(PSP22Error::Custom(String::from("Transfer from zero address not allowed")));
        } else {
            Ok(())
        }
    }

    impl StakeToken {

        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {

            let mut instance = Self::default();
            let caller = Self::env().caller();

            psp22::Internal::_mint_to(&mut instance, caller, total_supply)
                .expect("Should mint total supply!");
            instance

        }


    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let accounts = accounts();
            let mint_amount = 10_000_000;

            let stake_token = StakeToken::new(mint_amount);
            let alice_balance = PSP22::balance_of(&staking_contract, accounts.alice);

            assert_eq!(alice_balance, mint_amount);
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = accounts();
            let mint_amount = 10_000_000;
            let transfer_amount = 1_000;

            let mut stake_token = StakeToken::new(mint_amount);
            let result = PSP22::transfer(&stake_token, accounts.bob, transfer_amount, Vec::<u8>::new());

            let alice_balance = PSP22::balance_of(&stake_token, accounts.alice);
            let bob_balance = PSP22::balance_of(&stake_token, accounts.bob);

            assert!(result.is_ok());
            assert_eq!(alice_balance, mint_account - transfer_amount);
            assert_eq!(bob_balance, transfer_amount);
        }
    }
}
