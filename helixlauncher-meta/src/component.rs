/*
 * Copyright 2022-2023 kb1000
 *
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{collections::BTreeSet, fmt::Display};

use crate::util::GradleSpecifier;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, OneOrMany};

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OsName {
	Linux,
	Osx,
	Windows,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ComponentDependency {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Hash {
	SHA256(String),
	SHA1(String),
}

impl Display for Hash {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Hash::SHA1(hash) => write!(f, "SHA1 hash {hash}"),
			Hash::SHA256(hash) => write!(f, "SHA256 hash {hash}"),
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Download {
	pub name: GradleSpecifier,
	pub url: String,
	// these two might have to be made optional
	pub size: u32,
	pub hash: Hash,
}

/// A trait of a component or instance.
///
/// The [Ord] and [PartialOrd] impls are for use with [BTreeSet].
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trait {
	/// This component needs -XstartOnFirstThread on macOS.
	MacStartOnFirstThread,
	SupportsCustomResolution,
	SupportsQuickPlayServerLegacy,
	SupportsQuickPlayServer,
	SupportsQuickPlayWorld,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Arch {
	X86,
	X86_64,
	Arm64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Platform {
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	#[serde_as(as = "OneOrMany<_>")]
	pub os: Vec<OsName>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arch: Option<Arch>,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Native {
	pub name: GradleSpecifier,
	pub platform: Platform,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub exclusions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum ConditionalClasspathEntry {
	All(GradleSpecifier),
	PlatformSpecific {
		name: GradleSpecifier,
		platform: Platform,
	},
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assets {
	pub id: String,
	pub url: String,
	pub sha1: String,
	pub size: u32,
	pub total_size: u32, // TODO: is this really necessary?
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConditionFeature {
	Demo,
	Fullscreen,
	CustomResolution,
	QuickPlayServerLegacy,
	QuickPlayServer,
	QuickPlayWorld,
}

// TODO: this feels a bit hacky?
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MinecraftArgument {
	Always(String),
	Conditional {
		value: String,
		feature: ConditionFeature,
	},
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Component {
	pub format_version: u32,
	pub id: String,
	pub version: String,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub requires: Vec<ComponentDependency>,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub conflicts: Vec<ComponentDependency>,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub before: Vec<ComponentDependency>,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub after: Vec<ComponentDependency>,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub provides: Vec<ComponentDependency>,
	#[serde(skip_serializing_if = "BTreeSet::is_empty", default)]
	pub traits: BTreeSet<Trait>,
	pub assets: Option<Assets>,
	pub downloads: Vec<Download>,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub jarmods: Vec<GradleSpecifier>,
	pub game_jar: Option<GradleSpecifier>, // separate from classpath to make injecting jarmods possible
	pub main_class: Option<String>,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub game_arguments: Vec<MinecraftArgument>,
	pub classpath: Vec<ConditionalClasspathEntry>,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub natives: Vec<Native>,
	pub release_time: DateTime<Utc>,
}
