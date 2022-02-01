#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>


pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
        sp_runtime::traits::{Hash, Zero},
        dispatch::{DispatchResultWithPostInfo, DispatchResult},
        traits::{Currency, ExistenceRequirement, Randomness},
        pallet_prelude::*
    };
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_128;
	use scale_info::TypeInfo;

	#[cfg(feature ="std")]
	use frame_support::serde::{Serialize, Deserialize};

	//kitty struct
	type AccountOf<T> =<T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountOf<T>>>::Balance;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub dna: [u8; 16],
		pub price: Option<BalanceOf<T>>,
		pub gender: Gender,
		pub owner: AccountOf<T>
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Gender {
		Male,
		Female,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Currency: Currency<Self::AccountId>;
		type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;
		type MaxKittyOwned: Get<u32>;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn kitty_count)]

	pub(super) type KittyTotal<T: Config> = StorageValue<_,u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties_map)]

	pub(super) type KittiesMap<T: Config> = StorageMap<_,Twox64Concat,T::Hash,Kitty<T>>;
	
	#[pallet::storage]
	#[pallet::getter(fn kitties_owned)]

	pub(super) type KittiesOwned<T:Config> =StorageMap<_,Twox64Concat,T::AccountId, BoundedVec<T::Hash,T::MaxKittyOwned>, ValueQuery>; 

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created(T::AccountId, T::Hash),

		PriceSet(T::AccountId, T::Hash, Option<BalanceOf<T>>),

		Transferred(T::AccountId, T::AccountId, T::Hash),

		Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
		
		Total(u64),
	}

	
	#[pallet::error]
	pub enum Error<T> {

		KittyCountOverflow,

		LimitExceededForStoringKittiesPerAccount,

	}

	
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(100)]
		pub fn create_kitty(origin:OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let kitty_id = Self::mint(&sender,None,None)?;

			log::info!("A kitty is born bitch ID: {:?}",kitty_id);

			Self::deposit_event(Event::Created(sender, kitty_id));

			Ok(())
		}

		#[pallet::weight(10)]
		pub fn retrieve_total_kitties(origin:OriginFor<T>) ->DispatchResult {
			ensure_signed(origin)?;
			let total = Self::kitty_count();
			Self::deposit_event(Event::Total(total));
			Ok(())
		}

		
	}

    //helper functions
	impl <T: Config> Pallet<T> {
		fn gen_gender() -> Gender {
			let random = T::KittyRandomness::random(&b"gender"[..]).0;
			match random.as_ref()[0] % 2 {
				0 => Gender::Male,
				_=> Gender::Female,
			}
		}


		fn gen_dna() -> [u8;16] {
			let payload = (
				T::KittyRandomness::random(&b"blood"[..]).0,
				<frame_system::Pallet<T>>::block_number(),
			);
			payload.using_encoded(blake2_128)
		}

		pub fn mint(owner:&T::AccountId, dna: Option<[u8;16]>, gender: Option<Gender>) -> Result<T::Hash,Error<T>> {
			let kitty = Kitty::<T> {
				dna: dna.unwrap_or_else(Self::gen_dna),
				price: None,
				gender: gender.unwrap_or_else(Self::gen_gender),
				owner: owner.clone()
			};

			let kitty_id = T::Hashing::hash_of(&kitty);

			let new_count = Self::kitty_count().checked_add(1)
			                .ok_or(<Error<T>>::KittyCountOverflow)?;

			//adding ktties's hash to a total no of kitties owned per account				
			<KittiesOwned<T>>::try_mutate(&owner, |kitty_vec| {
				kitty_vec.try_push(kitty_id)
			}).map_err(|_| <Error<T>>::LimitExceededForStoringKittiesPerAccount)?;
			
			//Storing kitty struct instance mapping to its hash
			<KittiesMap<T>>::insert(kitty_id,kitty);

			//Incrementing Total Kitties into Storage
			<KittyTotal<T>>::put(new_count);

			Ok(kitty_id)

		}
	}
}
