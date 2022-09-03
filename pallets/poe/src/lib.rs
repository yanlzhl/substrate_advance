#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
/// 导出poe模块内容
pub use frame_system::pallet::*;

// template也是如此定义的
// 涉及模块引用知识点 需要复习下
#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, BoundedVec};
	use frame_system::pallet_prelude::*;
	//集合类
	use sp_std::prelude::*;

	//模块配置接口
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// 关联类型
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber),
	>;

	//存证操作相关的事件
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreat(T::AccountId, Vec<u8>),
		ClaimRevoke(T::AccountId, Vec<u8>),
	}

	// 异常类型
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	// 类似生命周期环绕函数的调用
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	//可调用函数
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn creat_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let bound_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			ensure!(!Proofs::<T>::contains_key(&bound_claim), Error::<T>::ProofAlreadyExist);

			// (T::AccountId,T::BlockNumber)
			Proofs::<T>::insert(
				&bound_claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);

			// 发送事件
			Self::deposit_event(Event::ClaimCreat(sender, claim));

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let bound_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			let (owner, _) = Proofs::<T>::get(&bound_claim).ok_or(Error::<T>::ClaimNotExist)?;

			//判断存证是否存在
			ensure!(!Proofs::<T>::contains_key(&bound_claim), Error::<T>::ProofAlreadyExist);

			//确定所有权
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			//撤销存证
			Proofs::<T>::remove(&bound_claim);

			// 发送事件
			Self::deposit_event(Event::ClaimRevoke(sender, claim));

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			receiver: T::AccountId,
			proof: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let sender = ensure_signed(origin)?;

			let bound_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(proof.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			// Get Owner and verify that the specified proof has been claimed.
			let (owner, _) = Proofs::<T>::get(&bound_claim).ok_or(Error::<T>::ClaimNotExist)?;

			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotClaimOwner);

			// Get the block number from the FRAME System pallet.
			let current_block = <frame_system::Pallet<T>>::block_number();

			// Store the proof with the receiver and block number.
			Proofs::<T>::insert(&bound_claim, (receiver.clone(), current_block));

			// Emit an event that the claim was transfered.
			// Self::deposit_event(Event::TransferCreated(receiver, proof));

			Ok(().into())
		}
	}
}
