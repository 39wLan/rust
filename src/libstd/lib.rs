//! # Rust标准库
//!
//! Rust标准库是可移植Rust软件的基础，Rust软件是针对[更广泛的Rust生态系统][crates.io].的一组最小且经过测试的共享抽象. 它提供了诸如[`Vec<T>`] 和 [`Option<T>`]之类的核心
//! 类型, 对[语言原语](#primitives)的库定义操作, [标准宏](#macros]),  [I/O] 和 [多线程], [等等][other].
//!
//! `Std`默认情况下可用于所有Rust库. 因此, 标准库可以在 [`use`] 语句 通过 std路径访问, 就像使用[`use std::env`]一样.
//!
//! # 如何阅读本文档
//!
//! 如果您已经知道要查找的名称，最快的查找方法是使用页面顶部的 <a href="#" onclick="focusSearchBar();">搜索栏 </a> .
//!
//! 否则，您可能要跳到以下有用的部分之一：
//!
//! * [`std::*` 模块](#modules)
//! * [原生类型](#primitives)
//! * [标准宏](#macros)
//! * [Rust前导](prelude/index.html)
//!
//! 如果这是您第一次来，那么标准库的文档随意阅读。单击有趣的事物通常会把您带到
//! 有趣的地方。 尽管如此，您还是不想错过一些重要的内容，因此请继续阅读标准库及其文档！
//!
//! 一旦您熟悉了标准库的内容，您可能会发现散文使人分心。在开发的此阶段，您可能
//! 需要按页面顶部附近的[-] 按钮以将其折叠为更可浏览的视图。
//!
//! 当您查看该 `[-]`按钮时，也请注意该 `[src]`按钮。Rust的API文档带
//! 有源代码，建议您阅读。标准库资源通常是高质量的，幕后的偷看常常会启发人 
//!
//! # 标准库文档中有什么？
//!
//! 首先，Rust标准库分为多个重点模块, [所有内容都在此页面的下方列出](#modules). 这些模块是所有Rus构造的基石.
//! 它们的名称很强大，例如 [`std::slice`] and [`std::cmp`]. 模块的文档通常包括模块概述和示例，是开始熟悉该库的明智之地.
//!
//! 其次，此处记录了 [原生类型]的隐式方法. 造成混淆的原因有两个：
//!
//! 1. 由编译器实现原语时，标准库直接在原生类型上实现方法(这是唯一这样做的库), 该方法在[原生类型](#primitives).部分中进行了介绍
//! 2. 标准库导出了许多 与原生类型同名的模块,这些定义了与原生类型相关的其他项，但没有定义所有重要方法.
//!
//! 因此，例如 有一个[原始类型 `i32`](primitive.i32.html)  页面 , 列出了可以在32位整数上调用的所有方法 (非常有用), 而
//! 有一个模块 [std::i32](i32/index.html) 页面, 记录了常量值 [`MIN`]和 [`MAX`](i32/constant.MAX.html)  (非常有用).
//!
//! 请注意原生[`str`]  和 [`T`](slice)(也称为 "切片" )的文档. 通过[deref.强制](deref-coercions)， 对[`String`] 和 [`Vec<T>`] 的许多方法调用实际上分别是对 [`str`] 和 [`T`](slice) 上的方法的调用.
//!
//! 第三，标准库定义了 [Rust前导], 这是一小部分项(主要是特质)的集合- 这些项被导入到每个 Crate 的每个模块中。前导中的特质无处不在，这使t前导文档成为学习标准库的一个很好的切入点
//!
//! 最后，标准库导出许多标准宏，并[在此页上列出它们](#macros) (从技术上讲，并非所有标准宏都由标准库定义-有些是由编译器定义的，但与此处记录的内容相同) 像前导一样，默认情况下，标准宏会导入所有 Crate 中
//!
//! # 对文档进行更改
//!
//! 在[此处查看Rust贡献准则](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md)。 该文档的源代码可以在[Github](https://github.com/rust-lang)上找到。要做出更改，请确保您先阅读准则，然后提交建议更改的请求。
//!
//!感谢您的贡献！如果您发现文档中的一部分可以改进，请提交PR, 或首先在irc.mozilla.org＃rust-docs上与我们交流.
//!
//! # Rust标准库之旅
//!
//! 本文档的其余部分专用于说明Rust标准库的显着功能

