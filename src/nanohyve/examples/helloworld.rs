// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause
use std::convert::TryFrom;
use std::path::PathBuf;

use nanohyve::{KernelConfig, MemoryConfig, BlockConfig, VMMConfig, VcpuConfig, Vmm, DEFAULT_KERNEL_LOAD_ADDR};

fn default_memory_config() -> MemoryConfig {
    MemoryConfig { size_mib: 1024 }
}

fn default_kernel_config(path: PathBuf) -> KernelConfig {
    KernelConfig {
        path,
        load_addr: DEFAULT_KERNEL_LOAD_ADDR, // 1 MB
        cmdline: KernelConfig::default_cmdline(),
    }
}

fn default_vcpu_config() -> VcpuConfig {
    VcpuConfig { num: 1 }
}

fn run_vmm(kernel_path: PathBuf, block_path: PathBuf) {
    let vmm_config = VMMConfig {
        kernel_config: default_kernel_config(kernel_path),
        memory_config: default_memory_config(),
        vcpu_config: default_vcpu_config(),
        block_config: Some(BlockConfig {
            path: block_path,
        }),
        net_config: None,
    };

    let mut vmm = Vmm::try_from(vmm_config).unwrap();
    vmm.run().unwrap();
}

fn main() {
    run_vmm("./hypervisor-fw".parse().unwrap(), "./helloworld.img".parse().unwrap());
}
