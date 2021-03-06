//! ArrayFire is a high performance software library for parallel computing with
//! an easy-to-use API. ArrayFire abstracts away much of the details of
//! programming parallel architectures by providing a high-level container object,
//! the [Array](./struct.Array.html), that represents data stored on a CPU, GPU, FPGA,
//! or other type of accelerator. This abstraction permits developers to write
//! massively parallel applications in a high-level language where they need
//! not be concerned about low-level optimizations that are frequently required to
//! achieve high throughput on most parallel architectures.

//! This crate provides Rust bindings for the ArrayFire library. Given below table shows
//! the rust bindings compatability with ArrayFire upstream.  If you find any bugs,
//! please report them on [github](https://github.com/arrayfire/arrayfire-rust/issues).
//!
//! | arrayfire-rust crate | ArrayFire Upstream |
//! |:--------------------:|:------------------:|
//! |         M.m.p1       |      M.m.p2        |
//!
//! Only, Major(M) & Minor(m) version numbers need to match. *p1* and *p2*
//! are patch/fix updates for `arrayfire-rust` & `ArrayFire` respectively,
//! and they don't need to match.
//!
//! Please go through our [tutorials](./book/index.html) book for more explanations on how to
//! use ArrayFire to speedup your code.

#![doc(
    html_logo_url = "http://www.arrayfire.com/logos/arrayfire_logo_symbol.png",
    html_favicon_url = "http://www.rust-lang.org/favicon.ico",
    html_root_url = "http://arrayfire.org/arrayfire-rust/arrayfire/index.html"
)]
#![warn(missing_docs)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate lazy_static;

pub use crate::array::*;
mod array;

#[cfg(feature = "algorithm")]
pub use crate::algorithm::*;
#[cfg(feature = "algorithm")]
mod algorithm;

#[cfg(feature = "arithmetic")]
pub use crate::arith::*;
#[cfg(feature = "arithmetic")]
mod arith;

pub use crate::backend::*;
mod backend;

#[cfg(feature = "blas")]
pub use crate::blas::*;
#[cfg(feature = "blas")]
mod blas;

#[cfg(feature = "data")]
pub use crate::data::*;
#[cfg(feature = "data")]
mod data;

pub use crate::device::*;
mod device;

pub use crate::defines::*;
mod defines;

pub use crate::dim4::Dim4;
mod dim4;

pub use crate::error::{handle_error_general, register_error_handler, Callback, ErrorCallback};
mod error;

#[cfg(feature = "indexing")]
pub use crate::index::*;
#[cfg(feature = "indexing")]
mod index;

#[cfg(feature = "indexing")]
pub use crate::seq::Seq;
#[cfg(feature = "indexing")]
mod seq;

#[cfg(feature = "graphics")]
pub use crate::graphics::Window;
#[cfg(feature = "graphics")]
mod graphics;

#[cfg(feature = "image")]
pub use crate::image::*;
#[cfg(feature = "image")]
mod image;

#[cfg(feature = "lapack")]
pub use crate::lapack::*;
#[cfg(feature = "lapack")]
mod lapack;

#[cfg(feature = "macros")]
mod macros;
mod num;

#[cfg(feature = "random")]
pub use crate::random::*;
#[cfg(feature = "random")]
mod random;

#[cfg(feature = "signal")]
pub use crate::signal::*;
#[cfg(feature = "signal")]
mod signal;

#[cfg(feature = "sparse")]
pub use crate::sparse::*;
#[cfg(feature = "sparse")]
mod sparse;

#[cfg(feature = "statistics")]
pub use crate::statistics::*;
#[cfg(feature = "statistics")]
mod statistics;

pub use crate::util::{get_size, HasAfEnum, ImplicitPromote};
pub use crate::util::{ComplexFloating, FloatingPoint, RealFloating, RealNumber};
pub use crate::util::{CovarianceComputable, EdgeComputable, MedianComputable, MomentsComputable};
pub use crate::util::{GrayRGBConvertible, ImageFilterType, ImageNativeType, Scanable};
mod util;

#[cfg(feature = "vision")]
pub use crate::vision::*;
#[cfg(feature = "vision")]
mod vision;

// headers that are not exposed through rust wrapper are given follows:
// compatible.h
// constants.h
// complex.h
// cuda.h
// opencl.h
