// Gotham-city
//
// Copyright 2018 by Kzen Networks (kzencorp.com)
// Gotham city is free software: you can redistribute
// it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
//

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate curv;
extern crate kms;
extern crate multi_party_ecdsa;
extern crate rocket_contrib;
extern crate rocksdb;
extern crate uuid;
extern crate zk_paillier;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate strum;

#[macro_use]
extern crate strum_macros;

#[macro_use]
extern crate log;

extern crate time_test;
extern crate time;

pub mod routes;
pub mod server;
pub mod tests;
pub mod utilities;
