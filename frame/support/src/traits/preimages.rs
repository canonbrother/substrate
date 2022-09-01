// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
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

//! Stuff for dealing with 32-byte hashed preimages.

use codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use sp_core::{RuntimeDebug, H256};
use sp_io::hashing::blake2_256;
use sp_runtime::{traits::ConstU32, DispatchError};
use sp_std::borrow::Cow;

pub type Hash = H256;
pub type BoundedInline = crate::BoundedVec<u8, ConstU32<128>>;

#[derive(
	Encode, Decode, MaxEncodedLen, Clone, Eq, PartialEq, scale_info::TypeInfo, RuntimeDebug,
)]
#[codec(mel_bound())]
pub enum Bounded<T> {
	/// A Blake2 256 hash with no preimage length. We
	/// do not support creation of this except for transitioning from legacy state.
	/// In the future we will make this a pure `Dummy` item storing only the final `dummy` field.
	Legacy { hash: Hash, dummy: sp_std::marker::PhantomData<T> },
	/// A an bounded `Call`. Its encoding must be at most 128 bytes.
	Inline(BoundedInline),
	/// A Blake2-256 hash of the call together with an upper limit for its size.
	Lookup { hash: Hash, len: u32 },
}

impl<T> Bounded<T> {
	pub fn transmute<S: Encode>(self) -> Bounded<S>
	where
		T: Encode + EncodeLike<S>,
	{
		use Bounded::*;
		match self {
			Legacy { hash, .. } => Legacy { hash, dummy: sp_std::marker::PhantomData },
			Inline(x) => Inline(x),
			Lookup { hash, len } => Lookup { hash, len },
		}
	}

	pub fn hash(&self) -> H256 {
		use Bounded::*;
		match self {
			Legacy { hash, .. } => *hash,
			Inline(x) => blake2_256(x.as_ref()).into(),
			Lookup { hash, .. } => *hash,
		}
	}
}

// The maximum we expect a single legacy hash lookup to be.
const MAX_LEGACY_LEN: u32 = 1_000_000;

impl<T> Bounded<T> {
	pub fn len(&self) -> Option<u32> {
		match self {
			Self::Legacy { .. } => None,
			Self::Inline(i) => Some(i.len() as u32),
			Self::Lookup { len, .. } => Some(*len),
		}
	}
	pub fn lookup_needed(&self) -> bool {
		match self {
			Self::Inline(..) => false,
			Self::Legacy { .. } | Self::Lookup { .. } => true,
		}
	}
	pub fn lookup_len(&self) -> Option<u32> {
		match self {
			Self::Inline(..) => None,
			Self::Legacy { .. } => Some(MAX_LEGACY_LEN),
			Self::Lookup { len, .. } => Some(*len),
		}
	}
	pub fn unrequested(hash: Hash, len: u32) -> Self {
		Self::Lookup { hash, len }
	}

	#[deprecated = "This API is only for transitioning to Scheduler v3 API"]
	pub fn from_legacy_hash(hash: impl Into<Hash>) -> Self {
		Self::Legacy { hash: hash.into(), dummy: sp_std::marker::PhantomData }
	}
}

pub type FetchResult = Result<Cow<'static, [u8]>, DispatchError>;

/// A interface for looking up preimages from their hash on chain.
pub trait QueryPreimage {
	/// Returns whether a preimage exists for a given hash and if so its length.
	fn len(hash: &Hash) -> Option<u32>;

	/// Returns the preimage for a given hash. If given, `len` must be the size of the preimage.
	fn fetch(hash: &Hash, len: Option<u32>) -> FetchResult;

	/// Returns whether a preimage request exists for a given hash.
	fn is_requested(hash: &Hash) -> bool;

	/// Request that someone report a preimage. Providers use this to optimise the economics for
	/// preimage reporting.
	fn request(hash: &Hash);

	/// Cancel a previous preimage request.
	fn unrequest(hash: &Hash);

	/// Request that the data required for decoding the given `bounded` value is made available.
	fn hold<T>(bounded: &Bounded<T>) {
		use Bounded::*;
		match bounded {
			Inline(..) => {},
			Legacy { hash, .. } | Lookup { hash, .. } => Self::request(hash),
		}
	}

	/// No longer request that the data required for decoding the given `bounded` value is made
	/// available.
	fn drop<T>(bounded: &Bounded<T>) {
		use Bounded::*;
		match bounded {
			Inline(..) => {},
			Legacy { hash, .. } | Lookup { hash, .. } => Self::unrequest(hash),
		}
	}

