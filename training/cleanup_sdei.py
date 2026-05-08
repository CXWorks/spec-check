#!/usr/bin/env python3
"""cleanup_sdei.py — Post-process sdei_generated.rs → sdei_generated_clean.rs"""

import os
from cleanup_base import run_cleanup

BASE_DIR = os.path.dirname(os.path.abspath(__file__))

COMMANDS = [
    "SDEI_COMPLETE",
    "SDEI_COMPLETE_AND_RESUME",
    "SDEI_EVENT_COMPLETE",
    "SDEI_EVENT_COMPLETE_AND_RESUME",
    "SDEI_EVENT_CONTEXT",
    "SDEI_EVENT_DISABLE",
    "SDEI_EVENT_ENABLE",
    "SDEI_EVENT_GET_INFO",
    "SDEI_EVENT_REGISTER",
    "SDEI_EVENT_ROUTING_SET",
    "SDEI_EVENT_SIGNAL",
    "SDEI_EVENT_STATUS",
    "SDEI_EVENT_UNREGISTER",
    "SDEI_FEATURES",
    "SDEI_INTERRUPT_BIND",
    "SDEI_INTERRUPT_RELEASE",
    "SDEI_PE_MASK",
    "SDEI_PE_UNMASK",
    "SDEI_PRIVATE_RESET",
    "SDEI_SHARED_RESET",
    "SDEI_VERSION",
]

if __name__ == "__main__":
    run_cleanup(
        layer1_path=os.path.join(BASE_DIR, "boilerplate", "layer1_sdei.rs"),
        generated_path=os.path.join(BASE_DIR, "sdei_generated.rs"),
        output_path=os.path.join(BASE_DIR, "sdei_generated_clean.rs"),
        commands=COMMANDS,
        extra_ops=r"\bEventIs|\bPeIs|\bInterruptIs|\bSDEI_",
    )
