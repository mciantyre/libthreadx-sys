// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright 2024 Ian McIntyre

use crate::performance_info::*;
use crate::port::*;

#[repr(C)]
pub struct TX_BLOCK_POOL {
    tx_block_pool_id: ULONG,
    tx_block_pool_name: *mut CHAR,
    tx_block_pool_available: UINT,
    tx_block_pool_total: UINT,
    tx_block_pool_available_list: *mut UCHAR,
    tx_block_pool_start: *mut UCHAR,
    tx_block_pool_size: ULONG,
    tx_block_pool_block_size: UINT,
    tx_block_pool_suspension_list: *mut TX_THREAD,
    tx_block_pool_suspended_count: UINT,
    tx_block_pool_created_next: *mut TX_BLOCK_POOL,
    tx_block_pool_created_previous: *mut TX_BLOCK_POOL,
    tx_block_pool_performance_info: TX_BLOCK_POOL_PERFORMANCE_INFO,
    tx_block_pool_extension: TX_BLOCK_POOL_EXTENSION,
    _pin: core::marker::PhantomData<core::marker::PhantomPinned>,
}

impl TX_BLOCK_POOL {
    #[inline(always)]
    pub const fn id(&self) -> ULONG {
        self.tx_block_pool_id
    }
}

#[repr(C)]
pub struct TX_BYTE_POOL {
    tx_byte_pool_id: ULONG,
    tx_byte_pool_name: *mut CHAR,
    tx_byte_pool_available: ULONG,
    tx_byte_pool_fragments: UINT,
    tx_byte_pool_list: *mut UCHAR,
    tx_byte_pool_search: *mut UCHAR,
    tx_byte_pool_start: *mut UCHAR,
    tx_byte_pool_size: ULONG,
    tx_byte_pool_owner: *mut TX_THREAD,
    tx_byte_pool_suspension_list: *mut TX_THREAD,
    tx_byte_pool_suspended_count: UINT,
    tx_byte_pool_created_next: *mut TX_BYTE_POOL,
    tx_byte_pool_created_previous: *mut TX_BYTE_POOL,
    tx_byte_pool_performance_info: TX_BYTE_POOL_PERFORMANCE_INFO,
    tx_byte_pool_extension: TX_BYTE_POOL_EXTENSION,
    _pin: core::marker::PhantomData<core::marker::PhantomPinned>,
}

impl TX_BYTE_POOL {
    #[inline(always)]
    pub const fn id(&self) -> ULONG {
        self.tx_byte_pool_id
    }
}

#[repr(C)]
pub struct TX_EVENT_FLAGS_GROUP {
    tx_event_flags_group_id: ULONG,
    tx_event_flags_group_name: *mut CHAR,
    tx_event_flags_group_current: ULONG,
    tx_event_flags_group_reset_search: UINT,
    tx_event_flags_group_suspension_list: *mut TX_THREAD,
    tx_event_flags_group_suspended_count: UINT,
    tx_event_flags_group_created_next: *mut TX_EVENT_FLAGS_GROUP,
    tx_event_flags_group_created_previous: *mut TX_EVENT_FLAGS_GROUP,
    tx_event_flags_group_delayed_clear: ULONG,
    tx_event_flags_performance_info: TX_EVENT_FLAGS_PERFORMANCE_INFO,
    #[cfg(not(tx_disable_notify_callbacks))]
    tx_event_flags_group_set_notify:
        ::core::option::Option<unsafe extern "C" fn(group_ptr: *mut TX_EVENT_FLAGS_GROUP)>,
    tx_event_flags_group_extension: TX_EVENT_FLAGS_GROUP_EXTENSION,
    _pin: core::marker::PhantomData<core::marker::PhantomPinned>,
}

impl TX_EVENT_FLAGS_GROUP {
    #[inline(always)]
    pub const fn id(&self) -> ULONG {
        self.tx_event_flags_group_id
    }
}

