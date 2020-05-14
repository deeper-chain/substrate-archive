// Copyright 2017-2019 Parity Technologies (UK) Ltd.
// This file is part of substrate-archive.

// substrate-archive is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// substrate-archive is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with substrate-archive.  If not, see <http://www.gnu.org/licenses/>.

use subxt::{balances::Balances, system::System};

/// Consolidation of substrate traits representing fundamental types
pub trait Substrate: System + Balances + Send + Sync {}

impl<T> Substrate for T where T: System + Balances + Send + Sync {}

pub trait ChainInfo<T: Substrate> {
    fn get_hash(&self) -> T::Hash;
}