//!
//! ## 容器和集合
//!
//!   [`option`] 和 [`result`] 模块定义可选和错误处理的类型 : [`Option<T>`] 和 [`Result<T, E>`].  [`iter`]  模块定义Rust的迭代器特质 [`Iterator`], 与[`for`] 循环一起使用以访问集合.
//!
//! T标准库公开了三种处理内存连续区域的常用方法：
//!
//! * [`Vec<T>`] - 在运行时可调整大小的堆分配向量/`vector`
//! * [`[T; n]`][array] - 在编译时具有固定大小的内联数组
//! * [`[T]`][slice] - 具有容纳动态大小切片的任何类型连续存储, 无论是否进行堆分配
//!
//! 切片只能通过某种指针来处理，因此有多种形式，例如：
//!
//! * `&[T]` - *共享切片*
//! * `&mut [T]` - *可变切片*
//! * [`Box<[T]>`][owned slice] - *拥有切片*
//!
//! [`str`], UTF-8字符串切片是一种原始类型, 标准库为此定义了许多方法. Rust  [`str`]通常被当作不可变的引用:  `&str`来访问 . 使用拥有/`owned`  [`String`]来构造和可变字符串
//!
//! 要转换为字符串，请使用 [`format!`] 宏；要从字符串转换，请使用 [`FromStr`] 特质.
//!
//! 可以通过将数据放在引用计数 box 或 [`Rc`] 类型中来共享数据, 如果进一步包含在 [`Cell`] 或 [`RefCell`] ,则可以对其进行突变和共享.
//! Likewise, in a concurrent setting it is common to pair an
//! atomically-reference-counted box, [`Arc`], with a [`Mutex`] to get the same
//! effect.
//!
//! 该[`collections`] 模块定义了 maps , sets , linked lists 和其他典型的集合类型，包括 [`HashMap<K, V>`].
//!
//! ## 平台抽象和 I/O
//!
//! 除了基本数据类型外，标准库还主要关注常见平台（尤其是Windows和Unix派生）中的差异的抽象
//!
//! 常见类型的 I/O, 包括 [files], [TCP], [UDP], 在被定义在 [`io`], [`fs`], 和 [`net`] 模块.
//!
//! 该 [`thread`] 模块包含Rust的线程抽象. [`sync`] 进一步包含其他原始共享内存类型, 包括 [`atomic`] 和 [`mpsc`], 其中包含用于消息传递的通道类型.
//!
//! [I/O]: io/index.html
//! [`MIN`]: i32/constant.MIN.html
//! [TCP]: net/struct.TcpStream.html
//! [Rust前导]: prelude/index.html
//! [UDP]: net/struct.UdpSocket.html
//! [`Arc`]: sync/struct.Arc.html
//! [owned slice]: boxed/index.html
//! [`Cell`]: cell/struct.Cell.html
//! [`FromStr`]: str/trait.FromStr.html
//! [`HashMap<K, V>`]: collections/struct.HashMap.html
//! [`Iterator`]: iter/trait.Iterator.html
//! [`Mutex`]: sync/struct.Mutex.html
//! [`Option<T>`]: option/enum.Option.html
//! [`Rc`]: rc/index.html
//! [`RefCell`]: cell/struct.RefCell.html
//! [`Result<T, E>`]: result/enum.Result.html
//! [`String`]: string/struct.String.html
//! [`Vec<T>`]: vec/index.html
//! [array]: primitive.array.html
//! [slice]: primitive.slice.html
//! [`atomic`]: sync/atomic/index.html
//! [`collections`]: collections/index.html
//! [`for`]: ../book/ch03-05-control-flow.html#looping-through-a-collection-with-for
//! [`format!`]: macro.format.html
//! [`fs`]: fs/index.html
//! [`io`]: io/index.html
//! [`iter`]: iter/index.html
//! [`mpsc`]: sync/mpsc/index.html
//! [`net`]: net/index.html
//! [`option`]: option/index.html
//! [`result`]: result/index.html
//! [`std::cmp`]: cmp/index.html
//! [`std::slice`]: slice/index.html
//! [`str`]: primitive.str.html
//! [`sync`]: sync/index.html
//! [`thread`]: thread/index.html
//! [`use std::env`]: env/index.html
//! [`use`]: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
//! [crates.io]: https://crates.io
//! [deref-coercions]: ../book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
//! [files]: fs/struct.File.html
//! [多线程]: thread/index.html
//! [other]: #what-is-in-the-standard-library-documentation
//! [原生类型]: ../book/ch03-02-data-types.html

#![stable(feature = "rust1", since = "1.0.0")]
#![doc(html_root_url = "https://doc.rust-lang.org/nightly/",
       html_playground_url = "https://play.rust-lang.org/",
       issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
       test(no_crate_inject, attr(deny(warnings))),
       test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))))]

// Don't link to std. We are std.
#![no_std]

