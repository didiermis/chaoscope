// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of chaoscope.
//
// chaoscope is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// chaoscope is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the Affero GNU General Public License
// along with chaoscope.  If not, see <http://www.gnu.org/licenses/>.

// chaoscope RPC calls

use env_logger;
use subxt::ClientBuilder;

#[subxt::subxt(runtime_metadata_path = "metadata/substrate-node-chaos.scale")]
pub mod chaosrpc {}

pub async fn rpc_drag_block_unit_weight(n: u32) -> Result<u32, Box<dyn std::error::Error>> {
    let api = ClientBuilder::new()
        .set_url("wss://localhost:9944")
        .build()
        .await?
        .to_runtime_api::<chaosrpc::RuntimeApi<chaosrpc::DefaultConfig>>();

    let ret = 1;
    Ok(ret)
}