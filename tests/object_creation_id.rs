// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright 2024 Ian McIntyre

//! Make sure that each OS object has its ID set during creation.
//!
//! Create each OS object type, and show that creation establishes the
//! object's ID. Then, delete all of those objects. The test concludes
//! once all objects are deleted.
//!
//! We're evaluating an implementation detail of ThreadX. We'd like
//! to depend on this detail in higher-level bindings, and testing
//! here lets us catch any changes closer to the source.

use std::{cell::UnsafeCell, mem::MaybeUninit};

use libthreadx_sys::{error_checked::*, *};

struct ControlBlock<T>(UnsafeCell<MaybeUninit<T>>);
unsafe impl<T> Sync for ControlBlock<T> {}
impl<T> ControlBlock<T> {
    const fn zeroed() -> Self {
        Self(UnsafeCell::new(MaybeUninit::zeroed()))
    }
    const fn get(&self) -> *mut T {
        self.0.get().cast()
    }
}

const STACK_SIZE: usize = 512;
#[repr(align(4))]
struct Stack(ControlBlock<[u8; STACK_SIZE]>);

static THREAD: ControlBlock<TX_THREAD> = ControlBlock::zeroed();
static QUEUE: ControlBlock<TX_QUEUE> = ControlBlock::zeroed();
static SEMAPHORE: ControlBlock<TX_SEMAPHORE> = ControlBlock::zeroed();
static MUTEX: ControlBlock<TX_MUTEX> = ControlBlock::zeroed();
static BYTE_POOL: ControlBlock<TX_BYTE_POOL> = ControlBlock::zeroed();
static BLOCK_POOL: ControlBlock<TX_BLOCK_POOL> = ControlBlock::zeroed();
static TIMER: ControlBlock<TX_TIMER> = ControlBlock::zeroed();
static EVENT_FLAGS: ControlBlock<TX_EVENT_FLAGS_GROUP> = ControlBlock::zeroed();

unsafe fn delete_objects() {
    // Make sure all objects were created before we clean up.
    assert_eq!((*THREAD.get()).id(), TX_THREAD_ID);
    assert_eq!((*QUEUE.get()).id(), TX_QUEUE_ID);
    assert_eq!((*SEMAPHORE.get()).id(), TX_SEMAPHORE_ID);
    assert_eq!((*MUTEX.get()).id(), TX_MUTEX_ID);
    assert_eq!((*BYTE_POOL.get()).id(), TX_BYTE_POOL_ID);
    assert_eq!((*BLOCK_POOL.get()).id(), TX_BLOCK_POOL_ID);
    assert_eq!((*TIMER.get()).id(), TX_TIMER_ID);
    assert_eq!((*EVENT_FLAGS.get()).id(), TX_EVENT_FLAGS_ID);

    // Delete all the objects that can be deleted here.
    let mut result;

    result = tx_queue_delete(QUEUE.get());
    assert_eq!(result, TX_SUCCESS);
    assert_ne!((*QUEUE.get()).id(), TX_QUEUE_ID);

    result = tx_semaphore_delete(SEMAPHORE.get());
    assert_eq!(result, TX_SUCCESS);
    assert_ne!((*SEMAPHORE.get()).id(), TX_SEMAPHORE_ID);

    result = tx_mutex_delete(MUTEX.get());
    assert_eq!(result, TX_SUCCESS);
    assert_ne!((*MUTEX.get()).id(), TX_MUTEX_ID);

    result = tx_byte_pool_delete(BYTE_POOL.get());
    assert_eq!(result, TX_SUCCESS);
    assert_ne!((*BYTE_POOL.get()).id(), TX_BYTE_POOL_ID);

    result = tx_block_pool_delete(BLOCK_POOL.get());
    assert_eq!(result, TX_SUCCESS);
    assert_ne!((*BLOCK_POOL.get()).id(), TX_BLOCK_POOL_ID);

    result = tx_timer_delete(TIMER.get());
    assert_eq!(result, TX_SUCCESS);
    assert_ne!((*TIMER.get()).id(), TX_TIMER_ID);

    result = tx_event_flags_delete(EVENT_FLAGS.get());
    assert_eq!(result, TX_SUCCESS);
    assert_ne!((*EVENT_FLAGS.get()).id(), TX_EVENT_FLAGS_ID);

    // Spawn another thread to create the initial thread
    // (the thread on which we're currently running).
    static DELETE_THE_OTHER_THREAD: ControlBlock<TX_THREAD> = ControlBlock::zeroed();
    static STACK: Stack = Stack(ControlBlock::zeroed());

    unsafe extern "C" fn delete_the_other_thread(_: ULONG) {
        assert_eq!((*THREAD.get()).id(), TX_THREAD_ID);

        let result = tx_thread_delete(THREAD.get());
        assert_eq!(result, TX_SUCCESS);
        assert_ne!((*THREAD.get()).id(), TX_THREAD_ID);

        // If we get here, then the test has passed.
        std::process::exit(0);
    }

    // This thread has lower priority than THREAD, so it
    // won't be scheduled until THREAD is terminated.
    result = tx_thread_create(
        DELETE_THE_OTHER_THREAD.get(),
        core::ptr::null_mut(),
        Some(delete_the_other_thread),
        0,
        STACK.0.get().cast(),
        STACK_SIZE as _,
        28,
        28,
        TX_NO_TIME_SLICE,
        TX_AUTO_START,
    );
    assert_eq!(result, TX_SUCCESS);

    tx_thread_terminate(THREAD.get());
    unreachable!();
}

