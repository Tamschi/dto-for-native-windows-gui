//! TODO_DOCS_DESCRIPTION
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Fdto-for-native-windows-gui)](https://iteration-square.schichler.dev/#narrow/stream/project.2Fdto-for-native-windows-gui)

#![doc(html_root_url = "https://docs.rs/dto-for-native-windows-gui/0.0.1")]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::semicolon_if_nothing_returned, missing_docs)]

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, ops::Range};

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ProgressBarStateDto {
	Normal,
	Error,
	Paused,
}

#[non_exhaustive]
#[derive(Debug, Default, Clone, Builder, Serialize, Deserialize)]
#[builder(setter(into, strip_option))]
pub struct ProgressBarDto {
	pub x: Option<u32>,
	pub y: Option<u32>,

	pub width: Option<u32>,
	pub height: Option<u32>,

	pub visible: Option<bool>,

	pub vertical: Option<bool>,

	pub state: Option<ProgressBarStateDto>,

	pub range: Option<Range<u32>>,
	pub step: Option<u32>,
	pub pos: Option<u32>,

	pub enabled: Option<bool>,

	pub marquee: Option<(bool, u32)>,
}

#[non_exhaustive]
#[derive(Debug, Default, Clone, Builder, Serialize, Deserialize)]
#[builder(setter(into, strip_option))]
pub struct FontDto<'a> {
	pub family: Option<Cow<'a, str>>,

	pub size: Option<u32>,
	pub size_absolute: Option<u32>,

	pub weight: Option<u32>,
}

#[non_exhaustive]
#[derive(Debug, Default, Clone, Builder, Serialize, Deserialize)]
#[builder(setter(into, strip_option))]
pub struct ButtonDto<'a> {
	pub x: Option<u32>,
	pub y: Option<u32>,

	pub width: Option<u32>,
	pub height: Option<u32>,

	pub visible: Option<bool>,

	pub font: Option<FontDto<'a>>,

	pub enabled: Option<bool>,

	//TODO
}
