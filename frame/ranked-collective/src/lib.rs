// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Ranked collective system: Members of a set of account IDs can make their collective feelings
//! known through dispatched calls from one of two specialized origins.
//!
//! The membership can be provided in one of two ways: either directly, using the Root-dispatchable
//! function `set_members`, or indirectly, through implementing the `ChangeMembers`.
//! The pallet assumes that the amount of members stays at or below `MaxMembers` for its weight
//! calculations, but enforces this neither in `set_members` nor in `change_members_sorted`.
//!
//! A "prime" member may be set to help determine the default vote behavior based on chain
//! config. If `PrimeDefaultVote` is used, the prime vote acts as the default vote in case of any
//! abstentions after the voting period. If `MoreThanMajorityThenPrimeDefaultVote` is used, then
//! abstentions will first follow the majority of the collective voting, and then the prime
//! member.
//!
//! Voting happens through motions comprising a proposal (i.e. a curried dispatchable) plus a
//! number of approvals required for it to pass and be called. Motions are open for members to
//! vote on for a minimum period given by `MotionDuration`. As soon as the needed number of
//! approvals is given, the motion is closed and executed. If the number of approvals is not reached
//! during the voting period, then `close` may be called by any account in order to force the end
//! the motion explicitly. If a prime member is defined then their vote is used in place of any
//! abstentions and the proposal is executed if there are enough approvals counting the new votes.
//!
//! If there are not, or if no prime is set, then the motion is dropped without being executed.

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "128"]

use scale_info::TypeInfo;
use sp_arithmetic::traits::Saturating;
use sp_runtime::{
	traits::Convert,
	ArithmeticError::Overflow,
	Perbill, RuntimeDebug,
};
use sp_std::{marker::PhantomData, prelude::*};

use frame_support::{
	codec::{Decode, Encode, MaxEncodedLen},
	dispatch::{DispatchError, DispatchResultWithPostInfo},
	ensure,
	traits::{EnsureOrigin, PollStatus, Polling, VoteTally},
	weights::PostDispatchInfo,
	CloneNoBound, EqNoBound, PartialEqNoBound, RuntimeDebugNoBound,
};

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

/// A number of members.
pub type MemberIndex = u32;

/// Member rank.
pub type Rank = u16;

/// Votes.
pub type Votes = u32;

/// Aggregated votes for an ongoing poll.
#[derive(
	CloneNoBound,
	PartialEqNoBound,
	EqNoBound,
	RuntimeDebugNoBound,
	TypeInfo,
	Encode,
	Decode,
	MaxEncodedLen,
)]
#[scale_info(skip_type_params(M))]
pub struct Tally<M: GetMaxVoters> {
	bare_ayes: MemberIndex,
	ayes: Votes,
	nays: Votes,
	dummy: PhantomData<M>,
}

impl<M: GetMaxVoters> Tally<M> {
	fn from_parts(bare_ayes: MemberIndex, ayes: Votes, nays: Votes) -> Self {
		Tally { bare_ayes, ayes, nays, dummy: PhantomData }
	}
}

// Use (non-rank-weighted) ayes for calculating support.
// Allow only promotion/demotion by one rank only.
// Allow removal of member with rank zero only.
// This keeps everything O(1) while still allowing arbitrary number of ranks.

// All functions of VoteTally now include the class as a param.
// TODO: ** BEFORE COMMIT ** split and move into gg2t branch.

pub type TallyOf<T, I = ()> = Tally<Pallet<T, I>>;
pub type PollIndexOf<T, I = ()> = <<T as Config<I>>::Polls as Polling<TallyOf<T, I>>>::Index;

