// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright 2024 Ian McIntyre

//! Performance info extensions for control blocks.

#![allow(
    // ThreadX conventions. Some members have multiple '_' as separators.
    non_snake_case,
)]

#[allow(unused)]
use crate::{control_blocks::*, port::*};

#[cfg(tx_timer_enable_performance_info)]
#[repr(C)]
pub(crate) struct TX_TIMER_PERFORMANCE_INFO {
    tx_timer_performance_activate_count: ULONG,
    tx_timer_performance_reactivate_count: ULONG,
    tx_timer_performance_deactivate_count: ULONG,
    tx_timer_performance_expiration_count: ULONG,
    tx_timer_performance__expiration_adjust_count: ULONG,
}

#[cfg(not(tx_timer_enable_performance_info))]
pub(crate) type TX_TIMER_PERFORMANCE_INFO = ();

#[cfg(tx_thread_enable_performance_info)]
#[repr(C)]
pub(crate) struct TX_THREAD_PERFORMANCE_INFO {
    tx_thread_performance_resume_count: ULONG,
    tx_thread_performance_suspend_count: ULONG,
    tx_thread_performance_solicited_preemption_count: ULONG,
    tx_thread_performance_interrupt_preemption_count: ULONG,
    tx_thread_performance_priority_inversion_count: ULONG,
    tx_thread_performance_last_preempting_thread: *mut TX_THREAD,
    tx_thread_performance_time_slice_count: ULONG,
    tx_thread_performance_relinquish_count: ULONG,
    tx_thread_performance_timeout_count: ULONG,
    tx_thread_performance_wait_abort_count: ULONG,
}

#[cfg(not(tx_thread_enable_performance_info))]
pub(crate) type TX_THREAD_PERFORMANCE_INFO = ();

#[cfg(tx_block_pool_enable_performance_info)]
#[repr(C)]
pub(crate) struct TX_BLOCK_POOL_PERFORMANCE_INFO {
    tx_block_pool_performance_allocate_count: ULONG,
    tx_block_pool_performance_release_count: ULONG,
    tx_block_pool_performance_suspension_count: ULONG,
    tx_block_pool_performance_timeout_count: ULONG,
}

#[cfg(not(tx_block_pool_enable_performance_info))]
pub(crate) type TX_BLOCK_POOL_PERFORMANCE_INFO = ();

#[cfg(tx_byte_pool_enable_performance_info)]
#[repr(C)]
pub(crate) struct TX_BYTE_POOL_PERFORMANCE_INFO {
    tx_byte_pool_performance_allocate_count: ULONG,
    tx_byte_pool_performance_release_count: ULONG,
    tx_byte_pool_performance_merge_count: ULONG,
    tx_byte_pool_performance_split_count: ULONG,
    tx_byte_pool_performance_search_count: ULONG,
    tx_byte_pool_performance_suspension_count: ULONG,
    tx_byte_pool_performance_timeout_count: ULONG,
}

#[cfg(not(tx_byte_pool_enable_performance_info))]
pub(crate) type TX_BYTE_POOL_PERFORMANCE_INFO = ();

#[cfg(tx_event_flags_enable_performance_info)]
#[repr(C)]
pub(crate) struct TX_EVENT_FLAGS_PERFORMANCE_INFO {
    tx_event_flags_group_performance_set_count: ULONG,
    tx_event_flags_group__performance_get_count: ULONG,
    tx_event_flags_group___performance_suspension_count: ULONG,
    tx_event_flags_group____performance_timeout_count: ULONG,
}

#[cfg(not(tx_event_flags_enable_performance_info))]
pub(crate) type TX_EVENT_FLAGS_PERFORMANCE_INFO = ();

#[cfg(tx_mutex_enable_performance_info)]
#[repr(C)]
pub(crate) struct TX_MUTEX_PERFORMANCE_INFO {
    tx_mutex_performance_put_count: ULONG,
    tx_mutex_performance_get_count: ULONG,
    tx_mutex_performance_suspension_count: ULONG,
    tx_mutex_performance_timeout_count: ULONG,
    tx_mutex_performance_priority_inversion_count: ULONG,
    tx_mutex_performance__priority_inheritance_count: ULONG,
}

#[cfg(not(tx_mutex_enable_performance_info))]
pub(crate) type TX_MUTEX_PERFORMANCE_INFO = ();

#[cfg(tx_queue_enable_performance_info)]
#[repr(C)]
pub(crate) struct TX_QUEUE_PERFORMANCE_INFO {
    tx_queue_performance_messages_sent_count: ULONG,
    tx_queue_performance_messages_received_count: ULONG,
    tx_queue_performance_empty_suspension_count: ULONG,
    tx_queue_performance_full_suspension_count: ULONG,
    tx_queue_performance_full_error_count: ULONG,
    tx_queue_performance_timeout_count: ULONG,
}

#[cfg(not(tx_queue_enable_performance_info))]
pub(crate) type TX_QUEUE_PERFORMANCE_INFO = ();

#[cfg(tx_semaphore_enable_performance_info)]
#[repr(C)]
pub(crate) struct TX_SEMAPHORE_PERFORMANCE_INFO {
    tx_semaphore_performance_put_count: ULONG,
    tx_semaphore_performance_get_count: ULONG,
    tx_semaphore_performance_suspension_count: ULONG,
    tx_semaphore_performance_timeout_count: ULONG,
}

#[cfg(not(tx_semaphore_enable_performance_info))]
pub(crate) type TX_SEMAPHORE_PERFORMANCE_INFO = ();