#![warn(deprecated_in_future)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
#![allow(explicit_outlives_requirements)]
#![allow(unused_lifetimes)]

// Tell the compiler to link to either panic_abort or panic_unwind
#![needs_panic_runtime]

// std may use features in a platform-specific way
#![allow(unused_features)]

#![cfg_attr(test, feature(print_internals, set_stdio, update_panic_count))]
#![cfg_attr(all(target_vendor = "fortanix", target_env = "sgx"),
            feature(slice_index_methods, coerce_unsized,
                    sgx_platform, ptr_wrapping_offset_from))]
#![cfg_attr(all(test, target_vendor = "fortanix", target_env = "sgx"),
            feature(fixed_size_array, maybe_uninit_extra))]

// std is implemented with unstable features, many of which are internal
// compiler details that will never be stable
// NB: the following list is sorted to minimize merge conflicts.
#![feature(alloc_error_handler)]
#![feature(alloc_layout_extra)]
#![feature(allocator_api)]
#![feature(allocator_internals)]
#![feature(allow_internal_unsafe)]
#![feature(allow_internal_unstable)]
#![feature(arbitrary_self_types)]
#![feature(array_error_internals)]
#![feature(asm)]
#![feature(associated_type_bounds)]
#![feature(box_syntax)]
#![feature(c_variadic)]
#![feature(cfg_target_has_atomic)]
#![feature(cfg_target_thread_local)]
#![feature(char_error_internals)]
#![feature(clamp)]
#![feature(compiler_builtins_lib)]
#![feature(concat_idents)]
#![feature(const_cstr_unchecked)]
#![feature(const_raw_ptr_deref)]
#![feature(container_error_extra)]
#![feature(core_intrinsics)]
#![feature(custom_test_frameworks)]
#![feature(decl_macro)]
#![feature(doc_alias)]
#![feature(doc_cfg)]
#![feature(doc_keyword)]
#![feature(doc_masked)]
#![feature(doc_spotlight)]
#![feature(dropck_eyepatch)]
#![feature(duration_constants)]
#![feature(exact_size_is_empty)]
#![feature(exhaustive_patterns)]
#![feature(external_doc)]
#![feature(fn_traits)]
#![feature(format_args_nl)]
#![feature(generator_trait)]
#![feature(global_asm)]
#![feature(hash_raw_entry)]
#![feature(hashmap_internals)]
#![feature(int_error_internals)]
#![feature(int_error_matching)]
#![feature(integer_atomics)]
#![feature(lang_items)]
#![feature(libc)]
#![feature(link_args)]
#![feature(linkage)]
#![feature(log_syntax)]
#![feature(manually_drop_take)]
#![feature(matches_macro)]
#![feature(maybe_uninit_ref)]
#![feature(maybe_uninit_slice)]
#![feature(needs_panic_runtime)]
#![feature(never_type)]
#![feature(nll)]
#![cfg_attr(bootstrap, feature(non_exhaustive))]
#![feature(on_unimplemented)]
#![feature(optin_builtin_traits)]
#![feature(panic_info_message)]
#![feature(panic_internals)]
#![feature(panic_unwind)]
#![feature(prelude_import)]
#![feature(ptr_internals)]
#![feature(raw)]
#![feature(renamed_spin_loop)]
#![feature(rustc_attrs)]
#![feature(rustc_const_unstable)]
#![feature(rustc_private)]
#![feature(shrink_to)]
#![feature(slice_concat_ext)]
#![feature(slice_internals)]
#![feature(slice_patterns)]
#![feature(specialization)]
#![feature(staged_api)]
#![feature(std_internals)]
#![feature(stdsimd)]
#![feature(stmt_expr_attributes)]
#![feature(str_internals)]
#![feature(test)]
#![feature(thread_local)]
#![feature(toowned_clone_into)]
#![feature(trace_macros)]
#![feature(try_reserve)]
#![feature(unboxed_closures)]
#![feature(untagged_unions)]
#![feature(unwind_attributes)]
// NB: the above list is sorted to minimize merge conflicts.

#![default_lib_allocator]

// Explicitly import the prelude. The compiler uses this same unstable attribute
// to import the prelude implicitly when building crates that depend on std.
#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

// Access to Bencher, etc.
#[cfg(test)] extern crate test;

#[allow(unused_imports)] // macros from `alloc` are not used on all platforms
#[macro_use]
extern crate alloc as alloc_crate;
#[doc(masked)]
#[allow(unused_extern_crates)]
extern crate libc;

// We always need an unwinder currently for backtraces
#[doc(masked)]
#[allow(unused_extern_crates)]
extern crate unwind;

