// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright 2024 Ian McIntyre

//! An approximate line-for-line port of the demo_threadx.c sample application.
//!
//! # Getting started
//!
//! If your host system is Linux or Windows, you should be able to run this example
//! as-is:
//!
//! ```bash
//! cargo run --example=demo_threadx
//! ```
//!
//! This isn't a pleasant way to use these bindings! You should seek out, or
//! build, a safer API.

#![cfg_attr(any(all(target_os = "none", target_arch = "arm"),), no_std)]
#![cfg_attr(any(all(target_os = "none", target_arch = "arm"),), no_main)]

use core::{cell::UnsafeCell, mem::MaybeUninit};
use portable_atomic::{AtomicU32, Ordering};

use libthreadx_sys::{error_checked::*, *};

struct ControlBlock<T>(UnsafeCell<MaybeUninit<T>>);
unsafe impl<T> Sync for ControlBlock<T> {}
impl<T> ControlBlock<T> {
    const fn uninit() -> Self {
        Self(UnsafeCell::new(MaybeUninit::uninit()))
    }
    const fn get(&self) -> *mut T {
        self.0.get().cast()
    }
}

const DEMO_STACK_SIZE: ULONG = 1024;
const DEMO_BYTE_POOL_SIZE: ULONG = 9120;
const DEMO_BLOCK_POOL_SIZE: ULONG = 100;
const DEMO_QUEUE_SIZE: ULONG = 100;

/* Define the ThreadX object control blocks...  */

static THREAD_0: ControlBlock<TX_THREAD> = ControlBlock::uninit();
static THREAD_1: ControlBlock<TX_THREAD> = ControlBlock::uninit();
static THREAD_2: ControlBlock<TX_THREAD> = ControlBlock::uninit();
static THREAD_3: ControlBlock<TX_THREAD> = ControlBlock::uninit();
static THREAD_4: ControlBlock<TX_THREAD> = ControlBlock::uninit();
static THREAD_5: ControlBlock<TX_THREAD> = ControlBlock::uninit();
static THREAD_6: ControlBlock<TX_THREAD> = ControlBlock::uninit();
static THREAD_7: ControlBlock<TX_THREAD> = ControlBlock::uninit();
static QUEUE_0: ControlBlock<TX_QUEUE> = ControlBlock::uninit();
static SEMAPHORE_0: ControlBlock<TX_SEMAPHORE> = ControlBlock::uninit();
static MUTEX_0: ControlBlock<TX_MUTEX> = ControlBlock::uninit();
static EVENT_FLAGS_0: ControlBlock<TX_EVENT_FLAGS_GROUP> = ControlBlock::uninit();
static BYTE_POOL_0: ControlBlock<TX_BYTE_POOL> = ControlBlock::uninit();
static BLOCK_POOL_0: ControlBlock<TX_BLOCK_POOL> = ControlBlock::uninit();

/* Define the counters used in the demo application...  */

static THREAD_0_COUNTER: AtomicU32 = AtomicU32::new(0);
static THREAD_1_COUNTER: AtomicU32 = AtomicU32::new(0);
static THREAD_1_MESSAGES_SENT: AtomicU32 = AtomicU32::new(0);
static THREAD_2_COUNTER: AtomicU32 = AtomicU32::new(0);
static THREAD_2_MESSAGES_RECEIVED: AtomicU32 = AtomicU32::new(0);
static THREAD_3_COUNTER: AtomicU32 = AtomicU32::new(0);
static THREAD_4_COUNTER: AtomicU32 = AtomicU32::new(0);
static THREAD_5_COUNTER: AtomicU32 = AtomicU32::new(0);
static THREAD_6_COUNTER: AtomicU32 = AtomicU32::new(0);
static THREAD_7_COUNTER: AtomicU32 = AtomicU32::new(0);

/* Define main entry point.  */
#[cfg_attr(all(target_os = "none", target_arch = "arm"), cortex_m_rt::entry)]
fn main() -> ! {
    unsafe { tx_kernel_enter() }
}

/* Define what the initial system looks like.  */