	/// Check to see if all data required for the given `bounded` value is available for its
	/// decoding.
	fn have<T>(bounded: &Bounded<T>) -> bool {
		use Bounded::*;
		match bounded {
			Inline(..) => true,
			Legacy { hash, .. } | Lookup { hash, .. } => Self::len(hash).is_some(),
		}
	}

	/// Create a `Bounded` instance based on the `hash` and `len` of the encoded value. This may not
	/// be `peek`-able or `realize`-able.
	fn pick<T>(hash: Hash, len: u32) -> Bounded<T> {
		Self::request(&hash);
		Bounded::Lookup { hash, len }
	}

	/// Convert the given `bounded` instance back into its original instance, also returning the
	/// exact size of its encoded form if it needed to be looked-up from a stored preimage).
	///
	/// NOTE: This does not remove any data needed for realization. If you will no longer use the
	/// `bounded`, call `realize` instead or call `drop` afterwards.
	fn peek<T: Decode>(bounded: &Bounded<T>) -> Result<(T, Option<u32>), DispatchError> {
		use Bounded::*;
		match bounded {
			Inline(data) => T::decode(&mut &data[..]).ok().map(|x| (x, None)),
			Lookup { hash, len } => {
				let data = Self::fetch(hash, Some(*len))?;
				T::decode(&mut &data[..]).ok().map(|x| (x, Some(data.len() as u32)))
			},
			Legacy { hash, .. } => {
				let data = Self::fetch(hash, None)?;
				T::decode(&mut &data[..]).ok().map(|x| (x, Some(data.len() as u32)))
			},
		}
		.ok_or(DispatchError::Corruption)
	}

	/// Convert the given `bounded` value back into its original instance. If successful,
	/// `drop` any data backing it. This will not break the realisability of independently
	/// created instances of `Bounded` which happen to have identical data.
	fn realize<T: Decode>(bounded: &Bounded<T>) -> Result<(T, Option<u32>), DispatchError> {
		let r = Self::peek(bounded)?;
		Self::drop(bounded);
		Ok(r)
	}
}

/// A interface for managing preimages to hashes on chain.
///
/// Note that this API does not assume any underlying user is calling, and thus
/// does not handle any preimage ownership or fees. Other system level logic that
/// uses this API should implement that on their own side.
pub trait StorePreimage: QueryPreimage {
	/// The maximum length of preimage we can store.
	const MAX_LENGTH: usize;

	/// Request and attempt to store the bytes of a preimage on chain.
	///
	/// May return `DispatchError::Exhausted` if the preimage is just too big.
	fn note(bytes: Cow<[u8]>) -> Result<Hash, DispatchError>;

	/// Attempt to clear a previously noted preimage. Exactly the same as `unrequest` but is
	/// provided for symmetry.
	fn unnote(hash: &Hash) {
		Self::unrequest(hash)
	}

	/// Convert an otherwise unbounded or large value into a type ready for placing in storage. The
	/// result is a type whose `MaxEncodedLen` is 131 bytes.
	///
	/// NOTE: Once this API is used, you should use either `drop` or `realize`.
	fn bound<T: Encode>(t: T) -> Result<Bounded<T>, DispatchError> {
		let data = t.encode();
		let len = data.len() as u32;
		Ok(match BoundedInline::try_from(data) {
			Ok(bounded) => Bounded::Inline(bounded),
			Err(unbounded) => Bounded::Lookup { hash: Self::note(unbounded.into())?, len },
		})
	}
}

impl QueryPreimage for () {
	fn len(_: &Hash) -> Option<u32> {
		None
	}
	fn fetch(_: &Hash, _: Option<u32>) -> FetchResult {
		Err(DispatchError::Unavailable)
	}
	fn is_requested(_: &Hash) -> bool {
		false
	}
	fn request(_: &Hash) {}
	fn unrequest(_: &Hash) {}
}

impl StorePreimage for () {
	const MAX_LENGTH: usize = 0;
	fn note(_: Cow<[u8]>) -> Result<Hash, DispatchError> {
		Err(DispatchError::Exhausted)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn bounded_size_is_correct() {
		assert_eq!(<Bounded<Vec<u8>> as MaxEncodedLen>::max_encoded_len(), 131);
	}
}