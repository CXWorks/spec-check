#!/usr/bin/env python3
"""
cleanup_sbi.py

Post-process sbi_generated.rs to fix model output artifacts and produce
a syntactically valid sbi_generated_clean.rs.

Delegates all heavy lifting to cleanup_base.run_cleanup().
"""

import os
from cleanup_base import run_cleanup

BASE_DIR = os.path.dirname(os.path.abspath(__file__))

COMMANDS = sorted([
    # Base Extension (Chapter 3)
    "SBI_GET_SPEC_VERSION",
    "SBI_GET_IMPL_ID",
    "SBI_GET_IMPL_VERSION",
    "SBI_PROBE_EXTENSION",
    "SBI_GET_MVENDORID",
    "SBI_GET_MARCHID",
    "SBI_GET_MIMPID",
    # Timer Extension (Chapter 5)
    "SBI_SET_TIMER",
    # IPI Extension (Chapter 6)
    "SBI_SEND_IPI",
    # RFENCE Extension (Chapter 7)
    "SBI_REMOTE_FENCE_I",
    "SBI_REMOTE_SFENCE_VMA",
    "SBI_REMOTE_SFENCE_VMA_ASID",
    "SBI_REMOTE_HFENCE_GVMA_VMID",
    "SBI_REMOTE_HFENCE_VVMA",
    # HSM Extension (Chapter 8)
    "SBI_HART_START",
    "SBI_HART_STOP",
    "SBI_HART_GET_STATUS",
    "SBI_HART_SUSPEND",
    # System Reset Extension (Chapter 9)
    "SBI_SYSTEM_RESET",
    # PMU Extension (Chapter 10)
    "SBI_PMU_NUM_COUNTERS",
    "SBI_PMU_COUNTER_GET_INFO",
    "SBI_PMU_COUNTER_CONFIG_MATCHING",
    "SBI_PMU_COUNTER_START",
    "SBI_PMU_COUNTER_STOP",
    "SBI_PMU_COUNTER_FW_READ",
    # Debug Console Extension (Chapter 11)
    "SBI_DEBUG_CONSOLE_WRITE",
    "SBI_DEBUG_CONSOLE_READ",
    # NACL Extension (Chapter 12)
    "SBI_NACL_PROBE_FEATURE",
    "SBI_NACL_SETUP_SHMEM",
])

if __name__ == "__main__":
    run_cleanup(
        layer1_path=os.path.join(BASE_DIR, "boilerplate", "layer1_sbi.rs"),
        generated_path=os.path.join(BASE_DIR, "sbi_generated.rs"),
        output_path=os.path.join(BASE_DIR, "sbi_generated_clean.rs"),
        commands=COMMANDS,
        extra_ops=r"\bSBI_|\bHartIs|\bAddrRange|\bShmem|\bPmuCounter|\bExtension",
    )
