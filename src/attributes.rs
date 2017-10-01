//! Tools for working with compile time Vertex Attributes.
//!
//! The only peice of this module that you will most likely need is the [`buffer_layout`] macro.
//!
//! [`buffer_layout`]: ../macro.buffer_layout.html

use gl;
use gl::types::*;

use std::marker::PhantomData;
use std::cmp::max;

pub trait AttributeTrait {
    fn declare(index: u32, offset: usize, stride: i32);
    fn stride(total: i32, max_alignment: i32) -> i32;

    /// Calculate the padding necessary from offset to reach this Attributes alignment
    /// requirements.
    fn padding(offset: usize) -> usize;
}

pub struct AttributeTail;

impl AttributeTrait for AttributeTail {
    #[inline]
    fn declare(_: u32, _: usize, _: i32) {}

    #[inline]
    fn padding(_: usize) -> usize {
        0
    }

    #[inline]
    fn stride(total: i32, max_alignment: i32) -> i32 {
        // don't forget to pad the stride if necessary
        total + (max_alignment - total % max_alignment) % max_alignment
    }
}

pub struct Attribute<T: ToGlAttrib, A: AttributeTrait>(PhantomData<(T, A)>);

impl<T: ToGlAttrib, A: AttributeTrait> Attribute<T, A> {
    /// Shadow the trait method so the user never has to interact with the Traits
    #[inline]
    pub fn declare(index: u32) {
        <Self as AttributeTrait>::declare(index, 0, Self::stride());
    }

    #[inline]
    pub fn stride() -> i32 {
        <Self as AttributeTrait>::stride(0, T::alignment() as i32)
    }
}

impl<T: ToGlAttrib, A: AttributeTrait> AttributeTrait for Attribute<T, A> {
    #[inline]
    fn declare(index: u32, mut offset: usize, stride: i32) {
        offset += Self::padding(offset);
        unsafe {
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(index, T::components(), T::gl_enum(), T::normalized(),
                                    stride, offset as *const _);
        }
        A::declare(index + 1, offset + T::size() * T::components() as usize, stride);
    }

    #[inline]
    fn stride(mut total: i32, max_alignment: i32) -> i32 {
        total += Self::padding(total as usize) as i32;
        let size = T::size() as i32 * T::components();
        A::stride(total + size, max(T::alignment() as i32, max_alignment))
    }

    #[inline]
    fn padding(offset: usize) -> usize {
        (T::alignment() - offset % T::alignment()) % T::alignment()
    }
}

pub trait ToGlAttrib {
    fn size() -> usize;
    #[inline]
    fn alignment() -> usize {
        Self::size()
    }
    fn normalized() -> GLboolean;
    fn components() -> i32;
    fn gl_enum() -> GLenum;
}

/// Marker type for normalized attributes
pub struct Normalized<T>(T);

macro_rules! expand_ToGlAttrib_impls {
    (
        @norm $yesno:ident { $t:ty => $gl_enum:expr }
    ) => {
        impl ToGlAttrib for $t {
            #[inline]
            fn size() -> usize {
                use std::mem::size_of;
                size_of::<$t>()
            }

            #[inline]
            fn normalized() -> GLboolean {
                gl::$yesno
            }

            #[inline]
            fn components() -> i32 {
                1
            }

            #[inline]
            fn gl_enum() -> GLenum {
                $gl_enum
            }
        }

        impl ToGlAttrib for [$t; 2] {
            #[inline]
            fn size() -> usize {
                <$t as ToGlAttrib>::size()
            }

            #[inline]
            fn normalized() -> GLboolean {
                gl::$yesno
            }

            #[inline]
            fn components() -> i32 {
                2
            }

            #[inline]
            fn gl_enum() -> GLenum {
                <$t as ToGlAttrib>::gl_enum()
            }
        }

        impl ToGlAttrib for [$t; 3] {
            #[inline]
            fn size() -> usize {
                <$t as ToGlAttrib>::size()
            }

            #[inline]
            fn normalized() -> GLboolean {
                gl::$yesno
            }

            #[inline]
            fn components() -> i32 {
                3
            }

            #[inline]
            fn gl_enum() -> GLenum {
                <$t as ToGlAttrib>::gl_enum()
            }
        }

        impl ToGlAttrib for [$t; 4] {
            #[inline]
            fn size() -> usize {
                <$t as ToGlAttrib>::size()
            }

            #[inline]
            fn normalized() -> GLboolean {
                gl::$yesno
            }

            #[inline]
            fn components() -> i32 {
                4
            }

            #[inline]
            fn gl_enum() -> GLenum {
                <$t as ToGlAttrib>::gl_enum()
            }
        }
    }
}

macro_rules! impl_ToGlAttrib {
    (
        $(
            $t:ty => $gl_enum:expr
        ),+
    ) => {
        $(
            expand_ToGlAttrib_impls!(@norm FALSE { $t => $gl_enum });
        )+
    }
}


macro_rules! impl_ToNormalizedGlAttrib {
    (
        $(
            $t:ty => $gl_enum:expr
        ),+
    ) => {
        $(
            expand_ToGlAttrib_impls!(@norm TRUE { Normalized<$t> => $gl_enum });
        )+
    }
}

impl_ToGlAttrib! {
    i8 => gl::BYTE,
    i16 => gl::SHORT,
    i32 => gl::INT,
    u8 => gl::UNSIGNED_BYTE,
    u16 => gl::UNSIGNED_SHORT,
    u32 => gl::UNSIGNED_INT,
    f32 => gl::FLOAT,
    f64 => gl::DOUBLE,
    bool => gl::BYTE
}

