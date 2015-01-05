#![feature(slicing_syntax)]
// TODO: This is required to derive RustcEncodable/RustcDecodable. I'm not sure
//       what's up there, but I do know that this feature gate will be removed
//       soon, so some other fix is required.
#![feature(old_orphan_check)]


extern crate acpe;
extern crate "rustc-serialize" as rustc_serialize;


pub mod protocol;
