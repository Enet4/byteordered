# byteorder-runtime

[![Build Status](https://travis-ci.org/Enet4/byteorder-runtime.svg?branch=master)](https://travis-ci.org/Enet4/byteorder-runtime)

A library for reading and writing data in a byte order only known at run-time.

## Why yet another data parsing crate

While `byteorder` is well established in the Rust ecosystem, it relies on zero-constructor types for declaring the intended byte order. As such, it lacks a construct for reading and writing data in an endianness not known at compile time.

Rather than building yet another new library, this crates aims to extend `byteorder` with new types and facilities for this particular case, preserving most of its look-and-feel.

It is currently a work in progress: the API may have sudden changes at this phase.