#[repr(C)]
pub struct TX_QUEUE {
    tx_queue_id: ULONG,
    tx_queue_name: *mut CHAR,
    tx_queue_message_size: UINT,
    tx_queue_capacity: UINT,
    tx_queue_enqueued: UINT,
    tx_queue_available_storage: UINT,
    tx_queue_start: *mut ULONG,
    tx_queue_end: *mut ULONG,
    tx_queue_read: *mut ULONG,
    tx_queue_write: *mut ULONG,
    tx_queue_suspension_list: *mut TX_THREAD,
    tx_queue_suspended_count: UINT,
    tx_queue_created_next: *mut TX_QUEUE,
    tx_queue_created_previous: *mut TX_QUEUE,
    tx_queue_performance_info: TX_QUEUE_PERFORMANCE_INFO,
    #[cfg(not(tx_disable_notify_callbacks))]
    tx_queue_send_notify: ::core::option::Option<unsafe extern "C" fn(queue_ptr: *mut TX_QUEUE)>,
    tx_queue_extension: TX_QUEUE_EXTENSION,
    _pin: core::marker::PhantomData<core::marker::PhantomPinned>,
}

impl TX_QUEUE {
    #[inline(always)]
    pub const fn id(&self) -> ULONG {
        self.tx_queue_id
    }
}

#[repr(C)]
pub struct TX_MUTEX {
    tx_mutex_id: ULONG,
    tx_mutex_name: *mut CHAR,
    tx_mutex_ownership_count: UINT,
    tx_mutex_owner: *mut TX_THREAD,
    tx_mutex_inherit: UINT,
    tx_mutex_original_priority: UINT,
    tx_mutex_suspension_list: *mut TX_THREAD,
    tx_mutex_suspended_count: UINT,
    tx_mutex_created_next: *mut TX_MUTEX,
    tx_mutex_created_previous: *mut TX_MUTEX,
    tx_mutex_highest_priority_waiting: UINT,
    tx_mutex_owned_next: *mut TX_MUTEX,
    tx_mutex_owned_previous: *mut TX_MUTEX,
    tx_mutex_performance_info: TX_MUTEX_PERFORMANCE_INFO,
    tx_mutex_extension: TX_MUTEX_EXTENSION,
    _pin: core::marker::PhantomData<core::marker::PhantomPinned>,
}

impl TX_MUTEX {
    #[inline(always)]
    pub const fn id(&self) -> ULONG {
        self.tx_mutex_id
    }
}

#[repr(C)]
pub struct TX_SEMAPHORE {
    tx_semaphore_id: ULONG,
    tx_semaphore_name: *mut CHAR,
    tx_semaphore_count: ULONG,
    tx_semaphore_suspension_list: *mut TX_THREAD,
    tx_semaphore_suspended_count: UINT,
    tx_semaphore_created_next: *mut TX_SEMAPHORE,
    tx_semaphore_created_previous: *mut TX_SEMAPHORE,
    tx_semaphore_performance_info: TX_SEMAPHORE_PERFORMANCE_INFO,
    #[cfg(not(tx_disable_notify_callbacks))]
    tx_semaphore_put_notify:
        ::core::option::Option<unsafe extern "C" fn(semaphore_ptr: *mut TX_SEMAPHORE)>,
    tx_semaphore_extension: TX_SEMAPHORE_EXTENSION,
    _pin: core::marker::PhantomData<core::marker::PhantomPinned>,
}

impl TX_SEMAPHORE {
    #[inline(always)]
    pub const fn id(&self) -> ULONG {
        self.tx_semaphore_id
    }
}