unsafe fn create_thread() {
    static STACK: Stack = Stack(ControlBlock::zeroed());

    unsafe extern "C" fn thread_entry(_: ULONG) {
        delete_objects()
    }

    let result = tx_thread_create(
        THREAD.get(),
        core::ptr::null_mut(),
        Some(thread_entry),
        0,
        STACK.0.get().cast(),
        STACK_SIZE as _,
        5,
        5,
        TX_NO_TIME_SLICE,
        TX_AUTO_START,
    );
    assert_eq!(result, TX_SUCCESS);
    assert_eq!((*THREAD.get()).id(), TX_THREAD_ID);
}

unsafe fn create_queue() {
    static SLOTS: ControlBlock<[u32; 64]> = ControlBlock::zeroed();

    let result = tx_queue_create(
        QUEUE.get(),
        core::ptr::null_mut(),
        1,
        SLOTS.get().cast(),
        core::mem::size_of_val(&SLOTS) as _,
    );
    assert_eq!(result, TX_SUCCESS);
    assert_eq!((*QUEUE.get()).id(), TX_QUEUE_ID);
}

unsafe fn create_semaphore() {
    let result = tx_semaphore_create(SEMAPHORE.get(), core::ptr::null_mut(), 0);
    assert_eq!(result, TX_SUCCESS);
    assert_eq!((*SEMAPHORE.get()).id(), TX_SEMAPHORE_ID);
}

unsafe fn create_mutex() {
    let result = tx_mutex_create(MUTEX.get(), core::ptr::null_mut(), TX_NO_INHERIT);
    assert_eq!(result, TX_SUCCESS);
    assert_eq!((*MUTEX.get()).id(), TX_MUTEX_ID);
}

unsafe fn create_byte_pool() {
    #[repr(align(4))]
    struct AlignedStorage(ControlBlock<[u8; 512]>);
    static STORAGE: AlignedStorage = AlignedStorage(ControlBlock::zeroed());

    let result = tx_byte_pool_create(
        BYTE_POOL.get(),
        core::ptr::null_mut(),
        STORAGE.0.get().cast(),
        512,
    );
    assert_eq!(result, TX_SUCCESS);
    assert_eq!((*BYTE_POOL.get()).id(), TX_BYTE_POOL_ID);
}

unsafe fn create_block_pool() {
    static STORAGE: ControlBlock<[u32; 512]> = ControlBlock::zeroed();

    let result = tx_block_pool_create(
        BLOCK_POOL.get(),
        core::ptr::null_mut(),
        4,
        STORAGE.get().cast(),
        core::mem::size_of_val(&STORAGE) as _,
    );
    assert_eq!(result, TX_SUCCESS);
    assert_eq!((*BLOCK_POOL.get()).id(), TX_BLOCK_POOL_ID);
}

unsafe fn create_timer() {
    unsafe extern "C" fn the_timer_doesnt_activate(_: ULONG) {
        unreachable!()
    }

    let result = tx_timer_create(
        TIMER.get(),
        core::ptr::null_mut(),
        Some(the_timer_doesnt_activate),
        0,
        !0,
        0,
        TX_NO_ACTIVATE,
    );
    assert_eq!(result, TX_SUCCESS);
    assert_eq!((*TIMER.get()).id(), TX_TIMER_ID);
}

unsafe fn create_event_flags() {
    let result = tx_event_flags_create(EVENT_FLAGS.get(), core::ptr::null_mut());
    assert_eq!(result, TX_SUCCESS);
    assert_eq!((*EVENT_FLAGS.get()).id(), TX_EVENT_FLAGS_ID);
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tx_application_define(_: *mut std::ffi::c_void) {
    create_thread();
    create_queue();
    create_semaphore();
    create_mutex();
    create_byte_pool();
    create_block_pool();
    create_timer();
    create_event_flags();
}

#[test]
fn object_creation_id() {
    unsafe { tx_kernel_enter() };
}
