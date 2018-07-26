//! This library provides a number of utilities for working with OpenGL directly.
//!
//! This library expects you to already be comfortable with OpenGL. This library provides the very
//! basic functions that you would have to write anyways when starting a new project.
//!
//! As your project grows, when a function becomes insufficient for your use case, you can copy
//! the code from here and then modify it as necessary *for your project.* If your code becomes
//! useful accross many OpenGL apps, feel free to open a pull request.
//!
//! The `raw` module contains basic functions like `create_buffer()` and `create_vao()`, and the
//! `error` module provides a very basic, boilerplate `GlError` type.
//!
//! A "goody" included in the library is the `create_basic_program(vertex_src, shader_src)`
//! function, which may save you a few minutes when making a new project or small toy app.
//!
//! A good example of when you may want to just copy code from this library, instead of using it
//! directly, is when you are writing a shader program generator and decide on an opinionated
//! framework using the GLSL preprocessor. You could refer to the source of `create_linked_program`
//! as a starting point.

pub extern crate gl;

#[macro_use]
pub mod attributes;

pub mod error;
pub mod raw;