impl<M: GetMaxVoters> VoteTally<Votes, Rank> for Tally<M> {
	fn new(_: Rank) -> Self {
		Self { bare_ayes: 0, ayes: 0, nays: 0, dummy: PhantomData }
	}
	fn ayes(&self, _: Rank) -> Votes {
		self.bare_ayes
	}
	fn support(&self, class: Rank) -> Perbill {
		Perbill::from_rational(self.bare_ayes, M::get_max_voters(class))
	}
	fn approval(&self, _: Rank) -> Perbill {
		Perbill::from_rational(self.ayes, 1.max(self.ayes + self.nays))
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn unanimity(class: Rank) -> Self {
		Self {
			bare_ayes: M::get_max_voters(class),
			ayes: M::get_max_voters(class),
			nays: 0,
			dummy: PhantomData,
		}
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn rejection(class: Rank) -> Self {
		Self { bare_ayes: 0, ayes: 0, nays: M::get_max_voters(class), dummy: PhantomData }
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn from_requirements(support: Perbill, approval: Perbill, class: Rank) -> Self {
		let c = M::get_max_voters(class);
		let ayes = support * c;
		let nays = ((ayes as u64) * 1_000_000_000u64 / approval.deconstruct() as u64) as u32 - ayes;
		Self { bare_ayes: ayes, ayes, nays, dummy: PhantomData }
	}
}

/// Record needed for every member.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct MemberRecord {
	/// The rank of the member.
	rank: Rank,
}

/// Record needed for every vote.
#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum VoteRecord {
	/// Vote was an aye with given vote weight.
	Aye(Votes),
	/// Vote was a nay with given vote weight.
	Nay(Votes),
}

impl From<(bool, Votes)> for VoteRecord {
	fn from((aye, votes): (bool, Votes)) -> Self {
		match aye {
			true => VoteRecord::Aye(votes),
			false => VoteRecord::Nay(votes),
		}
	}
}

/// Vote-weight scheme where all voters get one vote regardless of rank.
pub struct Unit;
impl Convert<Rank, Votes> for Unit {
	fn convert(_: Rank) -> Votes {
		1
	}
}

/// Vote-weight scheme where all voters get one vote plus an additional vote for every excess rank
/// they have. I.e.:
///
/// - Each member with no excess rank gets 1 vote;
/// - ...with an excess rank of 1 gets 2 votes;
/// - ...with an excess rank of 2 gets 2 votes;
/// - ...with an excess rank of 3 gets 3 votes;
/// - ...with an excess rank of 4 gets 4 votes.
pub struct Linear;
impl Convert<Rank, Votes> for Linear {
	fn convert(r: Rank) -> Votes {
		(r + 1) as Votes
	}
}

/// Vote-weight scheme where all voters get one vote plus additional votes for every excess rank
/// they have incrementing by one vote for each excess rank. I.e.:
///
/// - Each member with no excess rank gets 1 vote;
/// - ...with an excess rank of 1 gets 2 votes;
/// - ...with an excess rank of 2 gets 3 votes;
/// - ...with an excess rank of 3 gets 6 votes;
/// - ...with an excess rank of 4 gets 10 votes.
pub struct Geometric;
impl Convert<Rank, Votes> for Geometric {
	fn convert(r: Rank) -> Votes {
		let v = (r + 1) as Votes;
		v * (v + 1) / 2
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// The outer event type.
		type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

		/// The origin required to add, promote or remove a member.
		type AdminOrigin: EnsureOrigin<Self::Origin>;

		/// The polling system used for our voting.
		type Polls: Polling<TallyOf<Self, I>, Votes = Votes, Class = Rank, Moment = Self::BlockNumber>;

		/// Convert a rank_delta into a number of votes the rank gets.
		///
		/// Rank_delta is defined as the number of ranks above the minimum required to take part
		/// in the poll.
		type VoteWeight: Convert<Rank, Votes>;
	}

	/// The number of members in the collective who have at least the rank according to the index
	/// of the vec.
	#[pallet::storage]
	pub type MemberCount<T: Config<I>, I: 'static = ()> =
		StorageMap<_, Twox64Concat, Rank, MemberIndex, ValueQuery>;

	/// The current members of the collective.
	#[pallet::storage]
	pub type Members<T: Config<I>, I: 'static = ()> =
		StorageMap<_, Twox64Concat, T::AccountId, MemberRecord>;

	/// The index of each ranks's member into the group of members who have at least that rank.
	#[pallet::storage]
	pub type IdToIndex<T: Config<I>, I: 'static = ()> =
		StorageDoubleMap<_, Twox64Concat, Rank, Twox64Concat, T::AccountId, MemberIndex>;

	/// The members in the collective by index. All indices in the range `0..MemberCount` will
	/// return `Some`, however a member's index is not guaranteed to remain unchanged over time.
	#[pallet::storage]
	pub type IndexToId<T: Config<I>, I: 'static = ()> =
		StorageDoubleMap<_, Twox64Concat, Rank, Twox64Concat, MemberIndex, T::AccountId>;

