use crate as airdrop;
use airdrop::pallet::Config;
use core::convert::Into;
use frame_support::pallet_prelude::*;
use frame_support::traits::Currency;
use frame_system;
use scale_info::TypeInfo;
use serde::Deserialize;
use sp_std::prelude::*;

/// AccountId of anything that implements frame_system::Config
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

/// Type that represent the balance
pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

/// Type that represent IconAddress
pub type IconAddress = sp_std::vec::Vec<u8>;

/// type that represnt the error that can occur while validation the signature
#[derive(Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
#[cfg_attr(not(feature = "std"), derive(RuntimeDebug))]
pub enum SignatureValidationError {
	InvalidIconAddress,
	InvalidIconSignature,
	InvalidIceAddress,
	Sha3Execution,
	ECRecoverExecution,
}

#[derive(Encode, Decode, Clone, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[cfg_attr(feature = "std", derive(Debug))]
#[cfg_attr(not(feature = "std"), derive(RuntimeDebug))]
#[derive(Eq, PartialEq)]
pub struct SnapshotInfo<T: Config> {
	/// Icon address of this snapshot
	pub icon_address: Vec<u8>,

	/// Total airdroppable-amount this icon_address hold
	pub amount: BalanceOf<T>,

	/// Indicator wather this icon_address holder is defi-user
	pub defi_user: bool,

	/// TODO: add description of this filed
	pub vesting_percentage: u32,

	/// indicator wather the user have claimmed the balance
	// TODO:
	// this field might need to be change when working with vesting
	// might be change into airdropped_amount: BalanceOf<T>
	pub claim_status: bool,
}

impl<T: Config> SnapshotInfo<T> {
	/// Helper function to set icon_address in builder-pattern way
	/// so that initilisation can be done in single line
	pub fn icon_address(mut self, val: Vec<u8>) -> Self {
		self.icon_address = val;
		self
	}
}

/// implement default values for snapshotInfo
impl<T: Config> Default for SnapshotInfo<T> {
	fn default() -> Self {
		Self {
			icon_address: sp_std::vec![],
			amount: 0_u32.into(),
			defi_user: false,
			vesting_percentage: 0,
			claim_status: false,
		}
	}
}

/// Possible values of error that can occur when doing claim request from offchain worker
#[cfg_attr(feature = "std", derive(Debug))]
#[cfg_attr(not(feature = "std"), derive(RuntimeDebug))]
#[derive(PartialEq, Eq)]
pub enum ClaimError {
	/// When there is no icon address in mapping corresponding
	/// to the ice_address stored in queue
	NoIconAddress,

	/// Error while doing an http request
	/// Also might contains the actual error
	HttpError,

	/// Server returned an response that is actually an error
	ServerError(ServerError),

	/// Server returned an response in a format that couldn't be understood
	/// this is set when response neither could not be deserialize into
	/// valid server response or valid server error
	InvalidResponse,

	/// Error was occured when making extrinsic call
	CallingError(CallDispatchableError),
}

/// Structure expected to return from server when doing a request for details of icon_address
#[derive(Deserialize, Encode, Decode, Clone, Default, Eq, PartialEq, TypeInfo, Copy)]
#[cfg_attr(feature = "std", derive(Debug))]
#[cfg_attr(not(feature = "std"), derive(RuntimeDebug))]
pub struct ServerResponse {
	// TODO: Add description of this field
	pub omm: u128,

	/// Amount to transfer in this claim
	// TODO:
	// is this amount to tranfer in this claim or tranfser in total?
	#[serde(rename = "balanced")]
	pub amount: u128,

	// TODO: add description of this field
	pub stake: u128,

	/// Indicator weather this icon_address is defi_user or not
	pub defi_user: bool,
}

/// Known error server might respond with
#[derive(Deserialize, Encode, Decode, Clone, Eq, PartialEq, TypeInfo, Copy)]
#[cfg_attr(feature = "std", derive(Debug))]
#[cfg_attr(not(feature = "std"), derive(RuntimeDebug))]
pub enum ServerError {
	/// When the given icon address do not existsin server
	InvalidAddress,

	/// When server is in maintainance mode
	HostOffline,

	/// When there is not data about this icon address
	NonExistentData,
}

/// Error while calling On-chain calls from offchain worker
#[cfg_attr(feature = "std", derive(Debug))]
#[cfg_attr(not(feature = "std"), derive(RuntimeDebug))]
#[derive(Eq, PartialEq)]
pub enum CallDispatchableError {
	/// No any account was found to send signed transaction from
	NoAccount,

	/// Error while dispatching the call
	CantDispatch,
}

/// Trait that marks something is verifable agains the given icon data
// This was originally created to be implemented against AccountId of airdrop-pallet
// as a way to ensure that the ice & icon address pair is authorised
pub trait IconVerifiable {
	fn verify_with_icon(
		&self,
		icon_wallet: &IconAddress,
		icon_signature: &[u8],
		message: &[u8],
	) -> Result<(), SignatureValidationError>;
}