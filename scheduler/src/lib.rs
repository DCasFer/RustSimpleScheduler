#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(naked_functions)]

#![no_std]

#![no_builtins]

#![deny(
    unused_import_braces,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    )]

#![recursion_limit = "300"]

extern crate embrs;

pub mod leds;

pub mod init;

pub mod scheduler_core;