	/// Votes on a given proposal, if it is ongoing.
	#[pallet::storage]
	pub type Voting<T: Config<I>, I: 'static = ()> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		PollIndexOf<T, I>,
		Twox64Concat,
		T::AccountId,
		VoteRecord,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// A member `who` has been added.
		MemberAdded { who: T::AccountId },
		/// The member `who`'s rank has been changed to the given `rank`.
		RankChanged { who: T::AccountId, rank: Rank },
		/// The member `who` of given `rank` has been removed from the collective.
		MemberRemoved { who: T::AccountId, rank: Rank },
		/// The member `who` has voted for the `poll` with the given `vote` leading to an updated
		/// `tally`.
		Voted { who: T::AccountId, poll: PollIndexOf<T, I>, vote: VoteRecord, tally: TallyOf<T, I> },
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// Account is already a member.
		AlreadyMember,
		/// Account is not a member.
		NotMember,
		/// The given poll index is unknown or has closed.
		NotPolling,
		/// The given poll is still ongoing.
		Ongoing,
		/// There are no further records to be removed.
		NoneRemaining,
		/// Unexpected error in state.
		Corruption,
		/// The member's rank is too low to vote.
		RankTooLow,
		/// The information provided is incorrect.
		InvalidWitness,
	}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Introduce a new member.
		///
		/// - `origin`: Must be the `AdminOrigin`.
		/// - `who`: Account of non-member which will become a member.
		/// - `rank`: The rank to give the new member.
		///
		/// Weight: `O(1)`
		#[pallet::weight(T::WeightInfo::add_member())]
		pub fn add_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			ensure!(!Members::<T, I>::contains_key(&who), Error::<T, I>::AlreadyMember);
			let index = MemberCount::<T, I>::get(0);
			let count = index.checked_add(1).ok_or(Overflow)?;

			Members::<T, I>::insert(&who, MemberRecord { rank: 0 });
			IdToIndex::<T, I>::insert(0, &who, index);
			IndexToId::<T, I>::insert(0, index, &who);
			MemberCount::<T, I>::insert(0, count);
			Self::deposit_event(Event::MemberAdded { who });

