// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright 2024 Ian McIntyre

//! ThreadX services without error checking.
//!
//! Unless you know what you're doing, you should use the functions exposed in
//! the [`error_checked`](crate::error_checked).
//!
//! These functions link against their `_tx_*` counterparts. If you're a C user
//! and you define `TX_DISABLE_ERROR_CHECKING`, then these are the functions
//! you're using.

use crate::{control_blocks::*, port::*};

pub use crate::common_services::*;

//
// Block pool
//

unsafe extern "C" {
    #[doc = include_str!("doc/tx_block_allocate.md")]
    #[link_name = "_tx_block_allocate"]
    pub fn tx_block_allocate(
        pool_ptr: *mut TX_BLOCK_POOL,
        block_ptr: *mut *mut VOID,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_block_pool_delete.md")]
    #[link_name = "_tx_block_pool_delete"]
    pub fn tx_block_pool_delete(pool_ptr: *mut TX_BLOCK_POOL) -> UINT;

    #[doc = include_str!("doc/tx_block_pool_info_get.md")]
    #[link_name = "_tx_block_pool_info_get"]
    pub fn tx_block_pool_info_get(
        pool_ptr: *mut TX_BLOCK_POOL,
        name: *mut *mut CHAR,
        available_blocks: *mut ULONG,
        total_blocks: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_pool: *mut *mut TX_BLOCK_POOL,
    ) -> UINT;

    #[doc = include_str!("doc/tx_block_pool_prioritize.md")]
    #[link_name = "_tx_block_pool_prioritize"]
    pub fn tx_block_pool_prioritize(pool_ptr: *mut TX_BLOCK_POOL) -> UINT;

    #[doc = include_str!("doc/tx_block_release.md")]
    #[link_name = "_tx_block_release"]
    pub fn tx_block_release(block_ptr: *mut VOID) -> UINT;

    #[doc = include_str!("doc/tx_block_pool_create.md")]
    #[link_name = "_tx_block_pool_create"]
    pub fn tx_block_pool_create(
        pool_ptr: *mut TX_BLOCK_POOL,
        name_ptr: *mut CHAR,
        block_size: ULONG,
        pool_start: *mut VOID,
        pool_size: ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_block_pool_performance_info_get.md")]
    #[link_name = "_tx_block_pool_performance_info_get"]
    pub fn tx_block_pool_performance_info_get(
        pool_ptr: *mut TX_BLOCK_POOL,
        allocates: *mut ULONG,
        releases: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_block_pool_performance_system_info_get.md")]
    #[link_name = "_tx_block_pool_performance_system_info_get"]
    pub fn tx_block_pool_performance_system_info_get(
        allocates: *mut ULONG,
        releases: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}

//
// Byte pool
//

unsafe extern "C" {
    #[doc = include_str!("doc/tx_byte_allocate.md")]
    #[link_name = "_tx_byte_allocate"]
    pub fn tx_byte_allocate(
        pool_ptr: *mut TX_BYTE_POOL,
        memory_ptr: *mut *mut ::core::ffi::c_void,
        memory_size: ULONG,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_byte_pool_delete.md")]
    #[link_name = "_tx_byte_pool_delete"]
    pub fn tx_byte_pool_delete(pool_ptr: *mut TX_BYTE_POOL) -> UINT;

    #[doc = include_str!("doc/tx_byte_pool_info_get.md")]
    #[link_name = "_tx_byte_pool_info_get"]
    pub fn tx_byte_pool_info_get(
        pool_ptr: *mut TX_BYTE_POOL,
        name: *mut *mut CHAR,
        available_bytes: *mut ULONG,
        fragments: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_pool: *mut *mut TX_BYTE_POOL,
    ) -> UINT;

    #[doc = include_str!("doc/tx_byte_pool_prioritize.md")]
    #[link_name = "_tx_byte_pool_prioritize"]
    pub fn tx_byte_pool_prioritize(pool_ptr: *mut TX_BYTE_POOL) -> UINT;

    #[doc = include_str!("doc/tx_byte_release.md")]
    #[link_name = "_tx_byte_release"]
    pub fn tx_byte_release(memory_ptr: *mut ::core::ffi::c_void) -> UINT;

    #[doc = include_str!("doc/tx_byte_pool_create.md")]
    #[link_name = "_tx_byte_pool_create"]
    pub fn tx_byte_pool_create(
        pool_ptr: *mut TX_BYTE_POOL,
        name_ptr: *mut CHAR,
        pool_start: *mut ::core::ffi::c_void,
        pool_size: ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_byte_pool_performance_info_get.md")]
    #[link_name = "_tx_byte_pool_performance_info_get"]
    pub fn tx_byte_pool_performance_info_get(
        pool_ptr: *mut TX_BYTE_POOL,
        allocates: *mut ULONG,
        releases: *mut ULONG,
        fragments_searched: *mut ULONG,
        merges: *mut ULONG,
        splits: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_byte_pool_performance_system_info_get.md")]
    #[link_name = "_tx_byte_pool_performance_system_info_get"]
    pub fn tx_byte_pool_performance_system_info_get(
        allocates: *mut ULONG,
        releases: *mut ULONG,
        fragments_searched: *mut ULONG,
        merges: *mut ULONG,
        splits: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}

//
// Event flags
//

unsafe extern "C" {
    #[doc = include_str!("doc/tx_event_flags_delete.md")]
    #[link_name = "_tx_event_flags_delete"]
    pub fn tx_event_flags_delete(group_ptr: *mut TX_EVENT_FLAGS_GROUP) -> UINT;

    #[doc = include_str!("doc/tx_event_flags_get.md")]
    #[link_name = "_tx_event_flags_get"]
    pub fn tx_event_flags_get(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        requested_flags: ULONG,
        get_option: UINT,
        actual_flags_ptr: *mut ULONG,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_event_flags_info_get.md")]
    #[link_name = "_tx_event_flags_info_get"]
    pub fn tx_event_flags_info_get(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        name: *mut *mut CHAR,
        current_flags: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_group: *mut *mut TX_EVENT_FLAGS_GROUP,
    ) -> UINT;

    #[doc = include_str!("doc/tx_event_flags_set.md")]
    #[link_name = "_tx_event_flags_set"]
    pub fn tx_event_flags_set(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        flags_to_set: ULONG,
        set_option: UINT,
    ) -> UINT;

    #[doc = include_str!("doc/tx_event_flags_set_notify.md")]
    #[link_name = "_tx_event_flags_set_notify"]
    pub fn tx_event_flags_set_notify(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        events_set_notify: ::core::option::Option<
            unsafe extern "C" fn(notify_group_ptr: *mut TX_EVENT_FLAGS_GROUP),
        >,
    ) -> UINT;

    #[doc = include_str!("doc/tx_event_flags_create.md")]
    #[link_name = "_tx_event_flags_create"]
    pub fn tx_event_flags_create(group_ptr: *mut TX_EVENT_FLAGS_GROUP, name_ptr: *mut CHAR)
        -> UINT;

    #[doc = include_str!("doc/tx_event_flags_performance_info_get.md")]
    #[link_name = "_tx_event_flags_performance_info_get"]
    pub fn tx_event_flags_performance_info_get(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        sets: *mut ULONG,
        gets: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_event_flags_performance_system_info_get.md")]
    #[link_name = "_tx_event_flags_performance_system_info_get"]
    pub fn tx_event_flags_performance_system_info_get(
        sets: *mut ULONG,
        gets: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}

//
// Mutex
//

unsafe extern "C" {
    #[doc = include_str!("doc/tx_mutex_delete.md")]
    #[link_name = "_tx_mutex_delete"]
    pub fn tx_mutex_delete(mutex_ptr: *mut TX_MUTEX) -> UINT;

    #[doc = include_str!("doc/tx_mutex_get.md")]
    #[link_name = "_tx_mutex_get"]
    pub fn tx_mutex_get(mutex_ptr: *mut TX_MUTEX, wait_option: ULONG) -> UINT;

    #[doc = include_str!("doc/tx_mutex_info_get.md")]
    #[link_name = "_tx_mutex_info_get"]
    pub fn tx_mutex_info_get(
        mutex_ptr: *mut TX_MUTEX,
        name: *mut *mut CHAR,
        count: *mut ULONG,
        owner: *mut *mut TX_THREAD,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_mutex: *mut *mut TX_MUTEX,
    ) -> UINT;

    #[doc = include_str!("doc/tx_mutex_prioritize.md")]
    #[link_name = "_tx_mutex_prioritize"]
    pub fn tx_mutex_prioritize(mutex_ptr: *mut TX_MUTEX) -> UINT;

    #[doc = include_str!("doc/tx_mutex_put.md")]
    #[link_name = "_tx_mutex_put"]
    pub fn tx_mutex_put(mutex_ptr: *mut TX_MUTEX) -> UINT;

    #[doc = include_str!("doc/tx_mutex_create.md")]
    #[link_name = "_tx_mutex_create"]
    pub fn tx_mutex_create(mutex_ptr: *mut TX_MUTEX, name_ptr: *mut CHAR, inherit: UINT) -> UINT;

    #[doc = include_str!("doc/tx_mutex_performance_info_get.md")]
    #[link_name = "_tx_mutex_performance_info_get"]
    pub fn tx_mutex_performance_info_get(
        mutex_ptr: *mut TX_MUTEX,
        puts: *mut ULONG,
        gets: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
        inversions: *mut ULONG,
        inheritances: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_mutex_performance_system_info_get.md")]
    #[link_name = "_tx_mutex_performance_system_info_get"]
    pub fn tx_mutex_performance_system_info_get(
        puts: *mut ULONG,
        gets: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
        inversions: *mut ULONG,
        inheritances: *mut ULONG,
    ) -> UINT;
}

//
// Queue
//

unsafe extern "C" {
    #[doc = include_str!("doc/tx_queue_delete.md")]
    #[link_name = "_tx_queue_delete"]
    pub fn tx_queue_delete(queue_ptr: *mut TX_QUEUE) -> UINT;

    #[doc = include_str!("doc/tx_queue_flush.md")]
    #[link_name = "_tx_queue_flush"]
    pub fn tx_queue_flush(queue_ptr: *mut TX_QUEUE) -> UINT;

    #[doc = include_str!("doc/tx_queue_info_get.md")]
    #[link_name = "_tx_queue_info_get"]
    pub fn tx_queue_info_get(
        queue_ptr: *mut TX_QUEUE,
        name: *mut *mut CHAR,
        enqueued: *mut ULONG,
        available_storage: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_queue: *mut *mut TX_QUEUE,
    ) -> UINT;

    #[doc = include_str!("doc/tx_queue_prioritize.md")]
    #[link_name = "_tx_queue_prioritize"]
    pub fn tx_queue_prioritize(queue_ptr: *mut TX_QUEUE) -> UINT;

    #[doc = include_str!("doc/tx_queue_receive.md")]
    #[link_name = "_tx_queue_receive"]
    pub fn tx_queue_receive(
        queue_ptr: *mut TX_QUEUE,
        destination_ptr: *mut ::core::ffi::c_void,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_queue_send.md")]
    #[link_name = "_tx_queue_send"]
    pub fn tx_queue_send(
        queue_ptr: *mut TX_QUEUE,
        source_ptr: *mut ::core::ffi::c_void,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_queue_send_notify.md")]
    #[link_name = "_tx_queue_send_notify"]
    pub fn tx_queue_send_notify(
        queue_ptr: *mut TX_QUEUE,
        queue_send_notify: ::core::option::Option<
            unsafe extern "C" fn(notify_queue_ptr: *mut TX_QUEUE),
        >,
    ) -> UINT;

    #[doc = include_str!("doc/tx_queue_front_send.md")]
    #[link_name = "_tx_queue_front_send"]
    pub fn tx_queue_front_send(
        queue_ptr: *mut TX_QUEUE,
        source_ptr: *mut ::core::ffi::c_void,
        wait_option: ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_queue_create.md")]
    #[link_name = "_tx_queue_create"]
    pub fn tx_queue_create(
        queue_ptr: *mut TX_QUEUE,
        name_ptr: *mut CHAR,
        message_size: UINT,
        queue_start: *mut ::core::ffi::c_void,
        queue_size: ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_queue_performance_info_get.md")]
    #[link_name = "_tx_queue_performance_info_get"]
    pub fn tx_queue_performance_info_get(
        queue_ptr: *mut TX_QUEUE,
        messages_sent: *mut ULONG,
        messages_received: *mut ULONG,
        empty_suspensions: *mut ULONG,
        full_suspensions: *mut ULONG,
        full_errors: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_queue_performance_system_info_get.md")]
    #[link_name = "_tx_queue_performance_system_info_get"]
    pub fn tx_queue_performance_system_info_get(
        messages_sent: *mut ULONG,
        messages_received: *mut ULONG,
        empty_suspensions: *mut ULONG,
        full_suspensions: *mut ULONG,
        full_errors: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}

//
// Semaphore
//

unsafe extern "C" {
    #[doc = include_str!("doc/tx_semaphore_ceiling_put.md")]
    #[link_name = "_tx_semaphore_ceiling_put"]
    pub fn tx_semaphore_ceiling_put(semaphore_ptr: *mut TX_SEMAPHORE, ceiling: ULONG) -> UINT;

    #[doc = include_str!("doc/tx_semaphore_delete.md")]
    #[link_name = "_tx_semaphore_delete"]
    pub fn tx_semaphore_delete(semaphore_ptr: *mut TX_SEMAPHORE) -> UINT;

    #[doc = include_str!("doc/tx_semaphore_get.md")]
    #[link_name = "_tx_semaphore_get"]
    pub fn tx_semaphore_get(semaphore_ptr: *mut TX_SEMAPHORE, wait_option: ULONG) -> UINT;

    #[doc = include_str!("doc/tx_semaphore_info_get.md")]
    #[link_name = "_tx_semaphore_info_get"]
    pub fn tx_semaphore_info_get(
        semaphore_ptr: *mut TX_SEMAPHORE,
        name: *mut *mut CHAR,
        current_value: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_semaphore: *mut *mut TX_SEMAPHORE,
    ) -> UINT;

    #[doc = include_str!("doc/tx_semaphore_prioritize.md")]
    #[link_name = "_tx_semaphore_prioritize"]
    pub fn tx_semaphore_prioritize(semaphore_ptr: *mut TX_SEMAPHORE) -> UINT;

    #[doc = include_str!("doc/tx_semaphore_put.md")]
    #[link_name = "_tx_semaphore_put"]
    pub fn tx_semaphore_put(semaphore_ptr: *mut TX_SEMAPHORE) -> UINT;

    #[doc = include_str!("doc/tx_semaphore_put_notify.md")]
    #[link_name = "_tx_semaphore_put_notify"]
    pub fn tx_semaphore_put_notify(
        semaphore_ptr: *mut TX_SEMAPHORE,
        semaphore_put_notify: ::core::option::Option<
            unsafe extern "C" fn(notify_semaphore_ptr: *mut TX_SEMAPHORE),
        >,
    ) -> UINT;

    #[doc = include_str!("doc/tx_semaphore_create.md")]
    #[link_name = "_tx_semaphore_create"]
    pub fn tx_semaphore_create(
        semaphore_ptr: *mut TX_SEMAPHORE,
        name_ptr: *mut CHAR,
        initial_count: ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_semaphore_performance_info_get.md")]
    #[link_name = "_tx_semaphore_performance_info_get"]
    pub fn tx_semaphore_performance_info_get(
        semaphore_ptr: *mut TX_SEMAPHORE,
        puts: *mut ULONG,
        gets: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_semaphore_performance_system_info_get.md")]
    #[link_name = "_tx_semaphore_performance_system_info_get"]
    pub fn tx_semaphore_performance_system_info_get(
        puts: *mut ULONG,
        gets: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}

//
// Thread
//

unsafe extern "C" {
    #[doc = include_str!("doc/tx_thread_delete.md")]
    #[link_name = "_tx_thread_delete"]
    pub fn tx_thread_delete(thread_ptr: *mut TX_THREAD) -> UINT;

    #[doc = include_str!("doc/tx_thread_entry_exit_notify.md")]
    #[link_name = "_tx_thread_entry_exit_notify"]
    pub fn tx_thread_entry_exit_notify(
        thread_ptr: *mut TX_THREAD,
        thread_entry_exit_notify: ::core::option::Option<
            unsafe extern "C" fn(notify_thread_ptr: *mut TX_THREAD, type_: UINT),
        >,
    ) -> UINT;

    #[doc = include_str!("doc/tx_thread_info_get.md")]
    #[link_name = "_tx_thread_info_get"]
    pub fn tx_thread_info_get(
        thread_ptr: *mut TX_THREAD,
        name: *mut *mut CHAR,
        state: *mut UINT,
        run_count: *mut ULONG,
        priority: *mut UINT,
        preemption_threshold: *mut UINT,
        time_slice: *mut ULONG,
        next_thread: *mut *mut TX_THREAD,
        next_suspended_thread: *mut *mut TX_THREAD,
    ) -> UINT;

    #[doc = include_str!("doc/tx_thread_preemption_change.md")]
    #[link_name = "_tx_thread_preemption_change"]
    pub fn tx_thread_preemption_change(
        thread_ptr: *mut TX_THREAD,
        new_threshold: UINT,
        old_threshold: *mut UINT,
    ) -> UINT;

    #[doc = include_str!("doc/tx_thread_priority_change.md")]
    #[link_name = "_tx_thread_priority_change"]
    pub fn tx_thread_priority_change(
        thread_ptr: *mut TX_THREAD,
        new_priority: UINT,
        old_priority: *mut UINT,
    ) -> UINT;

    #[doc = include_str!("doc/tx_thread_relinquish.md")]
    #[link_name = "_tx_thread_relinquish"]
    pub fn tx_thread_relinquish();

    #[doc = include_str!("doc/tx_thread_reset.md")]
    #[link_name = "_tx_thread_reset"]
    pub fn tx_thread_reset(thread_ptr: *mut TX_THREAD) -> UINT;

    #[doc = include_str!("doc/tx_thread_resume.md")]
    #[link_name = "_tx_thread_resume"]
    pub fn tx_thread_resume(thread_ptr: *mut TX_THREAD) -> UINT;

    #[doc = include_str!("doc/tx_thread_suspend.md")]
    #[link_name = "_tx_thread_suspend"]
    pub fn tx_thread_suspend(thread_ptr: *mut TX_THREAD) -> UINT;

    #[doc = include_str!("doc/tx_thread_terminate.md")]
    #[link_name = "_tx_thread_terminate"]
    pub fn tx_thread_terminate(thread_ptr: *mut TX_THREAD) -> UINT;

    #[doc = include_str!("doc/tx_thread_time_slice_change.md")]
    #[link_name = "_tx_thread_time_slice_change"]
    pub fn tx_thread_time_slice_change(
        thread_ptr: *mut TX_THREAD,
        new_time_slice: ULONG,
        old_time_slice: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_thread_wait_abort.md")]
    #[link_name = "_tx_thread_wait_abort"]
    pub fn tx_thread_wait_abort(thread_ptr: *mut TX_THREAD) -> UINT;

    #[doc = include_str!("doc/tx_thread_create.md")]
    #[link_name = "_tx_thread_create"]
    pub fn tx_thread_create(
        thread_ptr: *mut TX_THREAD,
        name_ptr: *mut CHAR,
        entry_function: ::core::option::Option<unsafe extern "C" fn(entry_input: ULONG)>,
        entry_input: ULONG,
        stack_start: *mut ::core::ffi::c_void,
        stack_size: ULONG,
        priority: UINT,
        preempt_threshold: UINT,
        time_slice: ULONG,
        auto_start: UINT,
    ) -> UINT;

    #[doc = include_str!("doc/tx_thread_identify.md")]
    #[link_name = "_tx_thread_identify"]
    pub fn tx_thread_identify() -> *mut TX_THREAD;

    #[doc = include_str!("doc/tx_thread_performance_info_get.md")]
    #[link_name = "_tx_thread_performance_info_get"]
    pub fn tx_thread_performance_info_get(
        thread_ptr: *mut TX_THREAD,
        resumptions: *mut ULONG,
        suspensions: *mut ULONG,
        solicited_preemptions: *mut ULONG,
        interrupt_preemptions: *mut ULONG,
        priority_inversions: *mut ULONG,
        time_slices: *mut ULONG,
        relinquishes: *mut ULONG,
        timeouts: *mut ULONG,
        wait_aborts: *mut ULONG,
        last_preempted_by: *mut *mut TX_THREAD,
    ) -> UINT;

    #[doc = include_str!("doc/tx_thread_performance_system_info_get.md")]
    #[link_name = "_tx_thread_performance_system_info_get"]
    pub fn tx_thread_performance_system_info_get(
        resumptions: *mut ULONG,
        suspensions: *mut ULONG,
        solicited_preemptions: *mut ULONG,
        interrupt_preemptions: *mut ULONG,
        priority_inversions: *mut ULONG,
        time_slices: *mut ULONG,
        relinquishes: *mut ULONG,
        timeouts: *mut ULONG,
        wait_aborts: *mut ULONG,
        non_idle_returns: *mut ULONG,
        idle_returns: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_thread_sleep.md")]
    #[link_name = "_tx_thread_sleep"]
    pub fn tx_thread_sleep(timer_ticks: ULONG) -> UINT;

    #[doc = include_str!("doc/tx_thread_stack_error_notify.md")]
    #[link_name = "_tx_thread_stack_error_notify"]
    pub fn tx_thread_stack_error_notify(
        stack_error_handler: ::core::option::Option<
            unsafe extern "C" fn(thread_ptr: *mut TX_THREAD),
        >,
    ) -> UINT;
}

//
// Timer
//

unsafe extern "C" {
    #[doc = include_str!("doc/tx_timer_activate.md")]
    #[link_name = "_tx_timer_activate"]
    pub fn tx_timer_activate(timer_ptr: *mut TX_TIMER) -> UINT;

    #[doc = include_str!("doc/tx_timer_change.md")]
    #[link_name = "_tx_timer_change"]
    pub fn tx_timer_change(
        timer_ptr: *mut TX_TIMER,
        initial_ticks: ULONG,
        reschedule_ticks: ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_timer_deactivate.md")]
    #[link_name = "_tx_timer_deactivate"]
    pub fn tx_timer_deactivate(timer_ptr: *mut TX_TIMER) -> UINT;

    #[doc = include_str!("doc/tx_timer_delete.md")]
    #[link_name = "_tx_timer_delete"]
    pub fn tx_timer_delete(timer_ptr: *mut TX_TIMER) -> UINT;

    #[doc = include_str!("doc/tx_timer_info_get.md")]
    #[link_name = "_tx_timer_info_get"]
    pub fn tx_timer_info_get(
        timer_ptr: *mut TX_TIMER,
        name: *mut *mut CHAR,
        active: *mut UINT,
        remaining_ticks: *mut ULONG,
        reschedule_ticks: *mut ULONG,
        next_timer: *mut *mut TX_TIMER,
    ) -> UINT;

    #[doc = include_str!("doc/tx_timer_create.md")]
    #[link_name = "_tx_timer_create"]
    pub fn tx_timer_create(
        timer_ptr: *mut TX_TIMER,
        name_ptr: *mut CHAR,
        expiration_function: ::core::option::Option<unsafe extern "C" fn(input: ULONG)>,
        expiration_input: ULONG,
        initial_ticks: ULONG,
        reschedule_ticks: ULONG,
        auto_activate: UINT,
    ) -> UINT;

    #[doc = include_str!("doc/tx_timer_performance_info_get.md")]
    #[link_name = "_tx_timer_performance_info_get"]
    pub fn tx_timer_performance_info_get(
        timer_ptr: *mut TX_TIMER,
        activates: *mut ULONG,
        reactivates: *mut ULONG,
        deactivates: *mut ULONG,
        expirations: *mut ULONG,
        expiration_adjusts: *mut ULONG,
    ) -> UINT;

    #[doc = include_str!("doc/tx_timer_performance_system_info_get.md")]
    #[link_name = "_tx_timer_performance_system_info_get"]
    pub fn tx_timer_performance_system_info_get(
        activates: *mut ULONG,
        reactivates: *mut ULONG,
        deactivates: *mut ULONG,
        expirations: *mut ULONG,
        expiration_adjusts: *mut ULONG,
    ) -> UINT;
}