#[repr(C)]
pub struct TX_THREAD {
    tx_thread_id: ULONG,
    tx_thread_run_count: ULONG,
    tx_thread_stack_ptr: *mut ::core::ffi::c_void,
    tx_thread_stack_start: *mut ::core::ffi::c_void,
    tx_thread_stack_end: *mut ::core::ffi::c_void,
    tx_thread_stack_size: ULONG,
    tx_thread_time_slice: ULONG,
    tx_thread_new_time_slice: ULONG,
    tx_thread_ready_next: *mut TX_THREAD,
    tx_thread_ready_previous: *mut TX_THREAD,
    tx_thread_extension_0: TX_THREAD_EXTENSION_0,
    tx_thread_name: *mut CHAR,
    tx_thread_priority: UINT,
    tx_thread_state: UINT,
    tx_thread_delayed_suspend: UINT,
    tx_thread_suspending: UINT,
    tx_thread_preempt_threshold: UINT,
    tx_thread_schedule_hook:
        ::core::option::Option<unsafe extern "C" fn(thread_ptr: *mut TX_THREAD, id: ULONG)>,
    tx_thread_entry: ::core::option::Option<unsafe extern "C" fn(id: ULONG)>,
    tx_thread_entry_parameter: ULONG,
    tx_thread_timer: TX_TIMER_INTERNAL,
    tx_thread_suspend_cleanup: ::core::option::Option<
        unsafe extern "C" fn(thread_ptr: *mut TX_THREAD, suspension_sequence: ULONG),
    >,
    tx_thread_suspend_control_block: *mut ::core::ffi::c_void,
    tx_thread_suspended_next: *mut TX_THREAD,
    tx_thread_suspended_previous: *mut TX_THREAD,
    tx_thread_suspend_info: ULONG,
    tx_thread_additional_suspend_info: *mut ::core::ffi::c_void,
    tx_thread_suspend_option: UINT,
    tx_thread_suspend_status: UINT,
    tx_thread_extension_1: TX_THREAD_EXTENSION_1,
    tx_thread_created_next: *mut TX_THREAD,
    tx_thread_created_previous: *mut TX_THREAD,
    tx_thread_extension_2: TX_THREAD_EXTENSION_2,
    tx_thread_filex_ptr: *mut ::core::ffi::c_void,
    tx_thread_user_priority: UINT,
    tx_thread_user_preempt_threshold: UINT,
    tx_thread_inherit_priority: UINT,
    tx_thread_owned_mutex_count: UINT,
    tx_thread_owned_mutex_list: *mut TX_MUTEX,
    tx_thread_performance_info: TX_THREAD_PERFORMANCE_INFO,
    tx_thread_stack_highest_ptr: *mut ::core::ffi::c_void,
    #[cfg(not(tx_disable_notify_callbacks))]
    tx_thread_entry_exit_notify:
        ::core::option::Option<unsafe extern "C" fn(thread_ptr: *mut TX_THREAD, type_: UINT)>,
    tx_thread_extension_3: TX_THREAD_EXTENSION_3,
    tx_thread_suspension_sequence: ULONG,
    tx_thread_user_extension: TX_THREAD_USER_EXTENSION,
    _pin: core::marker::PhantomData<core::marker::PhantomPinned>,
}

impl TX_THREAD {
    #[inline(always)]
    pub const fn id(&self) -> ULONG {
        self.tx_thread_id
    }

    #[inline(always)]
    pub const fn stack_start(&self) -> *mut ::core::ffi::c_void {
        self.tx_thread_stack_start
    }
}

#[repr(C)]
struct TX_TIMER_INTERNAL {
    tx_timer_internal_remaining_ticks: ULONG,
    tx_timer_internal_re_initialize_ticks: ULONG,
    tx_timer_internal_timeout_function: ::core::option::Option<unsafe extern "C" fn(id: ULONG)>,
    tx_timer_internal_timeout_param: ULONG,
    tx_timer_internal_active_next: *mut TX_TIMER_INTERNAL,
    tx_timer_internal_active_previous: *mut TX_TIMER_INTERNAL,
    tx_timer_internal_list_head: *mut *mut TX_TIMER_INTERNAL,
    tx_timer_internal_extension: TX_TIMER_INTERNAL_EXTENSION,
}

#[repr(C)]
pub struct TX_TIMER {
    tx_timer_id: ULONG,
    tx_timer_name: *mut CHAR,
    tx_timer_internal: TX_TIMER_INTERNAL,
    tx_timer_created_next: *mut TX_TIMER,
    tx_timer_created_previous: *mut TX_TIMER,
    tx_timer_extension: TX_TIMER_EXTENSION,
    tx_timer_performance_info: TX_TIMER_PERFORMANCE_INFO,
    _pin: core::marker::PhantomData<core::marker::PhantomPinned>,
}

impl TX_TIMER {
    #[inline(always)]
    pub const fn id(&self) -> ULONG {
        self.tx_timer_id
    }
}