// Only needed for now for the `std_detect` module until that crate changes to
// use `cfg_if::cfg_if!`
#[macro_use]
#[cfg(not(test))]
extern crate cfg_if;

// During testing, this crate is not actually the "real" std library, but rather
// it links to the real std library, which was compiled from this same source
// code. So any lang items std defines are conditionally excluded (or else they
// would generate duplicate lang item errors), and any globals it defines are
// _not_ the globals used by "real" std. So this import, defined only during
// testing gives test-std access to real-std lang items and globals. See #2912
#[cfg(test)] extern crate std as realstd;

// The standard macros that are not built-in to the compiler.
#[macro_use]
mod macros;

//  Rust前导
pub mod prelude;

// 公开模块声明和重导出
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::any;
#[stable(feature = "simd_arch", since = "1.27.0")]
#[doc(no_inline)]
pub use core::arch;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cell;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::clone;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cmp;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::convert;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::default;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::hash;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::intrinsics;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::iter;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::marker;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::mem;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ops;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ptr;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::raw;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::result;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::option;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::isize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i64;
#[stable(feature = "i128", since = "1.26.0")]
pub use core::i128;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::usize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u64;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc_crate::boxed;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc_crate::rc;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc_crate::borrow;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc_crate::fmt;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc_crate::format;
#[stable(feature = "pin", since = "1.33.0")]
pub use core::pin;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc_crate::slice;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc_crate::str;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc_crate::string;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc_crate::vec;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::char;
#[stable(feature = "i128", since = "1.26.0")]
pub use core::u128;
#[stable(feature = "core_hint", since = "1.27.0")]
pub use core::hint;
#[stable(feature = "core_array", since = "1.36.0")]
pub use core::array;

pub mod f32;
pub mod f64;

#[macro_use]
pub mod thread;
pub mod ascii;
pub mod backtrace;
pub mod collections;
pub mod env;
pub mod error;
pub mod ffi;
pub mod fs;
pub mod io;
pub mod net;
pub mod num;
pub mod os;
pub mod panic;
pub mod path;
pub mod process;
pub mod sync;
pub mod time;

#[stable(feature = "futures_api", since = "1.36.0")]
pub mod task {
    //! Types and Traits for working with asynchronous tasks.
    #[doc(inline)]
    #[stable(feature = "futures_api", since = "1.36.0")]
    pub use core::task::*;
}

#[stable(feature = "futures_api", since = "1.36.0")]
pub mod future;

// Platform-abstraction modules
#[macro_use]
mod sys_common;
mod sys;

pub mod alloc;

// Private support modules
mod panicking;
mod memchr;

// The runtime entry point and a few unstable public functions used by the
// compiler
pub mod rt;

// Pull in the `std_detect` crate directly into libstd. The contents of
// `std_detect` are in a different repository: rust-lang/stdarch.
//
// `std_detect` depends on libstd, but the contents of this module are
// set up in such a way that directly pulling it here works such that the
// crate uses the this crate as its libstd.
#[path = "../stdarch/crates/std_detect/src/mod.rs"]
#[allow(missing_debug_implementations, missing_docs, dead_code)]
#[unstable(feature = "stdsimd", issue = "48556")]
#[cfg(not(test))]
mod std_detect;

#[doc(hidden)]
#[unstable(feature = "stdsimd", issue = "48556")]
#[cfg(not(test))]
pub use std_detect::detect;

// Re-export macros defined in libcore.
#[stable(feature = "rust1", since = "1.0.0")]
#[allow(deprecated, deprecated_in_future)]
pub use core::{
    // Stable
    assert_eq,
    assert_ne,
    debug_assert_eq,
    debug_assert_ne,
    debug_assert,
    r#try,
    unimplemented,
    unreachable,
    write,
    writeln,
    // Unstable
    todo,
    matches,
};

// Re-export built-in macros defined through libcore.
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
pub use core::{
    // Stable
    assert,
    cfg,
    column,
    compile_error,
    concat,
    env,
    file,
    format_args,
    include,
    include_bytes,
    include_str,
    line,
    module_path,
    option_env,
    stringify,
    // Unstable
    asm,
    concat_idents,
    format_args_nl,
    global_asm,
    log_syntax,
    trace_macros,
};

// Include a number of private modules that exist solely to provide
// the rustdoc documentation for primitive types. Using `include!`
// because rustdoc only looks for these modules at the crate level.
include!("primitive_docs.rs");

// Include a number of private modules that exist solely to provide
// the rustdoc documentation for the existing keywords. Using `include!`
// because rustdoc only looks for these modules at the crate level.
include!("keyword_docs.rs");
