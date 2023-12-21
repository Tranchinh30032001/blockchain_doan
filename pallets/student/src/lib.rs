#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::{DispatchResult, OptionQuery, *},
		sp_std::vec::Vec,
		storage::types::{StorageDoubleMap, StorageMap},
		Blake2_128,
	};
	use frame_system::{
		ensure_signed,
		pallet_prelude::{OriginFor, *},
	};

	#[derive(Encode, Decode, TypeInfo, Debug, Default, Clone)]
	pub struct Student {
		mssv: Vec<u8>,
		name: Vec<u8>,
		lop_id: Vec<u8>,
		khoa_id: Vec<u8>,
		ngay_sinh: Vec<u8>,
		email: Vec<u8>,
		role_id: u8,
		que_quan: Vec<u8>,
		cccd: Vec<u8>,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_template::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallSelf::convert_str_to_slice(&mssv)et
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn map_double_key)]
	#[pallet::unbounded]
	pub type Map_Double_Key<T: Config> = StorageDoubleMap<
		_,
		Blake2_128,
		T::AccountId,
		Blake2_128,
		Vec<u8>,
		Vec<Student>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn map_lop_st)]
	#[pallet::unbounded]
	pub type Map_Lop_Student<T: Config> =
		StorageMap<_, Blake2_128, Vec<u8>, Vec<Student>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn map_mssv_student)]
	#[pallet::unbounded]
	pub type Map_Mssv_Student<T: Config> = StorageMap<_, Blake2_128, Vec<u8>, Student, OptionQuery>;
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored {
			something: u32,
			who: T::AccountId,
		},
		CreateStudent {
			account: T::AccountId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		NotFoundStudent,
		StudentExist,
	}
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]

		pub fn create_student(
			origin: OriginFor<T>,
			accountId: T::AccountId,
			mssv: Vec<u8>,
			name: Vec<u8>,
			lop_id: Vec<u8>,
			khoa_id: Vec<u8>,
			ngay_sinh: Vec<u8>,
			email: Vec<u8>,
			role_id: u8,
			que_quan: Vec<u8>,
			cccd: Vec<u8>,
		) -> DispatchResult {
			let _ = ensure_root(origin)?;
			if let Some(res) = Map_Mssv_Student::<T>::get(&mssv) {
				return Err(Error::<T>::StudentExist.into())
			} else {
				let new_student = Student {
					mssv: mssv.clone(),
					name,
					lop_id: lop_id.clone(),
					khoa_id,
					ngay_sinh,
					email,
					role_id,
					que_quan,
					cccd,
				};

				match Map_Lop_Student::<T>::get(&lop_id) {
					Some(mut res) => {
						res.push(new_student.clone());
						Map_Lop_Student::<T>::insert(&lop_id, res);
					},
					None => {
						let mut new_vec: Vec<Student> = Vec::new();
						new_vec.push(new_student.clone());
						Map_Lop_Student::<T>::insert(&lop_id, new_vec);
					},
				}

				match Map_Double_Key::<T>::get(&accountId, &lop_id) {
					Some(mut res) => {
						res.push(new_student.clone());
						Map_Double_Key::<T>::insert(&accountId, &lop_id, res);
					},
					None => {
						let mut new_vec: Vec<Student> = Vec::new();
						new_vec.push(new_student.clone());
						Map_Double_Key::<T>::insert(&accountId, &lop_id, new_vec);
					},
				}

				<Map_Mssv_Student<T>>::insert(&mssv, new_student.clone());
				Self::deposit_event(Event::CreateStudent { account: accountId });
				Ok(())
			}
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]

		pub fn update_student(
			origin: OriginFor<T>,
			accountId: T::AccountId,
			mssv: Vec<u8>,
			name: Vec<u8>,
			lop_id: Vec<u8>,
			khoa_id: Vec<u8>,
			ngay_sinh: Vec<u8>,
			email: Vec<u8>,
			role_id: u8,
			que_quan: Vec<u8>,
			cccd: Vec<u8>,
		) -> DispatchResult {
			let _ = ensure_root(origin)?;
			let new_student = match Map_Mssv_Student::<T>::get(&mssv) {
				Some(mut res) => {
					res.name = name;
					res.mssv = mssv.clone();
					res.lop_id = lop_id.clone();
					res.ngay_sinh = ngay_sinh;
					res.email = email;
					res.role_id = role_id;
					res.que_quan = que_quan;
					res.cccd = cccd;
					res
				},
				None => return Err(Error::<T>::NotFoundStudent.into()),
			};

			match Map_Lop_Student::<T>::get(&lop_id) {
				Some(mut res) => {
					for (index, item) in res.clone().into_iter().enumerate() {
						if item.mssv == mssv {
							res.remove(index);
						}
					}
					res.push(new_student.clone());
					Map_Lop_Student::<T>::insert(&lop_id, res);
				},
				None => return Err(Error::<T>::NotFoundStudent.into()),
			}

			match Map_Double_Key::<T>::get(&accountId, &lop_id) {
				Some(mut res) => {
					for (index, item) in res.clone().into_iter().enumerate() {
						if item.mssv == mssv {
							res.remove(index);
						}
					}
					res.push(new_student.clone());
					Map_Double_Key::<T>::insert(&accountId, &lop_id, res);
				},
				None => return Err(Error::<T>::NotFoundStudent.into()),
			}

			Map_Mssv_Student::<T>::insert(&mssv, &new_student);
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]

		pub fn delete_student(
			origin: OriginFor<T>,
			accountId: T::AccountId,
			mssv: Vec<u8>,
		) -> DispatchResult {
			let _ = ensure_root(origin)?;
			let lop_id;

			match Map_Mssv_Student::<T>::get(&mssv) {
				Some(res) => {
					lop_id = res.lop_id;
					Map_Mssv_Student::<T>::remove(&mssv);
				},
				None => return Err(Error::<T>::NotFoundStudent.into()),
			}

			match Map_Lop_Student::<T>::get(&lop_id) {
				Some(mut res) => {
					for (index, item) in res.clone().into_iter().enumerate() {
						if item.mssv == mssv {
							res.remove(index);
						}
					}
					Map_Lop_Student::<T>::insert(&lop_id, res);
				},
				None => return Err(Error::<T>::NotFoundStudent.into()),
			}

			match Map_Double_Key::<T>::get(&accountId, &lop_id) {
				Some(mut res) => {
					for (index, item) in res.clone().into_iter().enumerate() {
						if item.mssv == mssv {
							res.remove(index);
						}
					}
					Map_Double_Key::<T>::insert(&accountId, &lop_id, res);
				},
				None => return Err(Error::<T>::NotFoundStudent.into()),
			}

			Ok(())
		}
	}
}
