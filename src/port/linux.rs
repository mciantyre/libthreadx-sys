// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright 2024 Ian McIntyre

//! The default Linux port.
//!
//! Useful for host-side testing.

pub type VOID = ::core::ffi::c_void;
pub type CHAR = ::core::ffi::c_char;
pub type UCHAR = ::core::ffi::c_uchar;
pub type UINT = ::core::ffi::c_uint;
pub type ULONG = ::core::ffi::c_uint;
pub type SHORT = ::core::ffi::c_short;
pub type USHORT = ::core::ffi::c_ushort;
pub type ULONG64 = ::core::ffi::c_ulonglong;
pub type ALIGN_TYPE = ULONG64;

pub const TX_MINIMUM_STACK: u32 = 200;
pub const TX_MAX_PRIORITIES: u32 = 32;

pub const TX_INT_DISABLE: UINT = 1;
pub const TX_INT_ENABLE: UINT = 0;

#[repr(C)]
pub(crate) struct TX_THREAD_EXTENSION_0 {
    tx_thread_linux_thread_id: libc::pthread_t,
    tx_thread_linux_thread_run_semaphore: libc::sem_t,
    tx_thread_linux_suspension_type: UINT,
    tx_thread_linux_int_disabled_flag: UINT,
}

#[repr(C)]
pub(crate) struct TX_THREAD_EXTENSION_1 {
    tx_thread_extension_ptr: *mut VOID,
}

pub(crate) type TX_THREAD_EXTENSION_2 = ();
pub(crate) type TX_THREAD_EXTENSION_3 = ();
pub(crate) type TX_THREAD_USER_EXTENSION = ();

#[repr(C)]
pub(crate) struct TX_TIMER_INTERNAL_EXTENSION {
    tx_timer_internal_extension_ptr: *mut VOID,
}

pub(crate) type TX_BYTE_POOL_EXTENSION = ();
pub(crate) type TX_EVENT_FLAGS_GROUP_EXTENSION = ();
pub(crate) type TX_SEMAPHORE_EXTENSION = ();
pub(crate) type TX_BLOCK_POOL_EXTENSION = ();
pub(crate) type TX_MUTEX_EXTENSION = ();
pub(crate) type TX_QUEUE_EXTENSION = ();
pub(crate) type TX_TIMER_EXTENSION = ();
