/*
 * Copyright 2022-2023 kb1000
 *
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#![deny(rust_2018_idioms)]

use anyhow::Result;
use futures::try_join;
use helixlauncher_meta::{component::Hash, util::GradleSpecifier};
use reqwest::Client;
use serde::{Deserialize, Serialize};

mod forge;
mod intermediary;
mod mojang;
mod quilt;

#[tokio::main]
async fn main() -> Result<()> {
	let client = reqwest::Client::new();

	try_join!(
		mojang::fetch(&client),
		quilt::fetch(&client),
		intermediary::fetch(&client),
	)?;

	mojang::process()?;

	// forge::process()?;

	quilt::process()?;

	intermediary::process()?;

	Ok(())
}

pub(crate) async fn get_hash(client: &Client, coord: &Library) -> Result<Hash> {
	Ok(Hash::SHA256(
		client
			.get(coord.name.to_url(&coord.url) + ".sha256")
			.header("User-Agent", "helixlauncher-meta")
			.send()
			.await?
			.text()
			.await?,
	))
}

pub(crate) async fn get_size(client: &Client, coord: &Library) -> Result<u64> {
	Ok(client
		.head(coord.name.to_url(&coord.url))
		.header("User-Agent", "helixlauncher-meta")
		.send()
		.await?
		.headers()
		.get("content-length")
		.expect("Cannot handle servers returning no content length")
		.to_str()?
		.parse()?)
}

#[derive(Serialize, Deserialize, Debug)]
struct Library {
	name: GradleSpecifier,
	url: String,
}