#[unsafe(no_mangle)]
unsafe extern "C" fn tx_application_define(mut _first_unused_memory: *mut VOID) {
    let mut result: UINT;
    let mut pointer: *mut VOID = core::ptr::null_mut();

    #[cfg(all(target_os = "none", target_arch = "arm"))]
    {
        static MEMORY_AREA: ControlBlock<[UCHAR; DEMO_BYTE_POOL_SIZE as _]> =
            ControlBlock::uninit();
        _first_unused_memory = MEMORY_AREA.get().cast();
    }

    /* Create a byte memory pool from which to allocate the thread stacks.  */
    result = tx_byte_pool_create(
        BYTE_POOL_0.get(),
        c"byte pool 0".as_ptr().cast_mut(),
        _first_unused_memory,
        DEMO_BYTE_POOL_SIZE,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Allocate the stack for thread 0.  */
    result = tx_byte_allocate(BYTE_POOL_0.get(), &mut pointer, DEMO_STACK_SIZE, TX_NO_WAIT);
    assert_eq!(result, TX_SUCCESS);

    /* Create the main thread.  */
    result = tx_thread_create(
        THREAD_0.get(),
        c"thread 0".as_ptr().cast_mut(),
        Some(thread_0_entry),
        0,
        pointer,
        DEMO_STACK_SIZE,
        1,
        1,
        TX_NO_TIME_SLICE,
        TX_AUTO_START,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Allocate the stack for thread 1.  */
    result = tx_byte_allocate(BYTE_POOL_0.get(), &mut pointer, DEMO_STACK_SIZE, TX_NO_WAIT);
    assert_eq!(result, TX_SUCCESS);

    /* Create threads 1 and 2. These threads pass information through a ThreadX
    message queue.  It is also interesting to note that these threads have a time
    slice.  */
    result = tx_thread_create(
        THREAD_1.get(),
        c"thread 1".as_ptr().cast_mut(),
        Some(thread_1_entry),
        1,
        pointer,
        DEMO_STACK_SIZE,
        16,
        16,
        4,
        TX_AUTO_START,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Allocate the stack for thread 2.  */
    result = tx_byte_allocate(BYTE_POOL_0.get(), &mut pointer, DEMO_STACK_SIZE, TX_NO_WAIT);
    assert_eq!(result, TX_SUCCESS);

    result = tx_thread_create(
        THREAD_2.get(),
        c"thread 2".as_ptr().cast_mut(),
        Some(thread_2_entry),
        2,
        pointer,
        DEMO_STACK_SIZE,
        16,
        16,
        4,
        TX_AUTO_START,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Allocate the stack for thread 3.  */
    result = tx_byte_allocate(BYTE_POOL_0.get(), &mut pointer, DEMO_STACK_SIZE, TX_NO_WAIT);
    assert_eq!(result, TX_SUCCESS);

    /* Create threads 3 and 4.  These threads compete for a ThreadX counting semaphore.
    An interesting thing here is that both threads share the same instruction area.  */
    result = tx_thread_create(
        THREAD_3.get(),
        c"thread 3".as_ptr().cast_mut(),
        Some(thread_3_and_4_entry),
        3,
        pointer,
        DEMO_STACK_SIZE,
        8,
        8,
        TX_NO_TIME_SLICE,
        TX_AUTO_START,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Allocate the stack for thread 4.  */
    result = tx_byte_allocate(BYTE_POOL_0.get(), &mut pointer, DEMO_STACK_SIZE, TX_NO_WAIT);
    assert_eq!(result, TX_SUCCESS);

    result = tx_thread_create(
        THREAD_4.get(),
        c"thread 4".as_ptr().cast_mut(),
        Some(thread_3_and_4_entry),
        4,
        pointer,
        DEMO_STACK_SIZE,
        8,
        8,
        TX_NO_TIME_SLICE,
        TX_AUTO_START,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Allocate the stack for thread 5.  */
    result = tx_byte_allocate(BYTE_POOL_0.get(), &mut pointer, DEMO_STACK_SIZE, TX_NO_WAIT);
    assert_eq!(result, TX_SUCCESS);

    /* Create thread 5.  This thread simply pends on an event flag which will be set
    by thread_0.  */
    result = tx_thread_create(
        THREAD_5.get(),
        c"thread 5".as_ptr().cast_mut(),
        Some(thread_5_entry),
        5,
        pointer,
        DEMO_STACK_SIZE,
        4,
        4,
        TX_NO_TIME_SLICE,
        TX_AUTO_START,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Allocate the stack for thread 6.  */
    result = tx_byte_allocate(BYTE_POOL_0.get(), &mut pointer, DEMO_STACK_SIZE, TX_NO_WAIT);
    assert_eq!(result, TX_SUCCESS);

    /* Create threads 6 and 7.  These threads compete for a ThreadX mutex.  */
    result = tx_thread_create(
        THREAD_6.get(),
        c"thread 6".as_ptr().cast_mut(),
        Some(thread_6_and_7_entry),
        6,
        pointer,
        DEMO_STACK_SIZE,
        8,
        8,
        TX_NO_TIME_SLICE,
        TX_AUTO_START,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Allocate the stack for thread 7.  */
    result = tx_byte_allocate(BYTE_POOL_0.get(), &mut pointer, DEMO_STACK_SIZE, TX_NO_WAIT);
    assert_eq!(result, TX_SUCCESS);

    result = tx_thread_create(
        THREAD_7.get(),
        c"thread 7".as_ptr().cast_mut(),
        Some(thread_6_and_7_entry),
        7,
        pointer,
        DEMO_STACK_SIZE,
        8,
        8,
        TX_NO_TIME_SLICE,
        TX_AUTO_START,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Allocate the message queue.  */
    result = tx_byte_allocate(
        BYTE_POOL_0.get(),
        &mut pointer,
        DEMO_QUEUE_SIZE * core::mem::size_of::<ULONG>() as u32,
        TX_NO_WAIT,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Create the message queue shared by threads 1 and 2.  */
    result = tx_queue_create(
        QUEUE_0.get(),
        c"queue 0".as_ptr().cast_mut(),
        1,
        pointer,
        DEMO_QUEUE_SIZE * core::mem::size_of::<ULONG>() as u32,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Create the semaphore used by threads 3 and 4.  */
    result = tx_semaphore_create(SEMAPHORE_0.get(), c"semaphore 0".as_ptr().cast_mut(), 1);
    assert_eq!(result, TX_SUCCESS);

    /* Create the event flags group used by threads 1 and 5.  */
    result = tx_event_flags_create(EVENT_FLAGS_0.get(), c"event flags 0".as_ptr().cast_mut());
    assert_eq!(result, TX_SUCCESS);

    /* Create the mutex used by thread 6 and 7 without priority inheritance.  */
    result = tx_mutex_create(MUTEX_0.get(), c"mutex 0".as_ptr().cast_mut(), TX_NO_INHERIT);
    assert_eq!(result, TX_SUCCESS);

    /* Allocate the memory for a small block pool.  */
    result = tx_byte_allocate(
        BYTE_POOL_0.get(),
        &mut pointer,
        DEMO_BLOCK_POOL_SIZE,
        TX_NO_WAIT,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Create a block memory pool to allocate a message buffer from.  */
    result = tx_block_pool_create(
        BLOCK_POOL_0.get(),
        c"block pool 0".as_ptr().cast_mut(),
        core::mem::size_of::<ULONG>() as _,
        pointer,
        DEMO_BLOCK_POOL_SIZE,
    );
    assert_eq!(result, TX_SUCCESS);

    /* Allocate a block and release the block memory.  */
    result = tx_block_allocate(BLOCK_POOL_0.get(), &mut pointer, TX_NO_WAIT);
    assert_eq!(result, TX_SUCCESS);

    /* Release the block back to the pool.  */
    result = tx_block_release(pointer);
    assert_eq!(result, TX_SUCCESS);
}

/* Define the test threads.  */

unsafe extern "C" fn thread_0_entry(_: ULONG) {
    static COUNTERS: &[&AtomicU32] = &[
        &THREAD_0_COUNTER,
        &THREAD_1_COUNTER,
        &THREAD_2_COUNTER,
        &THREAD_3_COUNTER,
        &THREAD_4_COUNTER,
        &THREAD_5_COUNTER,
        &THREAD_6_COUNTER,
        &THREAD_7_COUNTER,
    ];

    /* This thread simply sits in while-forever-sleep loop.  */
    loop {
        /* Increment the thread counter.  */
        THREAD_0_COUNTER.fetch_add(1, Ordering::Relaxed);

        /* Print results.  */
        for (idx, counter) in COUNTERS.iter().enumerate() {
            println!(
                "thread {} events sent: {}",
                idx,
                counter.load(Ordering::Relaxed)
            );
        }
        println!();

        /* Sleep for 10 ticks.  */
        tx_thread_sleep(10);

        /* Set event flag 0 to wakeup thread 5.  */
        let status = tx_event_flags_set(EVENT_FLAGS_0.get(), 0x1, TX_OR);

        /* Check status.  */
        assert_eq!(status, TX_SUCCESS);

        if THREAD_0_COUNTER.load(Ordering::Relaxed) > 30 {
            break;
        }
    }

    for counter in COUNTERS {
        assert!(counter.load(Ordering::Relaxed) > 30);
    }

    exit_success();
}

unsafe extern "C" fn thread_1_entry(_: ULONG) {
    /* This thread simply sends messages to a queue shared by thread 2.  */
    loop {
        /* Increment the thread counter.  */
        THREAD_1_COUNTER.fetch_add(1, Ordering::Relaxed);

        /* Send message to queue 0.  */
        let mut msg = THREAD_1_MESSAGES_SENT.load(Ordering::Relaxed);
        let status = tx_queue_send(QUEUE_0.get(), &mut msg as *mut u32 as _, TX_WAIT_FOREVER);

        /* Check completion status.  */
        assert_eq!(status, TX_SUCCESS);

        /* Increment the message sent.  */
        THREAD_1_MESSAGES_SENT.fetch_add(1, Ordering::Relaxed);
    }
}

unsafe extern "C" fn thread_2_entry(_: ULONG) {
    /* This thread retrieves messages placed on the queue by thread 1.  */
    loop {
        /* Increment the thread counter.  */
        THREAD_2_COUNTER.fetch_add(1, Ordering::Relaxed);

        /* Retrieve a message from the queue.  */
        let mut received_msg: ULONG = 0;
        let status = tx_queue_receive(
            QUEUE_0.get(),
            &mut received_msg as *mut ULONG as _,
            TX_WAIT_FOREVER,
        );

        /* Check completion status and make sure the message is what we
        expected.  */
        assert_eq!(status, TX_SUCCESS);
        assert_eq!(
            received_msg,
            THREAD_2_MESSAGES_RECEIVED.load(Ordering::Relaxed)
        );

        /* Otherwise, all is okay.  Increment the received message count.  */
        THREAD_2_MESSAGES_RECEIVED.fetch_add(1, Ordering::Relaxed);
    }
}

unsafe extern "C" fn thread_3_and_4_entry(thread_input: ULONG) {
    /* This function is executed from thread 3 and thread 4.  As the loop
    below shows, these function compete for ownership of semaphore_0.  */
    loop {
        /* Increment the thread counter.  */
        (if thread_input == 3 {
            &THREAD_3_COUNTER
        } else {
            &THREAD_4_COUNTER
        })
        .fetch_add(1, Ordering::Relaxed);

        /* Increment the thread counter.  */
        let status = tx_semaphore_get(SEMAPHORE_0.get(), TX_WAIT_FOREVER);

        /* Check status.  */
        assert_eq!(status, TX_SUCCESS);

        /* Sleep for 2 ticks to hold the semaphore.  */
        tx_thread_sleep(2);

        /* Release the semaphore.  */
        let status = tx_semaphore_put(SEMAPHORE_0.get());

        /* Check status.  */
        assert_eq!(status, TX_SUCCESS);
    }
}

unsafe extern "C" fn thread_5_entry(_: ULONG) {
    /* This thread simply waits for an event in a forever loop.  */
    loop {
        /* Increment the thread counter.  */
        THREAD_5_COUNTER.fetch_add(1, Ordering::Relaxed);

        /* Wait for event flag 0.  */
        let mut actual_flags = 0;
        let status = tx_event_flags_get(
            EVENT_FLAGS_0.get(),
            0x1,
            TX_OR_CLEAR,
            &mut actual_flags,
            TX_WAIT_FOREVER,
        );

        /* Check status.  */
        assert_eq!(status, TX_SUCCESS);
        assert_eq!(actual_flags, 0x1);
    }
}

unsafe extern "C" fn thread_6_and_7_entry(thread_input: ULONG) {
    /* This function is executed from thread 6 and thread 7.  As the loop
    below shows, these function compete for ownership of mutex_0.  */
    loop {
        /* Increment the thread counter.  */
        (if thread_input == 6 {
            &THREAD_6_COUNTER
        } else {
            &THREAD_7_COUNTER
        })
        .fetch_add(1, Ordering::Relaxed);

        /* Get the mutex with suspension.  */
        let status = tx_mutex_get(MUTEX_0.get(), TX_WAIT_FOREVER);

        /* Check status.  */
        assert_eq!(status, TX_SUCCESS);

        /* Get the mutex again with suspension.  This shows
        that an owning thread may retrieve the mutex it
        owns multiple times.  */
        let status = tx_mutex_get(MUTEX_0.get(), TX_WAIT_FOREVER);

        /* Check status.  */
        assert_eq!(status, TX_SUCCESS);

        /* Sleep for 2 ticks to hold the mutex.  */
        tx_thread_sleep(2);

        /* Release the mutex.  */
        let status = tx_mutex_put(MUTEX_0.get());

        /* Check status.  */
        assert_eq!(status, TX_SUCCESS);

        /* Release the mutex again.  This will actually
        release ownership since it was obtained twice.  */
        let status = tx_mutex_put(MUTEX_0.get());

        /* Check status.  */
        assert_eq!(status, TX_SUCCESS);
    }
}

#[cfg(all(target_os = "none", target_arch = "arm"))]
fn exit_success() -> ! {
    use cortex_m_semihosting::debug::*;
    exit(EXIT_SUCCESS);
    unreachable!()
}
#[cfg(not(all(target_os = "none", target_arch = "arm")))]
fn exit_success() -> ! {
    std::process::exit(0)
}

#[cfg(any(all(target_os = "none", target_arch = "arm"),))]
use panic_semihosting as _;

#[cfg(all(target_os = "none", target_arch = "arm"))]
use cortex_m_semihosting::hprintln as println;

#[cfg(all(target_os = "none", target_arch = "arm"))]
use lm3s6965 as _;

#[cfg(all(target_os = "none", target_arch = "arm"))]
mod arm_none {

    // Designed for ARMv6-M. Just enough to get the example
    // running in QEMU with a meaningless system tick.
    #[unsafe(no_mangle)]
    pub extern "C" fn _tx_initialize_low_level() {
        use cortex_m::{peripheral::scb::SystemHandler, Peripherals};

        cortex_m::interrupt::disable();

        let Peripherals {
            mut SYST, mut SCB, ..
        } = Peripherals::take().unwrap();

        SYST.set_reload(80000 - 1);
        SYST.clear_current();
        SYST.enable_interrupt();
        SYST.enable_counter();

        unsafe {
            SCB.set_priority(SystemHandler::SysTick, 0x40);
            SCB.set_priority(SystemHandler::PendSV, 0xff);
            SCB.set_priority(SystemHandler::SVCall, 0xff);
        }
    }

    use cortex_m_rt::exception;

    extern "C" {
        fn _tx_timer_interrupt();
        fn PendSV_Handler();
    }

    #[exception]
    unsafe fn SysTick() {
        _tx_timer_interrupt()
    }

    #[exception]
    unsafe fn PendSV() {
        PendSV_Handler()
    }
}
