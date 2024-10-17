// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright 2024 Ian McIntyre

use std::{collections::HashMap, env, fs, path::PathBuf};

type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
#[allow(dead_code)]
enum Port {
    Armv6m,
    Armv7m,
    Armv8mBase,
    Armv8mMain,
    Linux,
    Win32,
}

impl Port {
    fn source_path(&self) -> PathBuf {
        match self {
            Self::Armv6m => "threadx/ports/cortex_m0/gnu/src".into(),
            Self::Armv7m => "threadx/ports_arch/ARMv7-M/threadx/gnu/src".into(),
            Self::Armv8mBase => "threadx/ports/cortex_m23/gnu/src".into(),
            Self::Armv8mMain => "threadx/ports/cortex_m33/gnu/src".into(),
            Self::Linux => "threadx/ports/linux/gnu/src".into(),
            Self::Win32 => "threadx/ports/win32/vs_2019/src".into(),
        }
    }
    fn include_path(&self) -> PathBuf {
        match self {
            Self::Armv6m => "threadx/ports/cortex_m0/gnu/inc".into(),
            Self::Armv7m => "threadx/ports_arch/ARMv7-M/threadx/inc".into(),
            Self::Armv8mBase => "threadx/ports/cortex_m23/gnu/inc".into(),
            Self::Armv8mMain => "threadx/ports/cortex_m33/gnu/inc".into(),
            Self::Linux => "threadx/ports/linux/gnu/inc".into(),
            Self::Win32 => "threadx/ports/win32/vs_2019/inc".into(),
        }
    }
    fn set_defines(&self, bld: &mut cc::Build) {
        match self {
            Self::Armv8mBase | Self::Armv8mMain => {
                bld.define("TX_SINGLE_MODE_SECURE", None);
            }
            Self::Win32 => {
                // win32 tx_port.h always enables performance info.
                for cfg in PERFORMANCE_INFO_CFGS {
                    println!("cargo::rustc-cfg={cfg}");
                }
            }
            _ => {}
        }
    }
}

fn estimate_threadx_port(host: &str, target: &str) -> Port {
    if host == target {
        if host.contains("linux") {
            return Port::Linux;
        }
        if host.contains("windows") {
            return Port::Win32;
        }
    }

    if target.starts_with("thumbv7m") || target.starts_with("thumbv7em") {
        return Port::Armv7m;
    }
    if target.starts_with("thumbv6m") {
        return Port::Armv6m;
    }
    if target.starts_with("thumbv8m.base") {
        return Port::Armv8mBase;
    }
    if target.starts_with("thumbv8m.main") {
        return Port::Armv8mMain;
    }

    panic!("Cannot estimate ThreadX port for target {target} built from host {host}");
}

static PERFORMANCE_INFO_CFGS: &[&'static str] = &[
    "tx_block_pool_enable_performance_info",
    "tx_byte_pool_enable_performance_info",
    "tx_event_flags_enable_performance_info",
    "tx_mutex_enable_performance_info",
    "tx_queue_enable_performance_info",
    "tx_semaphore_enable_performance_info",
    "tx_thread_enable_performance_info",
    "tx_timer_enable_performance_info",
];

static BINARY_CFGS: &[&'static str] = &["tx_disable_notify_callbacks"];

fn cfgs_to_defines() -> HashMap<String, String> {
    let cfgs = PERFORMANCE_INFO_CFGS.iter().chain(BINARY_CFGS.iter());

    let cargo_env_vars = cfgs
        .clone()
        .map(|cfg| format!("CARGO_CFG_{}", cfg.to_uppercase()));
    let defines = cfgs.map(|cfg| cfg.to_uppercase());
    cargo_env_vars.zip(defines).collect()
}

fn set_cfg_defines(bld: &mut cc::Build) {
    let cfgs_to_defines = cfgs_to_defines();
    for (cfg, _) in env::vars() {
        if let Some(define) = cfgs_to_defines.get(&cfg) {
            bld.define(define, None);
        }
    }
}

fn main() -> Result<(), Error> {
    if env::var("DOCS_RS").is_ok() {
        println!("cargo::warning=Hello docs.rs! Skipping ThreadX (cross) compile");
        return Ok(());
    }

    let target = env::var("TARGET")?;
    let host = env::var("HOST")?;

    let port = estimate_threadx_port(&host, &target);

    let mut bld = cc::Build::new();
    port.set_defines(&mut bld);
    set_cfg_defines(&mut bld);

    bld.include("threadx/common/inc");
    for src in fs::read_dir("threadx/common/src")? {
        bld.file(src?.path());
    }

    bld.include(port.include_path());
    for src in fs::read_dir(port.source_path())? {
        bld.file(src?.path());
    }

    bld.compile("threadx");
    Ok(())
}