			Ok(())
		}

		/// Increment the rank of an existing member by one.
		///
		/// - `origin`: Must be the `AdminOrigin`.
		/// - `who`: Account of existing member.
		///
		/// Weight: `O(1)`
		#[pallet::weight(T::WeightInfo::promote_member(0))]
		pub fn promote_member(
			origin: OriginFor<T>,
			who: T::AccountId,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			let record = Self::ensure_member(&who)?;
			let rank = record.rank.checked_add(1).ok_or(Overflow)?;
			let index = MemberCount::<T, I>::get(rank);
			MemberCount::<T, I>::insert(rank, index.checked_add(1).ok_or(Overflow)?);
			IdToIndex::<T, I>::insert(rank, &who, index);
			IndexToId::<T, I>::insert(rank, index, &who);
			Members::<T, I>::insert(&who, MemberRecord { rank, .. record });
			Self::deposit_event(Event::RankChanged { who, rank });

			Ok(())
		}

		/// Decrement the rank of an existing member by one. If the member is already at rank zero,
		/// then they are removed entirely.
		///
		/// - `origin`: Must be the `AdminOrigin`.
		/// - `who`: Account of existing member of rank greater than zero.
		///
		/// Weight: `O(1)`, less if the member's index is highest in its rank.
		#[pallet::weight(T::WeightInfo::demote_member(0))]
		pub fn demote_member(
			origin: OriginFor<T>,
			who: T::AccountId,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			let mut record = Self::ensure_member(&who)?;
			let rank = record.rank;

			Self::remove_from_rank(&who, rank)?;
			let maybe_rank = rank.checked_sub(1);
			match maybe_rank {
				None => {
					Members::<T, I>::remove(&who);
					Self::deposit_event(Event::MemberRemoved { who, rank: 0 });
				}
				Some(rank) => {
					record.rank = rank;
					Members::<T, I>::insert(&who, &record);
					Self::deposit_event(Event::RankChanged { who, rank });
				}
			}
			Ok(())
		}

		/// Remove the member entirely.
		///
		/// - `origin`: Must be the `AdminOrigin`.
		/// - `who`: Account of existing member of rank greater than zero.
		/// - `rank`: The rank of the member.
		///
		/// Weight: `O(rank)`.
		#[pallet::weight(T::WeightInfo::remove_member(*min_rank as u32))]
		pub fn remove_member(
			origin: OriginFor<T>,
			who: T::AccountId,
			min_rank: Rank,
		) -> DispatchResultWithPostInfo {
			T::AdminOrigin::ensure_origin(origin)?;
			let MemberRecord { rank, .. } = Self::ensure_member(&who)?;
			ensure!(min_rank >= rank, Error::<T, I>::InvalidWitness);

			for r in 0..=rank {
				Self::remove_from_rank(&who, r)?;
			}
			Members::<T, I>::remove(&who);
			Self::deposit_event(Event::MemberRemoved { who, rank });
			Ok(PostDispatchInfo {
				actual_weight: Some(T::WeightInfo::remove_member(rank as u32)),
				pays_fee: Pays::Yes,
			})
		}

		/// Add an aye or nay vote for the sender to the given proposal.
		///
		/// - `origin`: Must be `Signed` by a member account.
		/// - `poll`: Index of a poll which is ongoing.
		/// - `aye`: `true` if the vote is to approve the proposal, `false` otherwise.
		///
		/// Transaction fees are be waived if the member is voting on any particular proposal
		/// for the first time and the call is successful. Subsequent vote changes will charge a
		/// fee.
		///
		/// Weight: `O(1)`, less if there was no previous vote on the poll by the member.
		#[pallet::weight(T::WeightInfo::vote())]
		pub fn vote(
			origin: OriginFor<T>,
			poll: PollIndexOf<T, I>,
			aye: bool,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let record = Self::ensure_member(&who)?;
			use VoteRecord::*;
			let mut pays = Pays::Yes;

			let (tally, vote) = T::Polls::try_access_poll(poll, |mut status| -> Result<(TallyOf<T, I>, VoteRecord), DispatchError> {
				match status {
					PollStatus::None | PollStatus::Completed(..) => Err(Error::<T, I>::NotPolling)?,
					PollStatus::Ongoing(ref mut tally, min_rank) => {
						match Voting::<T, I>::get(&poll, &who) {
							Some(Aye(votes)) => {
								tally.bare_ayes.saturating_dec();
								tally.ayes.saturating_reduce(votes);
							},
							Some(Nay(votes)) => tally.nays.saturating_reduce(votes),
							None => pays = Pays::No,
						}
						let votes = Self::rank_to_votes(record.rank, min_rank)?;
						let vote = VoteRecord::from((aye, votes));
						match aye {
							true => {
								tally.bare_ayes.saturating_inc();
								tally.ayes.saturating_accrue(votes);
							},
							false => tally.nays.saturating_accrue(votes),
						}
						Voting::<T, I>::insert(&poll, &who, &vote);
						Ok((tally.clone(), vote))
					},
				}
			})?;
			Self::deposit_event(Event::Voted { who, poll, vote, tally });
			Ok(pays.into())
		}

		/// Remove votes from the given poll. It must have ended.
		///
		/// - `origin`: Must be `Signed` by any account.
		/// - `poll_index`: Index of a poll which is completed and for which votes continue to
		///   exist.
		/// - `max`: Maximum number of vote items from remove in this call.
		///
		/// Transaction fees are waived if the operation is successful.
		///
		/// Weight `O(max)` (less if there are fewer items to remove than `max`).
		#[pallet::weight(T::WeightInfo::cleanup_poll(*max))]
		pub fn cleanup_poll(
			origin: OriginFor<T>,
			poll_index: PollIndexOf<T, I>,
			max: u32,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;
			ensure!(T::Polls::as_ongoing(poll_index).is_none(), Error::<T, I>::Ongoing);

			use sp_io::KillStorageResult::*;
			let count = match Voting::<T, I>::remove_prefix(poll_index, Some(max)) {
//				AllRemoved(0) => Err(Error::<T, I>::NoneRemaining)?,
				AllRemoved(0) => return Ok(Pays::Yes.into()),
				AllRemoved(n) | SomeRemaining(n) => n,
			};
			Ok(PostDispatchInfo {
				actual_weight: Some(T::WeightInfo::cleanup_poll(count)),
				pays_fee: Pays::No,
			})
		}
	}

	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		fn ensure_member(who: &T::AccountId) -> Result<MemberRecord, DispatchError> {
			Members::<T, I>::get(who).ok_or(Error::<T, I>::NotMember.into())
		}

		fn rank_to_votes(rank: Rank, min: Rank) -> Result<Votes, DispatchError> {
			let excess = rank.checked_sub(min).ok_or(Error::<T, I>::RankTooLow)?;
			Ok(T::VoteWeight::convert(excess))
		}

		fn remove_from_rank(who: &T::AccountId, rank: Rank) -> DispatchResult {
			let last_index = MemberCount::<T, I>::get(rank).saturating_sub(1);
			let index = IdToIndex::<T, I>::get(rank, &who).ok_or(Error::<T, I>::Corruption)?;
			if index != last_index {
				let last = IndexToId::<T, I>::get(rank, last_index).ok_or(Error::<T, I>::Corruption)?;
				IdToIndex::<T, I>::insert(rank, &last, index);
				IndexToId::<T, I>::insert(rank, index, &last);
			}
			MemberCount::<T, I>::mutate(rank, |r| r.saturating_dec());
			Ok(())
		}
	}

	pub trait GetMaxVoters {
		fn get_max_voters(r: Rank) -> MemberIndex;
	}
	impl<T: Config<I>, I: 'static> GetMaxVoters for Pallet<T, I> {
		fn get_max_voters(r: Rank) -> MemberIndex {
			MemberCount::<T, I>::get(r)
		}
	}
}