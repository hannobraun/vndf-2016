// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use stb_image::bindgen::*;

use std::libc;
use std::libc::{c_void, c_int};
use std::vec::raw::from_buf_raw;

pub struct Image<T> {
    width   : uint,
    height  : uint,
    depth   : uint,
    data    : ~[T],
}

pub fn new_image<T>(width: uint, height: uint, depth: uint, data: ~[T]) -> Image<T> {
    Image::<T> {
        width   : width,
        height  : height,
        depth   : depth,
        data    : data,
    }
}

pub enum LoadResult {
    Error(~str),
    ImageU8(Image<u8>),
    ImageF32(Image<f32>),
}

impl LoadResult {
    pub fn from_result(res: Result<LoadResult,~Any>)-> LoadResult {
        match res {
            Ok(res) => res,
            Err(e)  => Error(e.to_str()),
        }
    }
}

pub fn load(path: ~str) -> LoadResult {
    let force_depth = 0;
    load_with_depth(path, force_depth, false)
}


fn load_internal<T>(buf : *T, w : c_int, h : c_int, d : c_int) -> Image<T> {
    unsafe {
        // FIXME: Shouldn't copy; instead we should use a sendable resource. They
        // aren't particularly safe yet though.
        let data = from_buf_raw(buf, (w * h * d) as uint);
        libc::free(buf as *mut c_void);
        Image::<T>{
            width   : w as uint,
            height  : h as uint,
            depth   : d as uint,
            data    : data}
    }
}

pub fn load_with_depth(path: ~str, force_depth: uint, convert_hdr:bool) -> LoadResult {
    unsafe {
        let mut width = 0 as c_int;
        let mut height = 0 as c_int;
        let mut depth = 0 as c_int;
        path.to_c_str().with_ref(|bytes| {
            if !convert_hdr && stbi_is_hdr(bytes)!=0   {
                let buffer = stbi_loadf(bytes,
                                        &mut width,
                                        &mut height,
                                        &mut depth,
                                        force_depth as c_int);
                if buffer.is_null() {
                    Error(~"stbi_loadf failed")
                } else {
                    ImageF32(load_internal(buffer, width, height, depth))
                }
            } else {
                let buffer = stbi_load(bytes,
                                       &mut width,
                                       &mut height,
                                       &mut depth,
                                       force_depth as c_int);
                if buffer.is_null() {
                    Error(~"stbi_load failed")
                } else {
                    ImageU8(load_internal(buffer, width, height, depth))
                }
            }
        })
    }
}

pub fn load_from_memory(buffer: &[u8]) -> LoadResult {
    let force_depth = 0;
    load_from_memory_with_depth(buffer, force_depth, false)
}

pub fn load_from_memory_with_depth(buffer: &[u8], force_depth: uint, convert_hdr:bool) -> LoadResult {
    unsafe {
        let mut width = 0 as c_int;
        let mut height = 0 as c_int;
        let mut depth = 0 as c_int;
        if !convert_hdr && stbi_is_hdr_from_memory(buffer.as_ptr(), buffer.len() as c_int) != 0 {
            let buffer = stbi_loadf_from_memory(buffer.as_ptr(),
                                                buffer.len() as c_int,
                                                &mut width,
                                                &mut height,
                                                &mut depth,
                                                force_depth as c_int);
            if buffer.is_null() {
                Error(~"stbi_loadf_from_memory failed")
            } else {
                let actual_depth = if force_depth != 0 { force_depth as c_int } else { depth };
                ImageF32(load_internal(buffer, width, height, actual_depth))
            }
        } else {
            let buffer = stbi_load_from_memory(buffer.as_ptr(),
                                               buffer.len() as c_int,
                                               &mut width,
                                               &mut height,
                                               &mut depth,
                                               force_depth as c_int);
            if buffer.is_null() {
                Error(~"stbi_load_from_memory failed")
            } else {
                let actual_depth = if force_depth != 0 { force_depth as c_int } else { depth };
                ImageU8(load_internal(buffer, width, height, actual_depth))
            }
        }
    }
}
