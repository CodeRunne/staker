use openbrush::contracts::psp22::PSP22Error;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum StakingError {
	PSP22Error(PSP22Error),
	TokenNotSet,
	GreaterAmountRequested
}

impl From<PSP22Error> for StakingError {
	fn from(error: PSP22Error) -> StakingError {
		StakingError::PSP22Error(error)
	}
}