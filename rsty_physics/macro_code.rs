#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use godot::engine::utilities::sin;
use godot::prelude::*;
struct RustyPhysics;
mod trig {
    use std::borrow::Borrow;
    use godot::engine::node::InternalMode;
    use godot::engine::packed_scene::GenEditState;
    use godot::engine::utilities::sin;
    use godot::engine::{Line2D, Marker2D, PathFollow2D, RigidBody2D, Timer};
    use godot::builtin::Vector2;
    use godot::prelude::*;
    use godot::private::You_forgot_the_attribute__godot_api;
    use std::f64::consts::PI;
    use godot::builtin::VariantType::PackedVector2Array;
    #[class(base = Line2D)]
    pub struct SineWave2D {
        score: i64,
        #[base]
        base: Base<Line2D>,
    }
    impl ::godot::obj::GodotClass for SineWave2D {
        type Base = ::godot::engine::Line2D;
        type Declarer = ::godot::obj::dom::UserDomain;
        type Mem = <Self::Base as ::godot::obj::GodotClass>::Mem;
        const CLASS_NAME: &'static str = "SineWave2D";
    }
    impl std::ops::Deref for SineWave2D {
        type Target = <Self as ::godot::obj::GodotClass>::Base;
        fn deref(&self) -> &Self::Target {
            &*self.base
        }
    }
    impl std::ops::DerefMut for SineWave2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.base
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        #[used]
        #[link_section = ".init_array"]
        static __init: extern "C" fn() = {
            #[link_section = ".text.startup"]
            extern "C" fn __inner_init() {
                let mut guard = ::godot::private::__godot_rust_plugin___GODOT_PLUGIN_REGISTRY
                    .lock()
                    .unwrap();
                guard
                    .push(::godot::private::ClassPlugin {
                        class_name: "SineWave2D",
                        component: ::godot::private::PluginComponent::ClassDef {
                            base_class_name: "Line2D",
                            generated_create_fn: None,
                            free_fn: ::godot::private::callbacks::free::<SineWave2D>,
                        },
                    });
            }
            __inner_init
        };
    };
    impl ::godot::obj::Inherits<::godot::engine::Line2D> for SineWave2D {}
    impl ::godot::obj::Inherits<::godot::engine::Node2D> for SineWave2D {}
    impl ::godot::obj::Inherits<::godot::engine::CanvasItem> for SineWave2D {}
    impl ::godot::obj::Inherits<::godot::engine::Node> for SineWave2D {}
    impl ::godot::obj::Inherits<::godot::engine::Object> for SineWave2D {}
    impl SineWave2D {
        fn game_over(&mut self) {
            {
                ::std::io::_print(
                    ::core::fmt::Arguments::new_v1(&["Game over!\n"], &[]),
                );
            };
        }
        pub fn new_game(&mut self) {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["New game\n"], &[]));
            };
        }
        fn test_sin(&mut self, angle: f64, answer: f64) {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1_formatted(
                            &[""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &(sin(angle.to_radians()) * 10.0),
                                ),
                            ],
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Is(1usize),
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                            unsafe { ::core::fmt::UnsafeArg::new() },
                        ),
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1_formatted(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&(answer * 10.0))],
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Is(1usize),
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                            unsafe { ::core::fmt::UnsafeArg::new() },
                        ),
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
        fn full_circle_sin(&mut self) {
            self.test_sin(30.0, 0.50);
            self.test_sin(60.0, 0.87);
            self.test_sin(90.0, 1.0);
            self.test_sin(120.0, 0.87);
            self.test_sin(150.0, 0.5);
            self.test_sin(180.0, f64::EPSILON);
            self.test_sin(210.0, -0.50);
            self.test_sin(240.0, -0.87);
            self.test_sin(270.0, -1.0);
            self.test_sin(300.0, -0.87);
            self.test_sin(330.0, -0.5);
            self.test_sin(360.0, -f64::EPSILON);
        }
        fn draw_wave(&mut self) {}
    }
    impl ::godot::obj::cap::ImplementsGodotApi for SineWave2D {
        fn __register_methods() {
            unsafe {
                use ::godot_core::sys;
                use ::godot_core::builtin::{Variant, StringName};
                use ::godot_core::builtin::meta::*;
                const NUM_ARGS: usize = 0;
                type Sig = ((),);
                let varcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeVariantPtr,
                        _arg_count: sys::GDNativeInt,
                        ret: sys::GDNativeVariantPtr,
                        err: *mut sys::GDNativeCallError,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::varcall::<
                                SineWave2D,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                err,
                                |inst, params| {
                                    let () = params;
                                    inst.game_over()
                                },
                                "game_over",
                            )
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"game_over")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    26u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                            (*err).error = sys::GDNATIVE_CALL_ERROR_INVALID_METHOD;
                            {
                                unsafe {
                                    ::godot_ffi::get_interface()
                                        .variant_new_nil
                                        .unwrap_unchecked()
                                }
                            }
                            (ret);
                        }
                    }
                    function
                };
                let ptrcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeTypePtr,
                        ret: sys::GDNativeTypePtr,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::ptrcall::<
                                SineWave2D,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                |inst, params| {
                                    let () = params;
                                    inst.game_over()
                                },
                                "game_over",
                            );
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"game_over")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    26u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                        }
                    }
                    function
                };
                let has_return_value: bool = true;
                let return_value_info = Sig::property_info(-1, "");
                let mut return_value_info_sys = return_value_info.property_sys();
                let return_value_metadata = Sig::param_metadata(-1);
                let argument_count = NUM_ARGS as u32;
                let mut arguments_info: [PropertyInfo; NUM_ARGS] = {
                    let mut i = -1i32;
                    []
                };
                let mut arguments_info_sys: [sys::GDNativePropertyInfo; NUM_ARGS] = std::array::from_fn(|
                    i|
                arguments_info[i].property_sys());
                let mut arguments_metadata: [sys::GDNativeExtensionClassMethodArgumentMetadata; NUM_ARGS] = std::array::from_fn(|
                    i|
                Sig::param_metadata(i as i32));
                let class_name = StringName::from("SineWave2D");
                let method_name = StringName::from("game_over");
                let method_info = sys::GDNativeExtensionClassMethodInfo {
                    name: method_name.string_sys(),
                    method_userdata: std::ptr::null_mut(),
                    call_func: Some(varcall_func),
                    ptrcall_func: Some(ptrcall_func),
                    method_flags: sys::GDNATIVE_EXTENSION_METHOD_FLAGS_DEFAULT as u32,
                    has_return_value: has_return_value as u8,
                    return_value_info: &raw mut return_value_info_sys,
                    return_value_metadata,
                    argument_count,
                    arguments_info: arguments_info_sys.as_mut_ptr(),
                    arguments_metadata: arguments_metadata.as_mut_ptr(),
                    default_argument_count: 0,
                    default_arguments: std::ptr::null_mut(),
                };
                {
                    use std::io::{sink, Write};
                    let _ = sink()
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["   Register fn:   ", "::"],
                                &match (&"SineWave2D", &"game_over") {
                                    args => {
                                        [
                                            ::core::fmt::ArgumentV1::new_display(args.0),
                                            ::core::fmt::ArgumentV1::new_display(args.1),
                                        ]
                                    }
                                },
                            ),
                        );
                };
                {
                    unsafe {
                        ::godot_ffi::get_interface()
                            .classdb_register_extension_class_method
                            .unwrap_unchecked()
                    }
                }
                (sys::get_library(), class_name.string_sys(), &raw const method_info);
            };
            unsafe {
                use ::godot_core::sys;
                use ::godot_core::builtin::{Variant, StringName};
                use ::godot_core::builtin::meta::*;
                const NUM_ARGS: usize = 0;
                type Sig = ((),);
                let varcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeVariantPtr,
                        _arg_count: sys::GDNativeInt,
                        ret: sys::GDNativeVariantPtr,
                        err: *mut sys::GDNativeCallError,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::varcall::<
                                SineWave2D,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                err,
                                |inst, params| {
                                    let () = params;
                                    inst.new_game()
                                },
                                "new_game",
                            )
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"new_game")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    26u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                            (*err).error = sys::GDNATIVE_CALL_ERROR_INVALID_METHOD;
                            {
                                unsafe {
                                    ::godot_ffi::get_interface()
                                        .variant_new_nil
                                        .unwrap_unchecked()
                                }
                            }
                            (ret);
                        }
                    }
                    function
                };
                let ptrcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeTypePtr,
                        ret: sys::GDNativeTypePtr,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::ptrcall::<
                                SineWave2D,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                |inst, params| {
                                    let () = params;
                                    inst.new_game()
                                },
                                "new_game",
                            );
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"new_game")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    26u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                        }
                    }
                    function
                };
                let has_return_value: bool = true;
                let return_value_info = Sig::property_info(-1, "");
                let mut return_value_info_sys = return_value_info.property_sys();
                let return_value_metadata = Sig::param_metadata(-1);
                let argument_count = NUM_ARGS as u32;
                let mut arguments_info: [PropertyInfo; NUM_ARGS] = {
                    let mut i = -1i32;
                    []
                };
                let mut arguments_info_sys: [sys::GDNativePropertyInfo; NUM_ARGS] = std::array::from_fn(|
                    i|
                arguments_info[i].property_sys());
                let mut arguments_metadata: [sys::GDNativeExtensionClassMethodArgumentMetadata; NUM_ARGS] = std::array::from_fn(|
                    i|
                Sig::param_metadata(i as i32));
                let class_name = StringName::from("SineWave2D");
                let method_name = StringName::from("new_game");
                let method_info = sys::GDNativeExtensionClassMethodInfo {
                    name: method_name.string_sys(),
                    method_userdata: std::ptr::null_mut(),
                    call_func: Some(varcall_func),
                    ptrcall_func: Some(ptrcall_func),
                    method_flags: sys::GDNATIVE_EXTENSION_METHOD_FLAGS_DEFAULT as u32,
                    has_return_value: has_return_value as u8,
                    return_value_info: &raw mut return_value_info_sys,
                    return_value_metadata,
                    argument_count,
                    arguments_info: arguments_info_sys.as_mut_ptr(),
                    arguments_metadata: arguments_metadata.as_mut_ptr(),
                    default_argument_count: 0,
                    default_arguments: std::ptr::null_mut(),
                };
                {
                    use std::io::{sink, Write};
                    let _ = sink()
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["   Register fn:   ", "::"],
                                &match (&"SineWave2D", &"new_game") {
                                    args => {
                                        [
                                            ::core::fmt::ArgumentV1::new_display(args.0),
                                            ::core::fmt::ArgumentV1::new_display(args.1),
                                        ]
                                    }
                                },
                            ),
                        );
                };
                {
                    unsafe {
                        ::godot_ffi::get_interface()
                            .classdb_register_extension_class_method
                            .unwrap_unchecked()
                    }
                }
                (sys::get_library(), class_name.string_sys(), &raw const method_info);
            };
            unsafe {
                use ::godot_core::sys;
                use ::godot_core::builtin::{Variant, StringName};
                use ::godot_core::builtin::meta::*;
                const NUM_ARGS: usize = 1 + (1 + 0);
                type Sig = ((), f64, f64);
                let varcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeVariantPtr,
                        _arg_count: sys::GDNativeInt,
                        ret: sys::GDNativeVariantPtr,
                        err: *mut sys::GDNativeCallError,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::varcall::<
                                SineWave2D,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                err,
                                |inst, params| {
                                    let (angle, answer) = params;
                                    inst.test_sin(angle, answer)
                                },
                                "test_sin",
                            )
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"test_sin")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    26u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                            (*err).error = sys::GDNATIVE_CALL_ERROR_INVALID_METHOD;
                            {
                                unsafe {
                                    ::godot_ffi::get_interface()
                                        .variant_new_nil
                                        .unwrap_unchecked()
                                }
                            }
                            (ret);
                        }
                    }
                    function
                };
                let ptrcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeTypePtr,
                        ret: sys::GDNativeTypePtr,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::ptrcall::<
                                SineWave2D,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                |inst, params| {
                                    let (angle, answer) = params;
                                    inst.test_sin(angle, answer)
                                },
                                "test_sin",
                            );
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"test_sin")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    26u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                        }
                    }
                    function
                };
                let has_return_value: bool = true;
                let return_value_info = Sig::property_info(-1, "");
                let mut return_value_info_sys = return_value_info.property_sys();
                let return_value_metadata = Sig::param_metadata(-1);
                let argument_count = NUM_ARGS as u32;
                let mut arguments_info: [PropertyInfo; NUM_ARGS] = {
                    let mut i = -1i32;
                    [
                        {
                            i += 1;
                            let prop = Sig::property_info(i, "angle");
                            prop
                        },
                        {
                            i += 1;
                            let prop = Sig::property_info(i, "answer");
                            prop
                        },
                    ]
                };
                let mut arguments_info_sys: [sys::GDNativePropertyInfo; NUM_ARGS] = std::array::from_fn(|
                    i|
                arguments_info[i].property_sys());
                let mut arguments_metadata: [sys::GDNativeExtensionClassMethodArgumentMetadata; NUM_ARGS] = std::array::from_fn(|
                    i|
                Sig::param_metadata(i as i32));
                let class_name = StringName::from("SineWave2D");
                let method_name = StringName::from("test_sin");
                let method_info = sys::GDNativeExtensionClassMethodInfo {
                    name: method_name.string_sys(),
                    method_userdata: std::ptr::null_mut(),
                    call_func: Some(varcall_func),
                    ptrcall_func: Some(ptrcall_func),
                    method_flags: sys::GDNATIVE_EXTENSION_METHOD_FLAGS_DEFAULT as u32,
                    has_return_value: has_return_value as u8,
                    return_value_info: &raw mut return_value_info_sys,
                    return_value_metadata,
                    argument_count,
                    arguments_info: arguments_info_sys.as_mut_ptr(),
                    arguments_metadata: arguments_metadata.as_mut_ptr(),
                    default_argument_count: 0,
                    default_arguments: std::ptr::null_mut(),
                };
                {
                    use std::io::{sink, Write};
                    let _ = sink()
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["   Register fn:   ", "::"],
                                &match (&"SineWave2D", &"test_sin") {
                                    args => {
                                        [
                                            ::core::fmt::ArgumentV1::new_display(args.0),
                                            ::core::fmt::ArgumentV1::new_display(args.1),
                                        ]
                                    }
                                },
                            ),
                        );
                };
                {
                    unsafe {
                        ::godot_ffi::get_interface()
                            .classdb_register_extension_class_method
                            .unwrap_unchecked()
                    }
                }
                (sys::get_library(), class_name.string_sys(), &raw const method_info);
            };
            unsafe {
                use ::godot_core::sys;
                use ::godot_core::builtin::{Variant, StringName};
                use ::godot_core::builtin::meta::*;
                const NUM_ARGS: usize = 0;
                type Sig = ((),);
                let varcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeVariantPtr,
                        _arg_count: sys::GDNativeInt,
                        ret: sys::GDNativeVariantPtr,
                        err: *mut sys::GDNativeCallError,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::varcall::<
                                SineWave2D,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                err,
                                |inst, params| {
                                    let () = params;
                                    inst.full_circle_sin()
                                },
                                "full_circle_sin",
                            )
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"full_circle_sin")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    26u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                            (*err).error = sys::GDNATIVE_CALL_ERROR_INVALID_METHOD;
                            {
                                unsafe {
                                    ::godot_ffi::get_interface()
                                        .variant_new_nil
                                        .unwrap_unchecked()
                                }
                            }
                            (ret);
                        }
                    }
                    function
                };
                let ptrcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeTypePtr,
                        ret: sys::GDNativeTypePtr,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::ptrcall::<
                                SineWave2D,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                |inst, params| {
                                    let () = params;
                                    inst.full_circle_sin()
                                },
                                "full_circle_sin",
                            );
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"full_circle_sin")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    26u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                        }
                    }
                    function
                };
                let has_return_value: bool = true;
                let return_value_info = Sig::property_info(-1, "");
                let mut return_value_info_sys = return_value_info.property_sys();
                let return_value_metadata = Sig::param_metadata(-1);
                let argument_count = NUM_ARGS as u32;
                let mut arguments_info: [PropertyInfo; NUM_ARGS] = {
                    let mut i = -1i32;
                    []
                };
                let mut arguments_info_sys: [sys::GDNativePropertyInfo; NUM_ARGS] = std::array::from_fn(|
                    i|
                arguments_info[i].property_sys());
                let mut arguments_metadata: [sys::GDNativeExtensionClassMethodArgumentMetadata; NUM_ARGS] = std::array::from_fn(|
                    i|
                Sig::param_metadata(i as i32));
                let class_name = StringName::from("SineWave2D");
                let method_name = StringName::from("full_circle_sin");
                let method_info = sys::GDNativeExtensionClassMethodInfo {
                    name: method_name.string_sys(),
                    method_userdata: std::ptr::null_mut(),
                    call_func: Some(varcall_func),
                    ptrcall_func: Some(ptrcall_func),
                    method_flags: sys::GDNATIVE_EXTENSION_METHOD_FLAGS_DEFAULT as u32,
                    has_return_value: has_return_value as u8,
                    return_value_info: &raw mut return_value_info_sys,
                    return_value_metadata,
                    argument_count,
                    arguments_info: arguments_info_sys.as_mut_ptr(),
                    arguments_metadata: arguments_metadata.as_mut_ptr(),
                    default_argument_count: 0,
                    default_arguments: std::ptr::null_mut(),
                };
                {
                    use std::io::{sink, Write};
                    let _ = sink()
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["   Register fn:   ", "::"],
                                &match (&"SineWave2D", &"full_circle_sin") {
                                    args => {
                                        [
                                            ::core::fmt::ArgumentV1::new_display(args.0),
                                            ::core::fmt::ArgumentV1::new_display(args.1),
                                        ]
                                    }
                                },
                            ),
                        );
                };
                {
                    unsafe {
                        ::godot_ffi::get_interface()
                            .classdb_register_extension_class_method
                            .unwrap_unchecked()
                    }
                }
                (sys::get_library(), class_name.string_sys(), &raw const method_info);
            };
            unsafe {
                use ::godot_core::sys;
                use ::godot_core::builtin::{Variant, StringName};
                use ::godot_core::builtin::meta::*;
                const NUM_ARGS: usize = 0;
                type Sig = ((),);
                let varcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeVariantPtr,
                        _arg_count: sys::GDNativeInt,
                        ret: sys::GDNativeVariantPtr,
                        err: *mut sys::GDNativeCallError,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::varcall::<
                                SineWave2D,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                err,
                                |inst, params| {
                                    let () = params;
                                    inst.draw_wave()
                                },
                                "draw_wave",
                            )
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"draw_wave")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    26u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                            (*err).error = sys::GDNATIVE_CALL_ERROR_INVALID_METHOD;
                            {
                                unsafe {
                                    ::godot_ffi::get_interface()
                                        .variant_new_nil
                                        .unwrap_unchecked()
                                }
                            }
                            (ret);
                        }
                    }
                    function
                };
                let ptrcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeTypePtr,
                        ret: sys::GDNativeTypePtr,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::ptrcall::<
                                SineWave2D,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                |inst, params| {
                                    let () = params;
                                    inst.draw_wave()
                                },
                                "draw_wave",
                            );
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"draw_wave")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    26u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                        }
                    }
                    function
                };
                let has_return_value: bool = true;
                let return_value_info = Sig::property_info(-1, "");
                let mut return_value_info_sys = return_value_info.property_sys();
                let return_value_metadata = Sig::param_metadata(-1);
                let argument_count = NUM_ARGS as u32;
                let mut arguments_info: [PropertyInfo; NUM_ARGS] = {
                    let mut i = -1i32;
                    []
                };
                let mut arguments_info_sys: [sys::GDNativePropertyInfo; NUM_ARGS] = std::array::from_fn(|
                    i|
                arguments_info[i].property_sys());
                let mut arguments_metadata: [sys::GDNativeExtensionClassMethodArgumentMetadata; NUM_ARGS] = std::array::from_fn(|
                    i|
                Sig::param_metadata(i as i32));
                let class_name = StringName::from("SineWave2D");
                let method_name = StringName::from("draw_wave");
                let method_info = sys::GDNativeExtensionClassMethodInfo {
                    name: method_name.string_sys(),
                    method_userdata: std::ptr::null_mut(),
                    call_func: Some(varcall_func),
                    ptrcall_func: Some(ptrcall_func),
                    method_flags: sys::GDNATIVE_EXTENSION_METHOD_FLAGS_DEFAULT as u32,
                    has_return_value: has_return_value as u8,
                    return_value_info: &raw mut return_value_info_sys,
                    return_value_metadata,
                    argument_count,
                    arguments_info: arguments_info_sys.as_mut_ptr(),
                    arguments_metadata: arguments_metadata.as_mut_ptr(),
                    default_argument_count: 0,
                    default_arguments: std::ptr::null_mut(),
                };
                {
                    use std::io::{sink, Write};
                    let _ = sink()
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["   Register fn:   ", "::"],
                                &match (&"SineWave2D", &"draw_wave") {
                                    args => {
                                        [
                                            ::core::fmt::ArgumentV1::new_display(args.0),
                                            ::core::fmt::ArgumentV1::new_display(args.1),
                                        ]
                                    }
                                },
                            ),
                        );
                };
                {
                    unsafe {
                        ::godot_ffi::get_interface()
                            .classdb_register_extension_class_method
                            .unwrap_unchecked()
                    }
                }
                (sys::get_library(), class_name.string_sys(), &raw const method_info);
            };
            unsafe {
                let class_name = ::godot::builtin::StringName::from("SineWave2D");
                use ::godot::sys;
            }
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        #[used]
        #[link_section = ".init_array"]
        static __init: extern "C" fn() = {
            #[link_section = ".text.startup"]
            extern "C" fn __inner_init() {
                let mut guard = ::godot::private::__godot_rust_plugin___GODOT_PLUGIN_REGISTRY
                    .lock()
                    .unwrap();
                guard
                    .push(::godot::private::ClassPlugin {
                        class_name: "SineWave2D",
                        component: ::godot::private::PluginComponent::UserMethodBinds {
                            generated_register_fn: ::godot::private::ErasedRegisterFn {
                                raw: ::godot::private::callbacks::register_user_binds::<
                                    SineWave2D,
                                >,
                            },
                        },
                    });
            }
            __inner_init
        };
    };
    impl GodotExt for SineWave2D {
        fn init(base: Base<Line2D>) -> Self {
            SineWave2D { score: 0, base }
        }
        fn ready(&mut self) {
            self.full_circle_sin();
        }
    }
    impl ::godot::obj::cap::GodotInit for SineWave2D {
        fn __godot_init(base: ::godot::obj::Base<Self::Base>) -> Self {
            <Self as ::godot::bind::GodotExt>::init(base)
        }
    }
    impl ::godot::private::You_forgot_the_attribute__godot_api for SineWave2D {}
    impl ::godot::obj::cap::ImplementsGodotExt for SineWave2D {
        fn __virtual_call(
            name: &str,
        ) -> ::godot::sys::GDNativeExtensionClassCallVirtual {
            match name {
                "_ready" => {
                    Some({
                        use ::godot_core::sys;
                        unsafe extern "C" fn function(
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args: *const sys::GDNativeTypePtr,
                            ret: sys::GDNativeTypePtr,
                        ) {
                            use ::godot_core::sys;
                            {
                                use std::io::{sink, Write};
                                let _ = sink()
                                    .write_fmt(
                                        ::core::fmt::Arguments::new_v1(
                                            &["ptrcall: "],
                                            &[::core::fmt::ArgumentV1::new_display(&"ready")],
                                        ),
                                    );
                            };
                            let storage = ::godot_core::private::as_storage::<
                                SineWave2D,
                            >(instance_ptr);
                            let mut instance = storage.get_mut();
                            let mut idx = 0;
                            let ret_val = instance.ready();
                            <() as sys::GodotFfi>::write_sys(&ret_val, ret);
                            std::mem::forget(ret_val);
                        }
                        function
                    })
                }
                _ => None,
            }
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        #[used]
        #[link_section = ".init_array"]
        static __init: extern "C" fn() = {
            #[link_section = ".text.startup"]
            extern "C" fn __inner_init() {
                let mut guard = ::godot::private::__godot_rust_plugin___GODOT_PLUGIN_REGISTRY
                    .lock()
                    .unwrap();
                guard
                    .push(::godot::private::ClassPlugin {
                        class_name: "SineWave2D",
                        component: ::godot::private::PluginComponent::UserVirtuals {
                            user_register_fn: None,
                            user_create_fn: Some(
                                ::godot::private::callbacks::create::<SineWave2D>,
                            ),
                            user_to_string_fn: None,
                            get_virtual_fn: ::godot::private::callbacks::get_virtual::<
                                SineWave2D,
                            >,
                        },
                    });
            }
            __inner_init
        };
    };
    #[class(base = Node)]
    pub struct Main {
        score: i64,
        #[base]
        base: Base<Node>,
    }
    impl ::godot::obj::GodotClass for Main {
        type Base = ::godot::engine::Node;
        type Declarer = ::godot::obj::dom::UserDomain;
        type Mem = <Self::Base as ::godot::obj::GodotClass>::Mem;
        const CLASS_NAME: &'static str = "Main";
    }
    impl std::ops::Deref for Main {
        type Target = <Self as ::godot::obj::GodotClass>::Base;
        fn deref(&self) -> &Self::Target {
            &*self.base
        }
    }
    impl std::ops::DerefMut for Main {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.base
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        #[used]
        #[link_section = ".init_array"]
        static __init: extern "C" fn() = {
            #[link_section = ".text.startup"]
            extern "C" fn __inner_init() {
                let mut guard = ::godot::private::__godot_rust_plugin___GODOT_PLUGIN_REGISTRY
                    .lock()
                    .unwrap();
                guard
                    .push(::godot::private::ClassPlugin {
                        class_name: "Main",
                        component: ::godot::private::PluginComponent::ClassDef {
                            base_class_name: "Node",
                            generated_create_fn: None,
                            free_fn: ::godot::private::callbacks::free::<Main>,
                        },
                    });
            }
            __inner_init
        };
    };
    impl ::godot::obj::Inherits<::godot::engine::Node> for Main {}
    impl ::godot::obj::Inherits<::godot::engine::Object> for Main {}
    impl Main {
        fn game_over(&mut self) {
            {
                ::std::io::_print(
                    ::core::fmt::Arguments::new_v1(&["Game over!\n"], &[]),
                );
            };
        }
        pub fn new_game(&mut self) {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["New game\n"], &[]));
            };
        }
        fn test_sin(&mut self, angle: f64, answer: f64) {
            match (
                &{
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1_formatted(
                            &[""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &(sin(angle.to_radians()) * 10.0),
                                ),
                            ],
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Is(1usize),
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                            unsafe { ::core::fmt::UnsafeArg::new() },
                        ),
                    );
                    res
                },
                &{
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1_formatted(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&(answer * 10.0))],
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Is(1usize),
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                            unsafe { ::core::fmt::UnsafeArg::new() },
                        ),
                    );
                    res
                },
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
        fn full_circle_sin(&mut self) {
            self.test_sin(30.0, 0.50);
            self.test_sin(60.0, 0.87);
            self.test_sin(90.0, 1.0);
            self.test_sin(120.0, 0.87);
            self.test_sin(150.0, 0.5);
            self.test_sin(180.0, f64::EPSILON);
            self.test_sin(210.0, -0.50);
            self.test_sin(240.0, -0.87);
            self.test_sin(270.0, -1.0);
            self.test_sin(300.0, -0.87);
            self.test_sin(330.0, -0.5);
            self.test_sin(360.0, -f64::EPSILON);
        }
        fn draw_circle(&mut self) {}
    }
    impl ::godot::obj::cap::ImplementsGodotApi for Main {
        fn __register_methods() {
            unsafe {
                use ::godot_core::sys;
                use ::godot_core::builtin::{Variant, StringName};
                use ::godot_core::builtin::meta::*;
                const NUM_ARGS: usize = 0;
                type Sig = ((),);
                let varcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeVariantPtr,
                        _arg_count: sys::GDNativeInt,
                        ret: sys::GDNativeVariantPtr,
                        err: *mut sys::GDNativeCallError,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::varcall::<
                                Main,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                err,
                                |inst, params| {
                                    let () = params;
                                    inst.game_over()
                                },
                                "game_over",
                            )
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"game_over")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    140u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                            (*err).error = sys::GDNATIVE_CALL_ERROR_INVALID_METHOD;
                            {
                                unsafe {
                                    ::godot_ffi::get_interface()
                                        .variant_new_nil
                                        .unwrap_unchecked()
                                }
                            }
                            (ret);
                        }
                    }
                    function
                };
                let ptrcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeTypePtr,
                        ret: sys::GDNativeTypePtr,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::ptrcall::<
                                Main,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                |inst, params| {
                                    let () = params;
                                    inst.game_over()
                                },
                                "game_over",
                            );
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"game_over")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    140u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                        }
                    }
                    function
                };
                let has_return_value: bool = true;
                let return_value_info = Sig::property_info(-1, "");
                let mut return_value_info_sys = return_value_info.property_sys();
                let return_value_metadata = Sig::param_metadata(-1);
                let argument_count = NUM_ARGS as u32;
                let mut arguments_info: [PropertyInfo; NUM_ARGS] = {
                    let mut i = -1i32;
                    []
                };
                let mut arguments_info_sys: [sys::GDNativePropertyInfo; NUM_ARGS] = std::array::from_fn(|
                    i|
                arguments_info[i].property_sys());
                let mut arguments_metadata: [sys::GDNativeExtensionClassMethodArgumentMetadata; NUM_ARGS] = std::array::from_fn(|
                    i|
                Sig::param_metadata(i as i32));
                let class_name = StringName::from("Main");
                let method_name = StringName::from("game_over");
                let method_info = sys::GDNativeExtensionClassMethodInfo {
                    name: method_name.string_sys(),
                    method_userdata: std::ptr::null_mut(),
                    call_func: Some(varcall_func),
                    ptrcall_func: Some(ptrcall_func),
                    method_flags: sys::GDNATIVE_EXTENSION_METHOD_FLAGS_DEFAULT as u32,
                    has_return_value: has_return_value as u8,
                    return_value_info: &raw mut return_value_info_sys,
                    return_value_metadata,
                    argument_count,
                    arguments_info: arguments_info_sys.as_mut_ptr(),
                    arguments_metadata: arguments_metadata.as_mut_ptr(),
                    default_argument_count: 0,
                    default_arguments: std::ptr::null_mut(),
                };
                {
                    use std::io::{sink, Write};
                    let _ = sink()
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["   Register fn:   ", "::"],
                                &match (&"Main", &"game_over") {
                                    args => {
                                        [
                                            ::core::fmt::ArgumentV1::new_display(args.0),
                                            ::core::fmt::ArgumentV1::new_display(args.1),
                                        ]
                                    }
                                },
                            ),
                        );
                };
                {
                    unsafe {
                        ::godot_ffi::get_interface()
                            .classdb_register_extension_class_method
                            .unwrap_unchecked()
                    }
                }
                (sys::get_library(), class_name.string_sys(), &raw const method_info);
            };
            unsafe {
                use ::godot_core::sys;
                use ::godot_core::builtin::{Variant, StringName};
                use ::godot_core::builtin::meta::*;
                const NUM_ARGS: usize = 0;
                type Sig = ((),);
                let varcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeVariantPtr,
                        _arg_count: sys::GDNativeInt,
                        ret: sys::GDNativeVariantPtr,
                        err: *mut sys::GDNativeCallError,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::varcall::<
                                Main,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                err,
                                |inst, params| {
                                    let () = params;
                                    inst.new_game()
                                },
                                "new_game",
                            )
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"new_game")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    140u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                            (*err).error = sys::GDNATIVE_CALL_ERROR_INVALID_METHOD;
                            {
                                unsafe {
                                    ::godot_ffi::get_interface()
                                        .variant_new_nil
                                        .unwrap_unchecked()
                                }
                            }
                            (ret);
                        }
                    }
                    function
                };
                let ptrcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeTypePtr,
                        ret: sys::GDNativeTypePtr,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::ptrcall::<
                                Main,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                |inst, params| {
                                    let () = params;
                                    inst.new_game()
                                },
                                "new_game",
                            );
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"new_game")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    140u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                        }
                    }
                    function
                };
                let has_return_value: bool = true;
                let return_value_info = Sig::property_info(-1, "");
                let mut return_value_info_sys = return_value_info.property_sys();
                let return_value_metadata = Sig::param_metadata(-1);
                let argument_count = NUM_ARGS as u32;
                let mut arguments_info: [PropertyInfo; NUM_ARGS] = {
                    let mut i = -1i32;
                    []
                };
                let mut arguments_info_sys: [sys::GDNativePropertyInfo; NUM_ARGS] = std::array::from_fn(|
                    i|
                arguments_info[i].property_sys());
                let mut arguments_metadata: [sys::GDNativeExtensionClassMethodArgumentMetadata; NUM_ARGS] = std::array::from_fn(|
                    i|
                Sig::param_metadata(i as i32));
                let class_name = StringName::from("Main");
                let method_name = StringName::from("new_game");
                let method_info = sys::GDNativeExtensionClassMethodInfo {
                    name: method_name.string_sys(),
                    method_userdata: std::ptr::null_mut(),
                    call_func: Some(varcall_func),
                    ptrcall_func: Some(ptrcall_func),
                    method_flags: sys::GDNATIVE_EXTENSION_METHOD_FLAGS_DEFAULT as u32,
                    has_return_value: has_return_value as u8,
                    return_value_info: &raw mut return_value_info_sys,
                    return_value_metadata,
                    argument_count,
                    arguments_info: arguments_info_sys.as_mut_ptr(),
                    arguments_metadata: arguments_metadata.as_mut_ptr(),
                    default_argument_count: 0,
                    default_arguments: std::ptr::null_mut(),
                };
                {
                    use std::io::{sink, Write};
                    let _ = sink()
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["   Register fn:   ", "::"],
                                &match (&"Main", &"new_game") {
                                    args => {
                                        [
                                            ::core::fmt::ArgumentV1::new_display(args.0),
                                            ::core::fmt::ArgumentV1::new_display(args.1),
                                        ]
                                    }
                                },
                            ),
                        );
                };
                {
                    unsafe {
                        ::godot_ffi::get_interface()
                            .classdb_register_extension_class_method
                            .unwrap_unchecked()
                    }
                }
                (sys::get_library(), class_name.string_sys(), &raw const method_info);
            };
            unsafe {
                use ::godot_core::sys;
                use ::godot_core::builtin::{Variant, StringName};
                use ::godot_core::builtin::meta::*;
                const NUM_ARGS: usize = 1 + (1 + 0);
                type Sig = ((), f64, f64);
                let varcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeVariantPtr,
                        _arg_count: sys::GDNativeInt,
                        ret: sys::GDNativeVariantPtr,
                        err: *mut sys::GDNativeCallError,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::varcall::<
                                Main,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                err,
                                |inst, params| {
                                    let (angle, answer) = params;
                                    inst.test_sin(angle, answer)
                                },
                                "test_sin",
                            )
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"test_sin")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    140u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                            (*err).error = sys::GDNATIVE_CALL_ERROR_INVALID_METHOD;
                            {
                                unsafe {
                                    ::godot_ffi::get_interface()
                                        .variant_new_nil
                                        .unwrap_unchecked()
                                }
                            }
                            (ret);
                        }
                    }
                    function
                };
                let ptrcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeTypePtr,
                        ret: sys::GDNativeTypePtr,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::ptrcall::<
                                Main,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                |inst, params| {
                                    let (angle, answer) = params;
                                    inst.test_sin(angle, answer)
                                },
                                "test_sin",
                            );
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"test_sin")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    140u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                        }
                    }
                    function
                };
                let has_return_value: bool = true;
                let return_value_info = Sig::property_info(-1, "");
                let mut return_value_info_sys = return_value_info.property_sys();
                let return_value_metadata = Sig::param_metadata(-1);
                let argument_count = NUM_ARGS as u32;
                let mut arguments_info: [PropertyInfo; NUM_ARGS] = {
                    let mut i = -1i32;
                    [
                        {
                            i += 1;
                            let prop = Sig::property_info(i, "angle");
                            prop
                        },
                        {
                            i += 1;
                            let prop = Sig::property_info(i, "answer");
                            prop
                        },
                    ]
                };
                let mut arguments_info_sys: [sys::GDNativePropertyInfo; NUM_ARGS] = std::array::from_fn(|
                    i|
                arguments_info[i].property_sys());
                let mut arguments_metadata: [sys::GDNativeExtensionClassMethodArgumentMetadata; NUM_ARGS] = std::array::from_fn(|
                    i|
                Sig::param_metadata(i as i32));
                let class_name = StringName::from("Main");
                let method_name = StringName::from("test_sin");
                let method_info = sys::GDNativeExtensionClassMethodInfo {
                    name: method_name.string_sys(),
                    method_userdata: std::ptr::null_mut(),
                    call_func: Some(varcall_func),
                    ptrcall_func: Some(ptrcall_func),
                    method_flags: sys::GDNATIVE_EXTENSION_METHOD_FLAGS_DEFAULT as u32,
                    has_return_value: has_return_value as u8,
                    return_value_info: &raw mut return_value_info_sys,
                    return_value_metadata,
                    argument_count,
                    arguments_info: arguments_info_sys.as_mut_ptr(),
                    arguments_metadata: arguments_metadata.as_mut_ptr(),
                    default_argument_count: 0,
                    default_arguments: std::ptr::null_mut(),
                };
                {
                    use std::io::{sink, Write};
                    let _ = sink()
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["   Register fn:   ", "::"],
                                &match (&"Main", &"test_sin") {
                                    args => {
                                        [
                                            ::core::fmt::ArgumentV1::new_display(args.0),
                                            ::core::fmt::ArgumentV1::new_display(args.1),
                                        ]
                                    }
                                },
                            ),
                        );
                };
                {
                    unsafe {
                        ::godot_ffi::get_interface()
                            .classdb_register_extension_class_method
                            .unwrap_unchecked()
                    }
                }
                (sys::get_library(), class_name.string_sys(), &raw const method_info);
            };
            unsafe {
                use ::godot_core::sys;
                use ::godot_core::builtin::{Variant, StringName};
                use ::godot_core::builtin::meta::*;
                const NUM_ARGS: usize = 0;
                type Sig = ((),);
                let varcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeVariantPtr,
                        _arg_count: sys::GDNativeInt,
                        ret: sys::GDNativeVariantPtr,
                        err: *mut sys::GDNativeCallError,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::varcall::<
                                Main,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                err,
                                |inst, params| {
                                    let () = params;
                                    inst.full_circle_sin()
                                },
                                "full_circle_sin",
                            )
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"full_circle_sin")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    140u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                            (*err).error = sys::GDNATIVE_CALL_ERROR_INVALID_METHOD;
                            {
                                unsafe {
                                    ::godot_ffi::get_interface()
                                        .variant_new_nil
                                        .unwrap_unchecked()
                                }
                            }
                            (ret);
                        }
                    }
                    function
                };
                let ptrcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeTypePtr,
                        ret: sys::GDNativeTypePtr,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::ptrcall::<
                                Main,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                |inst, params| {
                                    let () = params;
                                    inst.full_circle_sin()
                                },
                                "full_circle_sin",
                            );
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"full_circle_sin")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    140u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                        }
                    }
                    function
                };
                let has_return_value: bool = true;
                let return_value_info = Sig::property_info(-1, "");
                let mut return_value_info_sys = return_value_info.property_sys();
                let return_value_metadata = Sig::param_metadata(-1);
                let argument_count = NUM_ARGS as u32;
                let mut arguments_info: [PropertyInfo; NUM_ARGS] = {
                    let mut i = -1i32;
                    []
                };
                let mut arguments_info_sys: [sys::GDNativePropertyInfo; NUM_ARGS] = std::array::from_fn(|
                    i|
                arguments_info[i].property_sys());
                let mut arguments_metadata: [sys::GDNativeExtensionClassMethodArgumentMetadata; NUM_ARGS] = std::array::from_fn(|
                    i|
                Sig::param_metadata(i as i32));
                let class_name = StringName::from("Main");
                let method_name = StringName::from("full_circle_sin");
                let method_info = sys::GDNativeExtensionClassMethodInfo {
                    name: method_name.string_sys(),
                    method_userdata: std::ptr::null_mut(),
                    call_func: Some(varcall_func),
                    ptrcall_func: Some(ptrcall_func),
                    method_flags: sys::GDNATIVE_EXTENSION_METHOD_FLAGS_DEFAULT as u32,
                    has_return_value: has_return_value as u8,
                    return_value_info: &raw mut return_value_info_sys,
                    return_value_metadata,
                    argument_count,
                    arguments_info: arguments_info_sys.as_mut_ptr(),
                    arguments_metadata: arguments_metadata.as_mut_ptr(),
                    default_argument_count: 0,
                    default_arguments: std::ptr::null_mut(),
                };
                {
                    use std::io::{sink, Write};
                    let _ = sink()
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["   Register fn:   ", "::"],
                                &match (&"Main", &"full_circle_sin") {
                                    args => {
                                        [
                                            ::core::fmt::ArgumentV1::new_display(args.0),
                                            ::core::fmt::ArgumentV1::new_display(args.1),
                                        ]
                                    }
                                },
                            ),
                        );
                };
                {
                    unsafe {
                        ::godot_ffi::get_interface()
                            .classdb_register_extension_class_method
                            .unwrap_unchecked()
                    }
                }
                (sys::get_library(), class_name.string_sys(), &raw const method_info);
            };
            unsafe {
                use ::godot_core::sys;
                use ::godot_core::builtin::{Variant, StringName};
                use ::godot_core::builtin::meta::*;
                const NUM_ARGS: usize = 0;
                type Sig = ((),);
                let varcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeVariantPtr,
                        _arg_count: sys::GDNativeInt,
                        ret: sys::GDNativeVariantPtr,
                        err: *mut sys::GDNativeCallError,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::varcall::<
                                Main,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                err,
                                |inst, params| {
                                    let () = params;
                                    inst.draw_circle()
                                },
                                "draw_circle",
                            )
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"draw_circle")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    140u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                            (*err).error = sys::GDNATIVE_CALL_ERROR_INVALID_METHOD;
                            {
                                unsafe {
                                    ::godot_ffi::get_interface()
                                        .variant_new_nil
                                        .unwrap_unchecked()
                                }
                            }
                            (ret);
                        }
                    }
                    function
                };
                let ptrcall_func = {
                    unsafe extern "C" fn function(
                        _method_data: *mut std::ffi::c_void,
                        instance_ptr: sys::GDExtensionClassInstancePtr,
                        args: *const sys::GDNativeTypePtr,
                        ret: sys::GDNativeTypePtr,
                    ) {
                        let result = ::std::panic::catch_unwind(|| {
                            <Sig as SignatureTuple>::ptrcall::<
                                Main,
                            >(
                                instance_ptr,
                                args,
                                ret,
                                |inst, params| {
                                    let () = params;
                                    inst.draw_circle()
                                },
                                "draw_circle",
                            );
                        });
                        if let Err(e) = result {
                            unsafe {
                                let msg = {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["", "\u{0}"],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(
                                                    &::core::fmt::Arguments::new_v1(
                                                        &["Rust function panicked: "],
                                                        &[::core::fmt::ArgumentV1::new_display(&"draw_circle")],
                                                    ),
                                                ),
                                            ],
                                        ),
                                    );
                                    res
                                };
                                {
                                    unsafe {
                                        ::godot_ffi::get_interface().print_error.unwrap_unchecked()
                                    }
                                }
                                (
                                    msg.as_bytes().as_ptr() as *const _,
                                    "<function unset>\0".as_bytes().as_ptr() as *const _,
                                    "examples/rsty_physics/rsty_physics/src/trig.rs\u{0}"
                                        .as_ptr() as *const _,
                                    140u32 as _,
                                );
                            };
                            ::godot_core::private::print_panic(e);
                        }
                    }
                    function
                };
                let has_return_value: bool = true;
                let return_value_info = Sig::property_info(-1, "");
                let mut return_value_info_sys = return_value_info.property_sys();
                let return_value_metadata = Sig::param_metadata(-1);
                let argument_count = NUM_ARGS as u32;
                let mut arguments_info: [PropertyInfo; NUM_ARGS] = {
                    let mut i = -1i32;
                    []
                };
                let mut arguments_info_sys: [sys::GDNativePropertyInfo; NUM_ARGS] = std::array::from_fn(|
                    i|
                arguments_info[i].property_sys());
                let mut arguments_metadata: [sys::GDNativeExtensionClassMethodArgumentMetadata; NUM_ARGS] = std::array::from_fn(|
                    i|
                Sig::param_metadata(i as i32));
                let class_name = StringName::from("Main");
                let method_name = StringName::from("draw_circle");
                let method_info = sys::GDNativeExtensionClassMethodInfo {
                    name: method_name.string_sys(),
                    method_userdata: std::ptr::null_mut(),
                    call_func: Some(varcall_func),
                    ptrcall_func: Some(ptrcall_func),
                    method_flags: sys::GDNATIVE_EXTENSION_METHOD_FLAGS_DEFAULT as u32,
                    has_return_value: has_return_value as u8,
                    return_value_info: &raw mut return_value_info_sys,
                    return_value_metadata,
                    argument_count,
                    arguments_info: arguments_info_sys.as_mut_ptr(),
                    arguments_metadata: arguments_metadata.as_mut_ptr(),
                    default_argument_count: 0,
                    default_arguments: std::ptr::null_mut(),
                };
                {
                    use std::io::{sink, Write};
                    let _ = sink()
                        .write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["   Register fn:   ", "::"],
                                &match (&"Main", &"draw_circle") {
                                    args => {
                                        [
                                            ::core::fmt::ArgumentV1::new_display(args.0),
                                            ::core::fmt::ArgumentV1::new_display(args.1),
                                        ]
                                    }
                                },
                            ),
                        );
                };
                {
                    unsafe {
                        ::godot_ffi::get_interface()
                            .classdb_register_extension_class_method
                            .unwrap_unchecked()
                    }
                }
                (sys::get_library(), class_name.string_sys(), &raw const method_info);
            };
            unsafe {
                let class_name = ::godot::builtin::StringName::from("Main");
                use ::godot::sys;
            }
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        #[used]
        #[link_section = ".init_array"]
        static __init: extern "C" fn() = {
            #[link_section = ".text.startup"]
            extern "C" fn __inner_init() {
                let mut guard = ::godot::private::__godot_rust_plugin___GODOT_PLUGIN_REGISTRY
                    .lock()
                    .unwrap();
                guard
                    .push(::godot::private::ClassPlugin {
                        class_name: "Main",
                        component: ::godot::private::PluginComponent::UserMethodBinds {
                            generated_register_fn: ::godot::private::ErasedRegisterFn {
                                raw: ::godot::private::callbacks::register_user_binds::<
                                    Main,
                                >,
                            },
                        },
                    });
            }
            __inner_init
        };
    };
    impl GodotExt for Main {
        fn init(base: Base<Node>) -> Self {
            Main { score: 0, base }
        }
        fn ready(&mut self) {
            self.full_circle_sin();
            self.draw_circle();
        }
    }
    impl ::godot::obj::cap::GodotInit for Main {
        fn __godot_init(base: ::godot::obj::Base<Self::Base>) -> Self {
            <Self as ::godot::bind::GodotExt>::init(base)
        }
    }
    impl ::godot::private::You_forgot_the_attribute__godot_api for Main {}
    impl ::godot::obj::cap::ImplementsGodotExt for Main {
        fn __virtual_call(
            name: &str,
        ) -> ::godot::sys::GDNativeExtensionClassCallVirtual {
            match name {
                "_ready" => {
                    Some({
                        use ::godot_core::sys;
                        unsafe extern "C" fn function(
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args: *const sys::GDNativeTypePtr,
                            ret: sys::GDNativeTypePtr,
                        ) {
                            use ::godot_core::sys;
                            {
                                use std::io::{sink, Write};
                                let _ = sink()
                                    .write_fmt(
                                        ::core::fmt::Arguments::new_v1(
                                            &["ptrcall: "],
                                            &[::core::fmt::ArgumentV1::new_display(&"ready")],
                                        ),
                                    );
                            };
                            let storage = ::godot_core::private::as_storage::<
                                Main,
                            >(instance_ptr);
                            let mut instance = storage.get_mut();
                            let mut idx = 0;
                            let ret_val = instance.ready();
                            <() as sys::GodotFfi>::write_sys(&ret_val, ret);
                            std::mem::forget(ret_val);
                        }
                        function
                    })
                }
                _ => None,
            }
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        #[used]
        #[link_section = ".init_array"]
        static __init: extern "C" fn() = {
            #[link_section = ".text.startup"]
            extern "C" fn __inner_init() {
                let mut guard = ::godot::private::__godot_rust_plugin___GODOT_PLUGIN_REGISTRY
                    .lock()
                    .unwrap();
                guard
                    .push(::godot::private::ClassPlugin {
                        class_name: "Main",
                        component: ::godot::private::PluginComponent::UserVirtuals {
                            user_register_fn: None,
                            user_create_fn: Some(
                                ::godot::private::callbacks::create::<Main>,
                            ),
                            user_to_string_fn: None,
                            get_virtual_fn: ::godot::private::callbacks::get_virtual::<
                                Main,
                            >,
                        },
                    });
            }
            __inner_init
        };
    };
}
unsafe impl ExtensionLibrary for RustyPhysics {}
#[no_mangle]
unsafe extern "C" fn gdextension_rust_init(
    interface: *const ::godot::sys::GDNativeInterface,
    library: ::godot::sys::GDNativeExtensionClassLibraryPtr,
    init: *mut ::godot::sys::GDNativeInitialization,
) -> ::godot::sys::GDNativeBool {
    ::godot::init::__gdext_load_library::<RustyPhysics>(interface, library, init)
}
fn __static_type_check() {
    let _unused: ::godot::sys::GDNativeInitializationFunction = Some(
        gdextension_rust_init,
    );
}
