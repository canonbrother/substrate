// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

use enumflags2::bitflags;
use codec::{Encode, Decode, MaxEncodedLen};
use scale_info::TypeInfo;
use frame_support::RuntimeDebug;

// Support for up to 64 user-enabled features on a token.
#[bitflags]
#[repr(u64)]
#[derive(Copy, Clone, RuntimeDebug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum UserFeatures {
	Administration,
}

// Support for up to 64 system-enabled features on a token.
#[bitflags]
#[repr(u64)]
#[derive(Copy, Clone, RuntimeDebug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum SystemFeatures {
	NoDeposit,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct TokenConfig {
	pub system_features: SystemFeatures,
	pub user_features: UserFeatures,
}
