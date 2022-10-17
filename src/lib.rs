// Implementation of DKIM: https://datatracker.ietf.org/doc/html/rfc6376

#[macro_use]
extern crate quick_error;

mod bytes;
pub mod canonicalization;
pub mod dkim_sign;
pub mod dkim_verify;
pub mod dns;
mod errors;
mod hash;
mod header;
mod parser;
mod public_key;
mod result;
#[cfg(test)]
mod roundtrip_test;

pub use errors::DKIMError;
pub use result::DKIMResult;
