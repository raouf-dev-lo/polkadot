// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_xcm`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_xcm
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_xcm`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm::WeightInfo for WeightInfo<T> {
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SupportedVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	fn send() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `612`
		//  Estimated: `12582`
		// Minimum execution time: 35_247 nanoseconds.
		Weight::from_parts(35_914_000, 0)
			.saturating_add(Weight::from_parts(0, 12582))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn teleport_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 20_897 nanoseconds.
		Weight::from_parts(21_217_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn reserve_transfer_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 19_523 nanoseconds.
		Weight::from_parts(19_814_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn execute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_282 nanoseconds.
		Weight::from_parts(9_523_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: XcmPallet SupportedVersion (r:0 w:1)
	/// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	fn force_xcm_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_524 nanoseconds.
		Weight::from_parts(9_873_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: XcmPallet SafeXcmVersion (r:0 w:1)
	/// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	fn force_default_xcm_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_609 nanoseconds.
		Weight::from_parts(2_747_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: XcmPallet VersionNotifiers (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionNotifiers (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet QueryCounter (r:1 w:1)
	/// Proof Skipped: XcmPallet QueryCounter (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SupportedVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet Queries (r:0 w:1)
	/// Proof Skipped: XcmPallet Queries (max_values: None, max_size: None, mode: Measured)
	fn force_subscribe_version_notify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `612`
		//  Estimated: `17388`
		// Minimum execution time: 40_493 nanoseconds.
		Weight::from_parts(41_628_000, 0)
			.saturating_add(Weight::from_parts(0, 17388))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: XcmPallet VersionNotifiers (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionNotifiers (max_values: None, max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SupportedVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet Queries (r:0 w:1)
	/// Proof Skipped: XcmPallet Queries (max_values: None, max_size: None, mode: Measured)
	fn force_unsubscribe_version_notify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `884`
		//  Estimated: `18457`
		// Minimum execution time: 43_846 nanoseconds.
		Weight::from_parts(44_377_000, 0)
			.saturating_add(Weight::from_parts(0, 18457))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: XcmPallet SupportedVersion (r:4 w:2)
	/// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	fn migrate_supported_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `229`
		//  Estimated: `10129`
		// Minimum execution time: 16_954 nanoseconds.
		Weight::from_parts(17_622_000, 0)
			.saturating_add(Weight::from_parts(0, 10129))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: XcmPallet VersionNotifiers (r:4 w:2)
	/// Proof Skipped: XcmPallet VersionNotifiers (max_values: None, max_size: None, mode: Measured)
	fn migrate_version_notifiers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `233`
		//  Estimated: `10133`
		// Minimum execution time: 17_064 nanoseconds.
		Weight::from_parts(17_460_000, 0)
			.saturating_add(Weight::from_parts(0, 10133))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: XcmPallet VersionNotifyTargets (r:5 w:0)
	/// Proof Skipped: XcmPallet VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn already_notified_target() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `243`
		//  Estimated: `12618`
		// Minimum execution time: 19_267 nanoseconds.
		Weight::from_parts(19_864_000, 0)
			.saturating_add(Weight::from_parts(0, 12618))
			.saturating_add(T::DbWeight::get().reads(5))
	}
	/// Storage: XcmPallet VersionNotifyTargets (r:2 w:1)
	/// Proof Skipped: XcmPallet VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SupportedVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	fn notify_current_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `682`
		//  Estimated: `18634`
		// Minimum execution time: 37_866 nanoseconds.
		Weight::from_parts(38_331_000, 0)
			.saturating_add(Weight::from_parts(0, 18634))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: XcmPallet VersionNotifyTargets (r:3 w:0)
	/// Proof Skipped: XcmPallet VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn notify_target_migration_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `272`
		//  Estimated: `7697`
		// Minimum execution time: 8_655 nanoseconds.
		Weight::from_parts(8_974_000, 0)
			.saturating_add(Weight::from_parts(0, 7697))
			.saturating_add(T::DbWeight::get().reads(3))
	}
	/// Storage: XcmPallet VersionNotifyTargets (r:4 w:2)
	/// Proof Skipped: XcmPallet VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	fn migrate_version_notify_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `10140`
		// Minimum execution time: 17_328 nanoseconds.
		Weight::from_parts(18_147_000, 0)
			.saturating_add(Weight::from_parts(0, 10140))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: XcmPallet VersionNotifyTargets (r:4 w:2)
	/// Proof Skipped: XcmPallet VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SupportedVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	fn migrate_and_notify_old_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `686`
		//  Estimated: `23612`
		// Minimum execution time: 44_960 nanoseconds.
		Weight::from_parts(45_541_000, 0)
			.saturating_add(Weight::from_parts(0, 23612))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(5))
	}
}