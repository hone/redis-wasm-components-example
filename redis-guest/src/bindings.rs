// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod wasmredis {
    #[allow(dead_code)]
    pub mod host {
        #[allow(dead_code, clippy::all)]
        pub mod store {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Clone)]
            pub enum Error {
                NoSuchStore,
                AccessDenied,
                Other(_rt::String),
            }
            impl ::core::fmt::Debug for Error {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        Error::NoSuchStore => f.debug_tuple("Error::NoSuchStore").finish(),
                        Error::AccessDenied => f.debug_tuple("Error::AccessDenied").finish(),
                        Error::Other(e) => f.debug_tuple("Error::Other").field(e).finish(),
                    }
                }
            }
            impl ::core::fmt::Display for Error {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }

            impl std::error::Error for Error {}
            #[allow(unused_unsafe, clippy::all)]
            pub fn get(key: &str) -> Result<Option<_rt::Vec<u8>>, Error> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let vec0 = key;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasmredis:host/store")]
                    extern "C" {
                        #[link_name = "get"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0, ptr1);
                    let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = i32::from(*ptr1.add(4).cast::<u8>());

                                match l3 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let l4 = *ptr1.add(8).cast::<*mut u8>();
                                            let l5 = *ptr1.add(12).cast::<usize>();
                                            let len6 = l5;

                                            _rt::Vec::from_raw_parts(l4.cast(), len6, len6)
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                }
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l7 = i32::from(*ptr1.add(4).cast::<u8>());
                                let v11 = match l7 {
                                    0 => Error::NoSuchStore,
                                    1 => Error::AccessDenied,
                                    n => {
                                        debug_assert_eq!(n, 2, "invalid enum discriminant");
                                        let e11 = {
                                            let l8 = *ptr1.add(8).cast::<*mut u8>();
                                            let l9 = *ptr1.add(12).cast::<usize>();
                                            let len10 = l9;
                                            let bytes10 =
                                                _rt::Vec::from_raw_parts(l8.cast(), len10, len10);

                                            _rt::string_lift(bytes10)
                                        };
                                        Error::Other(e11)
                                    }
                                };

                                v11
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn set(key: &str, value: &[u8]) -> Result<(), Error> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let vec0 = key;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let vec1 = value;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasmredis:host/store")]
                    extern "C" {
                        #[link_name = "set"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize, _: *mut u8);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0, ptr1.cast_mut(), len1, ptr2);
                    let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                    match l3 {
                        0 => {
                            let e = ();
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l4 = i32::from(*ptr2.add(4).cast::<u8>());
                                let v8 = match l4 {
                                    0 => Error::NoSuchStore,
                                    1 => Error::AccessDenied,
                                    n => {
                                        debug_assert_eq!(n, 2, "invalid enum discriminant");
                                        let e8 = {
                                            let l5 = *ptr2.add(8).cast::<*mut u8>();
                                            let l6 = *ptr2.add(12).cast::<usize>();
                                            let len7 = l6;
                                            let bytes7 =
                                                _rt::Vec::from_raw_parts(l5.cast(), len7, len7);

                                            _rt::string_lift(bytes7)
                                        };
                                        Error::Other(e8)
                                    }
                                };

                                v8
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn delete(key: &str) -> Result<_rt::Vec<u8>, Error> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let vec0 = key;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasmredis:host/store")]
                    extern "C" {
                        #[link_name = "delete"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0, ptr1);
                    let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = *ptr1.add(4).cast::<*mut u8>();
                                let l4 = *ptr1.add(8).cast::<usize>();
                                let len5 = l4;

                                _rt::Vec::from_raw_parts(l3.cast(), len5, len5)
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l6 = i32::from(*ptr1.add(4).cast::<u8>());
                                let v10 = match l6 {
                                    0 => Error::NoSuchStore,
                                    1 => Error::AccessDenied,
                                    n => {
                                        debug_assert_eq!(n, 2, "invalid enum discriminant");
                                        let e10 = {
                                            let l7 = *ptr1.add(8).cast::<*mut u8>();
                                            let l8 = *ptr1.add(12).cast::<usize>();
                                            let len9 = l8;
                                            let bytes9 =
                                                _rt::Vec::from_raw_parts(l7.cast(), len9, len9);

                                            _rt::string_lift(bytes9)
                                        };
                                        Error::Other(e10)
                                    }
                                };

                                v10
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod wasmredis {
        #[allow(dead_code)]
        pub mod host {
            #[allow(dead_code, clippy::all)]
            pub mod run {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_run_cabi<T: Guest>() {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    T::run();
                }
                pub trait Guest {
                    fn run();
                }
                #[doc(hidden)]

                macro_rules! __export_wasmredis_host_run_cabi{
        ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

          #[export_name = "wasmredis:host/run#run"]
          unsafe extern "C" fn export_run() {
            $($path_to_types)*::_export_run_cabi::<$ty>()
          }
        };);
      }
                #[doc(hidden)]
                pub(crate) use __export_wasmredis_host_run_cabi;
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_example_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::wasmredis::host::run::__export_wasmredis_host_run_cabi!($ty with_types_in $($path_to_types_root)*::exports::wasmredis::host::run);
  )
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 369] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xf3\x01\x01A\x02\x01\
A\x04\x01B\x0d\x01q\x03\x0dno-such-store\0\0\x0daccess-denied\0\0\x05other\x01s\0\
\x04\0\x05error\x03\0\0\x01p}\x01k\x02\x01j\x01\x03\x01\x01\x01@\x01\x03keys\0\x04\
\x04\0\x03get\x01\x05\x01j\0\x01\x01\x01@\x02\x03keys\x05value\x02\0\x06\x04\0\x03\
set\x01\x07\x01j\x01\x02\x01\x01\x01@\x01\x03keys\0\x08\x04\0\x06delete\x01\x09\x03\
\x01\x14wasmredis:host/store\x05\0\x01B\x02\x01@\0\x01\0\x04\0\x03run\x01\0\x04\x01\
\x12wasmredis:host/run\x05\x01\x04\x01\x17wasmredis:guest/example\x04\0\x0b\x0d\x01\
\0\x07example\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x07\
0.208.1\x10wit-bindgen-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
