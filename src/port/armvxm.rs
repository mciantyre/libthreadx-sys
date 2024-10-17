// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright 2024 Ian McIntyre

//! The default ThreadX port all Cortex-M ports.
//!
//! Currently, it supports ARMv6-M and ARMv7-M, It also suports
//! ARMv8-M in secure-only mode.

pub type VOID = ::core::ffi::c_void;
pub type CHAR = ::core::ffi::c_char;
pub type UCHAR = ::core::ffi::c_uchar;
pub type INT = ::core::ffi::c_int;
pub type UINT = ::core::ffi::c_uint;
pub type LONG = ::core::ffi::c_long;
pub type ULONG = ::core::ffi::c_ulong;
pub type ULONG64 = ::core::ffi::c_ulonglong;
pub type SHORT = ::core::ffi::c_short;
pub type USHORT = ::core::ffi::c_ushort;
pub type ALIGN_TYPE = ULONG;

pub const TX_MAX_PRIORITIES: u32 = 32;
pub const TX_MINIMUM_STACK: u32 = 200;

pub const TX_INT_DISABLE: UINT = 1;
pub const TX_INT_ENABLE: UINT = 0;

pub(crate) type TX_THREAD_EXTENSION_0 = ();
pub(crate) type TX_THREAD_EXTENSION_1 = ();
/// If building with split secure and non-secure support
/// for ARMv8-M, this needs to change.
pub(crate) type TX_THREAD_EXTENSION_2 = ();
pub(crate) type TX_THREAD_EXTENSION_3 = ();
pub(crate) type TX_THREAD_USER_EXTENSION = ();

pub(crate) type TX_BYTE_POOL_EXTENSION = ();
pub(crate) type TX_TIMER_INTERNAL_EXTENSION = ();
pub(crate) type TX_EVENT_FLAGS_GROUP_EXTENSION = ();
pub(crate) type TX_SEMAPHORE_EXTENSION = ();
pub(crate) type TX_BLOCK_POOL_EXTENSION = ();
pub(crate) type TX_MUTEX_EXTENSION = ();
pub(crate) type TX_QUEUE_EXTENSION = ();
pub(crate) type TX_TIMER_EXTENSION = ();
