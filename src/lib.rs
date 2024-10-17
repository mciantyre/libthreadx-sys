// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright 2024 Ian McIntyre

//! Bindings for the ThreadX RTOS.
//!
//! For build and configuration instructions, see the project README.
//! This documentation describes the API and usage.
//!
//! The package root exposes control blocks for ThreadX operating
//! system resources. All members are private, although the API may
//! include accessors for certain members. Consider constructing these
//! control blocks within a [`MaybeUninit`](core::mem::MaybeUninit)
//! wrapper. It is safe to zero initialize all members.
//!
//! The package root also exposes public constants and type aliases.
//! Some of these may change value depending on the build target.
//!
//! The [`error_checked`] and [`unchecked`] modules export ThreadX
//! functions, or "services" in ThreadX parlance. Each module presents
//! the same API; however, depending on the module, the functions link
//! against different ThreadX symbols. In some cases, the functions
//! are implemented in Rust as stub functions.
//!
//! For convenience, each function includes its official ThreadX API
//! documentation. Consult the [official ThreadX
//! documentation][rtos-docs] for even more information.
//!
//! [rtos-docs]: https://github.com/eclipse-threadx/rtos-docs

#![no_std]
#![allow(
    // Follow ThreadX naming conventions.
    non_camel_case_types,
    // Safety docs aren't available in the standard
    // documentation. Since it's a call into a C
    // library, it's understood to have arbitrary
    // unsafety.
    clippy::missing_safety_doc,
)]

mod port {
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    mod armvxm;
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    pub use armvxm::*;

    #[cfg(unix)]
    mod linux;
    #[cfg(unix)]
    pub use linux::*;

    #[cfg(windows)]
    mod win32;
    #[cfg(windows)]
    pub use win32::*;
}
pub use port::*;

mod control_blocks;
pub use control_blocks::*;

pub mod error_checked;
pub mod unchecked;

mod performance_info;

//
// API arguments.
//

pub const TX_NO_WAIT: ULONG = 0;
pub const TX_WAIT_FOREVER: ULONG = 0xFFFFFFFF;
pub const TX_AND: UINT = 2;
pub const TX_AND_CLEAR: UINT = 3;
pub const TX_OR: UINT = 0;
pub const TX_OR_CLEAR: UINT = 1;
pub const TX_NO_TIME_SLICE: ULONG = 0;
pub const TX_AUTO_START: UINT = 1;
pub const TX_DONT_START: UINT = 0;
pub const TX_AUTO_ACTIVATE: UINT = 1;
pub const TX_NO_ACTIVATE: UINT = 0;
pub const TX_TRUE: UINT = 1;
pub const TX_FALSE: UINT = 0;
pub const TX_INHERIT: UINT = 1;
pub const TX_NO_INHERIT: UINT = 0;
pub const TX_THREAD_ENTRY: UINT = 0;
pub const TX_THREAD_EXIT: UINT = 1;
pub const TX_NO_SUSPENSIONS: UINT = 0;

//
// Thread states.
//

pub const TX_READY: UINT = 0;
pub const TX_COMPLETED: UINT = 1;
pub const TX_TERMINATED: UINT = 2;
pub const TX_SUSPENDED: UINT = 3;
pub const TX_SLEEP: UINT = 4;
pub const TX_QUEUE_SUSP: UINT = 5;
pub const TX_SEMAPHORE_SUSP: UINT = 6;
pub const TX_EVENT_FLAG: UINT = 7;
pub const TX_BLOCK_MEMORY: UINT = 8;
pub const TX_BYTE_MEMORY: UINT = 9;
pub const TX_IO_DRIVER: UINT = 10;
pub const TX_FILE: UINT = 11;
pub const TX_TCP_IP: UINT = 12;
pub const TX_MUTEX_SUSP: UINT = 13;
pub const TX_PRIORITY_CHANGE: UINT = 14;

//
// API return codes.
//

