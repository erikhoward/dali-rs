// Copyright 2022 Erik Howard <erikhoward@pm.me>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.


//! # dali
//! 
//!  `daili` is a database client library for ![SurrealDB](https://surrealdb.com/)
//! 
#[macro_use]
extern crate derive_builder;

extern crate serde;
extern crate serde_json;

pub mod client;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[inline(always)]
pub fn version() -> &'static str {
    VERSION
}