impl_ToNormalizedGlAttrib! {
    i8 => gl::BYTE,
    i16 => gl::SHORT,
    i32 => gl::INT,
    u8 => gl::UNSIGNED_BYTE,
    u16 => gl::UNSIGNED_SHORT,
    u32 => gl::UNSIGNED_INT
}

#[macro_export]
macro_rules! reverse_then_call_buffer_layout_inner {
    ([] $($reversed:tt)*) => {
        __buffer_layout_inner!([$($reversed)*])
    };
    ([$first:tt $($remaining:tt)*] $($reversed:tt)*) => {
        reverse_then_call_buffer_layout_inner!(
            [$($remaining)*] $first $($reversed)*
        )
    };
}

#[macro_export]
macro_rules! __buffer_layout_inner {
    ([] $parsed:ty) => {
        $parsed
    };
    ([$next:ty $(, $remaining:ty)*] $parsed:ty) => {
        __buffer_layout_inner!([$($remaining),*]
            $crate::attributes::Attribute<$next, $parsed>)
    };
    ([$next:ty $(, $remaining:ty)*]) => {
        __buffer_layout_inner!([$($remaining),*]
            $crate::attributes::Attribute<$next, $crate::attributes::AttributeTail>)
    }
}

/// Specify the layout of a single OpenGL buffer.
///
/// After optimization should be equivalent to calling `glEnableVertexAttribArray` and
/// `glVertexAttribPointer` manually.
///
/// To activate this layout manually you must call the "declare" method with the location of the
/// first attribute. **All following attributes are assumed to be at consecutive locations.**
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate rustic_gl;
/// # fn main() {
/// use rustic_gl::attributes::Normalized;
/// pub type StaticMeshFormat = buffer_layout!([f32; 3], [Normalized<u8>; 4]);
/// # }
/// ```
#[macro_export]
macro_rules! buffer_layout {
    ($($t:ty),*) => {
        reverse_then_call_buffer_layout_inner!([$($t),*])
    }
}

#[cfg(test)]
mod tests {
    // TODO: Verify that this mock is actually safe and reliable in the context of `cargo test`
    // Ideally we can just disable threading for these tests.

    use gl::types::*;
    use gl;

    #[derive(PartialEq, Eq)]
    struct Gl {
        attributes: [(bool, i32, GLenum, GLboolean, i32, usize); 16]
    }

    static mut GL_STATE: Gl = Gl {
        attributes: [
            (false, 0, 0 as _, gl::FALSE, 0, 0); 16
        ]
    };

    fn enable_attrib(attr: u32) {
        unsafe {
            GL_STATE.attributes[attr as usize].0 = true;
        }
    }

    fn attr_ptr(attr: u32, comps: i32, gl_ty: GLenum, norm: GLboolean, stride: i32, ptr: usize) {
        let attr = attr as usize;
        unsafe {
            GL_STATE.attributes[attr].1 = comps;
            GL_STATE.attributes[attr].2 = gl_ty;
            GL_STATE.attributes[attr].3 = norm;
            GL_STATE.attributes[attr].4 = stride;
            GL_STATE.attributes[attr].5 = ptr;
        }
    }

    fn mock_gl() {
        gl::EnableVertexAttribArray::load_with(|_| enable_attrib as *const _);
        gl::VertexAttribPointer::load_with(|_| attr_ptr as *const _);
    }

    fn setup() {
        unsafe {
            GL_STATE = Gl {
                attributes: [
                    (false, 0, 0 as _, gl::FALSE, 0, 0); 16
                ]
            };
        }
        mock_gl();
    }

    fn state() -> &'static Gl {
        unsafe {
            &GL_STATE
        }
    }

    #[test]
    fn test_mock() {
        setup();
        enable_attrib(0);
        enable_attrib(3);
        assert_eq!(state().attributes[0].0, true);
        assert_eq!(state().attributes[1].0, false);
        assert_eq!(state().attributes[2].0, false);
        assert_eq!(state().attributes[3].0, true);
    }

    #[test]
    fn attribute_stride() {
        setup();
        use std::mem::size_of;

        type Vf = buffer_layout!([f32; 3], f32);
        let stride = Vf::stride();
        assert_eq!(size_of::<f32>() * 4, stride as usize);
    }

    #[test]
    fn supports_layouts_with_padding_on_the_end() {
        setup();

        type Vf = buffer_layout!(f32, i16, i8, i16);
        let stride = Vf::stride();
        assert_eq!(4 + 2 + 1 + (1) + 2 + (2), stride as usize);
    }

    #[test]
    fn supports_a_basic_vertex_format() {
        setup();
        use super::Normalized;

        // imagine something like (vec3 pos, vec2 uv, vec4 color, vec3 normal)
        type Vf = buffer_layout!([f32; 3], [f32; 2], [Normalized<u8>; 4], [Normalized<i16>; 3]);
        let stride = Vf::stride();
        assert_eq!(3 * 4 + 2 * 4 + 1 * 4 + 3 * 2 + (2), stride as usize);

        Vf::declare(0);
        assert_eq!(state().attributes[0], (true, 3, gl::FLOAT, gl::FALSE, stride, 0),
            "got state: {:?}", state().attributes[0]);
        assert_eq!(state().attributes[1], (true, 2, gl::FLOAT, gl::FALSE, stride, 12),
            "got state: {:?}", state().attributes[1]);
        assert_eq!(state().attributes[2], (true, 4, gl::UNSIGNED_BYTE, gl::TRUE, stride, 20),
            "got state: {:?}", state().attributes[2]);
        assert_eq!(state().attributes[3], (true, 3, gl::SHORT, gl::TRUE, stride, 24),
            "got state: {:?}", state().attributes[3]);
    }
}