pub const TX_SUCCESS: UINT = 0x00;
pub const TX_DELETED: UINT = 0x01;
pub const TX_POOL_ERROR: UINT = 0x02;
pub const TX_PTR_ERROR: UINT = 0x03;
pub const TX_WAIT_ERROR: UINT = 0x04;
pub const TX_SIZE_ERROR: UINT = 0x05;
pub const TX_GROUP_ERROR: UINT = 0x06;
pub const TX_NO_EVENTS: UINT = 0x07;
pub const TX_OPTION_ERROR: UINT = 0x08;
pub const TX_QUEUE_ERROR: UINT = 0x09;
pub const TX_QUEUE_EMPTY: UINT = 0x0A;
pub const TX_QUEUE_FULL: UINT = 0x0B;
pub const TX_SEMAPHORE_ERROR: UINT = 0x0C;
pub const TX_NO_INSTANCE: UINT = 0x0D;
pub const TX_THREAD_ERROR: UINT = 0x0E;
pub const TX_PRIORITY_ERROR: UINT = 0x0F;
pub const TX_NO_MEMORY: UINT = 0x10;
pub const TX_START_ERROR: UINT = 0x10;
pub const TX_DELETE_ERROR: UINT = 0x11;
pub const TX_RESUME_ERROR: UINT = 0x12;
pub const TX_CALLER_ERROR: UINT = 0x13;
pub const TX_SUSPEND_ERROR: UINT = 0x14;
pub const TX_TIMER_ERROR: UINT = 0x15;
pub const TX_TICK_ERROR: UINT = 0x16;
pub const TX_ACTIVATE_ERROR: UINT = 0x17;
pub const TX_THRESH_ERROR: UINT = 0x18;
pub const TX_SUSPEND_LIFTED: UINT = 0x19;
pub const TX_WAIT_ABORTED: UINT = 0x1A;
pub const TX_WAIT_ABORT_ERROR: UINT = 0x1B;
pub const TX_MUTEX_ERROR: UINT = 0x1C;
pub const TX_NOT_AVAILABLE: UINT = 0x1D;
pub const TX_NOT_OWNED: UINT = 0x1E;
pub const TX_INHERIT_ERROR: UINT = 0x1F;
pub const TX_NOT_DONE: UINT = 0x20;
pub const TX_CEILING_EXCEEDED: UINT = 0x21;
pub const TX_INVALID_CEILING: UINT = 0x22;
pub const TX_FEATURE_NOT_ENABLED: UINT = 0xFF;

/// Services without an error-checked variant.
///
/// These are re-exported through the other service modules.
mod common_services {
    use crate::port::{UINT, ULONG};

    /// Enter the ThreadX kernel.
    ///
    /// After initializing internal state, this calls `tx_application_define`, then enters
    /// the thread scheduling loop. The scheduling loop never returns.
    ///
    /// Once you enter the kernel, you shouldn't re-enter the kernel.
    #[inline(always)]
    pub unsafe extern "C" fn tx_kernel_enter() -> ! {
        unsafe extern "C" {
            fn _tx_initialize_kernel_enter();
        }

        // Safety: Chapter 2 of the ThreadX documentation
        // states that tx_kernel_enter never returns. The
        // function is defined as _tx_initialize_kernel_enter
        // no matter the build. _tx_initialize_kernel_enter is
        // also documented as never returning.
        unsafe {
            _tx_initialize_kernel_enter();
            core::hint::unreachable_unchecked()
        };
    }

    unsafe extern "C" {
        #[doc = include_str!("doc/tx_interrupt_control.md")]
        #[link_name = "_tx_thread_interrupt_control"]
        pub fn tx_interrupt_control(new_posture: UINT) -> UINT;

        #[doc = include_str!("doc/tx_time_get.md")]
        #[link_name = "_tx_time_get"]
        pub fn tx_time_get() -> ULONG;

        #[doc = include_str!("doc/tx_time_set.md")]
        #[link_name = "_tx_time_set"]
        pub fn tx_time_set(new_time: ULONG);
    }
}

//
// Implementation details - control block constants.
//

pub const TX_BLOCK_POOL_ID: ULONG = 0x424C4F43;
pub const TX_BYTE_POOL_ID: ULONG = 0x42595445;
pub const TX_EVENT_FLAGS_ID: ULONG = 0x4456444E;
pub const TX_MUTEX_ID: ULONG = 0x4D555445;
pub const TX_QUEUE_ID: ULONG = 0x51554555;
pub const TX_SEMAPHORE_ID: ULONG = 0x53454D41;
pub const TX_THREAD_ID: ULONG = 0x54485244;
pub const TX_TIMER_ID: ULONG = 0x4154494D;
