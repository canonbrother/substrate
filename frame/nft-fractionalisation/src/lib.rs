#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use scale_info::Type;

pub type ItemId = <Type as pallet_nfts::Config>::ItemId;
pub type CollectionId = <Type as pallet_nfts::Config>::CollectionId;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_support::{
		dispatch::DispatchResult,
		sp_runtime::traits::{
			AccountIdConversion, AtLeast32BitUnsigned, IntegerSquareRoot, SaturatedConversion,
			Saturating, StaticLookup, Zero,
		},
		traits::{
			fungibles::{
				metadata::Mutate as MutateMetadata, Create, Inspect, InspectEnumerable, Mutate,
				Transfer,
			},
			tokens::nonfungibles_v2::{
				Inspect as NonFungiblesInspect, Transfer as NonFungiblesTransfer,
			},
			Currency,
		},
		PalletId,
	};

	pub use pallet_nfts::Incrementable;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	pub type AssetIdOf<T> =
		<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;
	pub type AssetBalanceOf<T> =
		<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

	pub type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: Currency<Self::AccountId>;

		/// Identifier for the collection of item.
		type CollectionId: Member + Parameter + MaxEncodedLen + Copy + Incrementable;

		/// The type used to identify a unique item within a collection.
		type ItemId: Member + Parameter + MaxEncodedLen + Copy;

		type AssetBalance: AtLeast32BitUnsigned
			+ codec::FullCodec
			+ Copy
			+ MaybeSerializeDeserialize
			+ sp_std::fmt::Debug
			+ Default
			+ From<u64>
			+ IntegerSquareRoot
			+ Zero
			+ TypeInfo
			+ MaxEncodedLen;

		type AssetId: Member
			+ Parameter
			+ Default
			+ Copy
			+ codec::HasCompact
			+ From<u32>
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ PartialOrd
			+ TypeInfo;

		type Assets: Inspect<Self::AccountId, AssetId = Self::AssetId, Balance = Self::AssetBalance>
			+ Create<Self::AccountId>
			+ InspectEnumerable<Self::AccountId>
			+ Mutate<Self::AccountId>
			+ MutateMetadata<Self::AccountId>
			+ Transfer<Self::AccountId>;

		type Items: NonFungiblesInspect<
				Self::AccountId,
				ItemId = Self::ItemId,
				CollectionId = Self::CollectionId,
			> + NonFungiblesTransfer<Self::AccountId>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		#[pallet::constant]
		type BuybackThreshold: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn assets_minted)]
	// TODO: query amount minted from pallet assets instead of storing it locally.
	// Add a public getter function to pallet assets.
	pub type AssetsMinted<T: Config> =
		StorageMap<_, Twox64Concat, AssetIdOf<T>, AssetBalanceOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn asset_to_nft)]
	// TODO: store information about Asset ID and the corresponding Collection and Item ID.
	pub type AssetToNft<T: Config> =
		StorageMap<_, Twox64Concat, AssetIdOf<T>, (T::CollectionId, T::ItemId), OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NFTLocked(T::CollectionId, T::ItemId),
		NFTUnlocked(T::CollectionId, T::ItemId),
	}

	#[pallet::error]
	pub enum Error<T> {
		AssetDataNotFound,
		NFTDataNotFound
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// TODO: correct weights
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2).ref_time())]
		/// Pallet's account must be funded before lock is possible!
		/// 5EYCAe5gjC5dxKPbV2GPQUetETjFNSYZsSwSurVTTXidSLbh
		pub fn lock_nft_create_asset(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			item_id: T::ItemId,
			asset_id: AssetIdOf<T>,
			beneficiary: T::AccountId,
			min_balance: AssetBalanceOf<T>,
			amount: AssetBalanceOf<T>,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			let admin_account_id = Self::pallet_account_id();

			Self::do_lock_nft(collection_id, item_id)?;
			Self::do_create_asset(asset_id, admin_account_id, min_balance)?;
			Self::do_mint_asset(asset_id, &beneficiary, amount)?;

			// Mutate this storage item to retain information about the amount minted.
			<AssetsMinted<T>>::try_mutate(
				asset_id,
				|assets_minted| -> Result<(), DispatchError> {
					match assets_minted.is_some() {
						true =>
							*assets_minted = Some(assets_minted.unwrap().saturating_add(amount)),
						false => *assets_minted = Some(amount),
					}

					Ok(())
				},
			)?;

			// Mutate this storage item to retain information about the asset created.
			<AssetToNft<T>>::try_mutate(asset_id, |nft_id| -> Result<(), DispatchError> {
				*nft_id = Some((collection_id, item_id));

				Ok(())
			})?;

			Self::deposit_event(Event::NFTLocked(collection_id, item_id));

			Ok(())
		}

		/// Return and burn a % of the asset, unlock the NFT. Currently 100% is the minimum
		/// threshold.
		// TODO: correct weights
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2).ref_time())]
		pub fn burn_asset_unlock_nft(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			amount: AssetBalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(<AssetToNft<T>>::contains_key(asset_id), Error::<T>::NFTDataNotFound);
			let (collection_id, item_id) = Self::get_nft_id(asset_id);
			Self::do_burn_asset(asset_id, &who, amount)?;

			// Mutate this storage item to retain information about the amount burned.
			<AssetsMinted<T>>::try_mutate(
				asset_id,
				|assets_minted| -> Result<(), DispatchError> {
					*assets_minted = Some(assets_minted.unwrap().saturating_sub(amount));
					Ok(())
				},
			)?;

			Self::do_unlock_nft(collection_id, item_id, asset_id, who)?;

			Self::deposit_event(Event::NFTUnlocked(collection_id, item_id));

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn pallet_account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		/// Transfer the NFT from the account locking the NFT to the pallet's account.
		fn do_lock_nft(collection_id: T::CollectionId, item_id: T::ItemId) -> DispatchResult {
			let admin_account_id = Self::pallet_account_id();
			T::Items::transfer(&collection_id, &item_id, &admin_account_id)
		}

		/// Transfer the NFT to the account returning the tokens. Remove the key and value from
		/// storage.
		fn do_unlock_nft(
			collection_id: T::CollectionId,
			item_id: T::ItemId,
			asset_id: AssetIdOf<T>,
			account: T::AccountId,
		) -> DispatchResult {
			match T::Items::transfer(&collection_id, &item_id, &account) {
				Ok(()) => {
					<AssetToNft<T>>::take(asset_id);
					return Ok(())
				},
				Err(e) => return Err(e),
			}
		}

		/// Assert that the `asset_id` was created by means of locking an NFT and fetch
		/// its `CollectionId` and `ItemId`.
		fn get_nft_id(asset_id: AssetIdOf<T>) -> (T::CollectionId, T::ItemId) {
			// Check for explicit existence of the value in the extrinsic.
			<AssetToNft<T>>::get(asset_id).unwrap()
		}

		/// Create the new asset.
		fn do_create_asset(
			asset_id: AssetIdOf<T>,
			admin: T::AccountId,
			min_balance: AssetBalanceOf<T>,
		) -> DispatchResult {
			T::Assets::create(asset_id, admin, true, min_balance)
		}

		/// Mint the `amount` of tokens with `asset_id` into the beneficiary's account.
		fn do_mint_asset(
			asset_id: AssetIdOf<T>,
			beneficiary: &T::AccountId,
			amount: AssetBalanceOf<T>,
		) -> DispatchResult {
			T::Assets::mint_into(asset_id, beneficiary, amount)
		}

		/// If the amount of tokens is enough for the buyback, burn the tokens from the
		/// account that is returning the tokens.
		fn do_burn_asset(
			asset_id: AssetIdOf<T>,
			account: &T::AccountId,
			amount: AssetBalanceOf<T>,
		) -> Result<AssetBalanceOf<T>, DispatchError> {
			// Assert that the asset exists in storage.
			ensure!(<AssetsMinted<T>>::contains_key(asset_id), Error::<T>::NFTDataNotFound);
			Self::check_token_amount(asset_id, amount);
			T::Assets::burn_from(asset_id, account, amount)
		}

		/// Assert that the amount of tokens returned is equal to the amount needed to buy
		/// back the locked NFT.
		fn check_token_amount(asset_id: AssetIdOf<T>, amount: AssetBalanceOf<T>) -> () {
			// TODO: create a threshold of tokens to return in order to get back the NFT.
			// Otherwise one person can hold one token in order not to let others buy back.
			let buyback_threshold: AssetBalanceOf<T> =
				T::BuybackThreshold::get().saturated_into::<AssetBalanceOf<T>>();
			assert_eq!(
				Some(amount),
				Some(<AssetsMinted<T>>::get(asset_id).unwrap().saturating_mul(buyback_threshold))
			);
		}
	}
}