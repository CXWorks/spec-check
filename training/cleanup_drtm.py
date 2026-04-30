#!/usr/bin/env python3
"""cleanup_drtm.py — Post-process drtm_generated.rs → drtm_generated_clean.rs"""

import os
from cleanup_base import run_cleanup

BASE_DIR = os.path.dirname(os.path.abspath(__file__))

COMMANDS = [
    "DRTM_CLOSE_LOCALITY",
    "DRTM_DYNAMIC_LAUNCH",
    "DRTM_ENABLE_SECURE_INTERRUPTS",
    "DRTM_FEATURES",
    "DRTM_GET_ERROR",
    "DRTM_LOCK_TCB_HASHES",
    "DRTM_SET_ERROR",
    "DRTM_SET_TCB_HASH",
    "DRTM_UNPROTECT_MEMORY",
    "DRTM_VERSION",
]

if __name__ == "__main__":
    run_cleanup(
        layer1_path=os.path.join(BASE_DIR, "boilerplate", "layer1_drtm.rs"),
        generated_path=os.path.join(BASE_DIR, "drtm_generated.rs"),
        output_path=os.path.join(BASE_DIR, "drtm_generated_clean.rs"),
        commands=COMMANDS,
        extra_ops=r"\bDrtmIs|\bTcbIs|\bDlmeIs|\bDRTM_",
    )
