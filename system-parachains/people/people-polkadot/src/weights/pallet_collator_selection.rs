// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
// for a list of specific contributors.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! Autogenerated weights for `pallet_collator_selection`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./people-polkadot-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=./people-polkadot-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=pallet_collator_selection
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./people-polkadot-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_collator_selection`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_selection::WeightInfo for WeightInfo<T> {
	/// Storage: `Session::NextKeys` (r:20 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::Invulnerables` (r:0 w:1)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 20]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `197 + b * (79 ±0)`
		//  Estimated: `1188 + b * (2555 ±0)`
		// Minimum execution time: 10_789_000 picoseconds.
		Weight::from_parts(8_042_445, 0)
			.saturating_add(Weight::from_parts(0, 1188))
			// Standard Error: 7_586
			.saturating_add(Weight::from_parts(3_215_553, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2555).saturating_mul(b.into()))
	}
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:1)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 19]`.
	/// The range of component `c` is `[1, 99]`.
	fn add_invulnerable(b: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `795 + b * (32 ±0) + c * (53 ±0)`
		//  Estimated: `6287 + b * (37 ±0) + c * (53 ±0)`
		// Minimum execution time: 38_784_000 picoseconds.
		Weight::from_parts(33_483_179, 0)
			.saturating_add(Weight::from_parts(0, 6287))
			// Standard Error: 18_928
			.saturating_add(Weight::from_parts(354_164, 0).saturating_mul(b.into()))
			// Standard Error: 3_588
			.saturating_add(Weight::from_parts(278_603, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 37).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 53).saturating_mul(c.into()))
	}
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:0)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:1)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[5, 20]`.
	fn remove_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `119 + b * (32 ±0)`
		//  Estimated: `6287`
		// Minimum execution time: 11_128_000 picoseconds.
		Weight::from_parts(10_875_605, 0)
			.saturating_add(Weight::from_parts(0, 6287))
			// Standard Error: 3_139
			.saturating_add(Weight::from_parts(178_803, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CollatorSelection::DesiredCandidates` (r:0 w:1)
	/// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_desired_candidates() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_839_000 picoseconds.
		Weight::from_parts(4_193_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CollatorSelection::CandidacyBond` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:100 w:100)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:100)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 100]`.
	/// The range of component `k` is `[0, 100]`.
	fn set_candidacy_bond(c: u32, k: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (182 ±0) + k * (115 ±0)`
		//  Estimated: `6287 + c * (901 ±29) + k * (901 ±29)`
		// Minimum execution time: 8_841_000 picoseconds.
		Weight::from_parts(9_232_000, 0)
			.saturating_add(Weight::from_parts(0, 6287))
			// Standard Error: 161_183
			.saturating_add(Weight::from_parts(5_473_032, 0).saturating_mul(c.into()))
			// Standard Error: 161_183
			.saturating_add(Weight::from_parts(5_062_373, 0).saturating_mul(k.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(k.into())))
			.saturating_add(Weight::from_parts(0, 901).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 901).saturating_mul(k.into()))
	}
	/// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	/// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[3, 100]`.
	fn update_bond(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `319 + c * (49 ±0)`
		//  Estimated: `6287`
		// Minimum execution time: 23_675_000 picoseconds.
		Weight::from_parts(24_628_653, 0)
			.saturating_add(Weight::from_parts(0, 6287))
			// Standard Error: 3_195
			.saturating_add(Weight::from_parts(235_225, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	/// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 99]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `765 + c * (52 ±0)`
		//  Estimated: `6287 + c * (54 ±0)`
		// Minimum execution time: 31_408_000 picoseconds.
		Weight::from_parts(35_038_583, 0)
			.saturating_add(Weight::from_parts(0, 6287))
			// Standard Error: 4_384
			.saturating_add(Weight::from_parts(270_443, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 54).saturating_mul(c.into()))
	}
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	/// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:2)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[3, 100]`.
	fn take_candidate_slot(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `905 + c * (53 ±0)`
		//  Estimated: `6287 + c * (54 ±0)`
		// Minimum execution time: 49_505_000 picoseconds.
		Weight::from_parts(53_836_940, 0)
			.saturating_add(Weight::from_parts(0, 6287))
			// Standard Error: 4_586
			.saturating_add(Weight::from_parts(293_412, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 54).saturating_mul(c.into()))
	}
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[3, 100]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `347 + c * (48 ±0)`
		//  Estimated: `6287`
		// Minimum execution time: 27_146_000 picoseconds.
		Weight::from_parts(29_282_116, 0)
			.saturating_add(Weight::from_parts(0, 6287))
			// Standard Error: 3_110
			.saturating_add(Weight::from_parts(238_927, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `System::BlockWeight` (r:1 w:1)
	/// Proof: `System::BlockWeight` (`max_values`: Some(1), `max_size`: Some(48), added: 543, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn note_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `155`
		//  Estimated: `6196`
		// Minimum execution time: 39_317_000 picoseconds.
		Weight::from_parts(40_142_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `CollatorSelection::CandidateList` (r:1 w:0)
	/// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:100 w:0)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::DesiredCandidates` (r:1 w:0)
	/// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::BlockWeight` (r:1 w:1)
	/// Proof: `System::BlockWeight` (`max_values`: Some(1), `max_size`: Some(48), added: 543, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:97 w:97)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	/// The range of component `c` is `[1, 100]`.
	fn new_session(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2302 + c * (97 ±0) + r * (114 ±0)`
		//  Estimated: `6287 + c * (2519 ±0) + r * (2603 ±0)`
		// Minimum execution time: 17_561_000 picoseconds.
		Weight::from_parts(17_854_000, 0)
			.saturating_add(Weight::from_parts(0, 6287))
			// Standard Error: 288_319
			.saturating_add(Weight::from_parts(12_756_367, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2519).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(r.into()))
	}
}
