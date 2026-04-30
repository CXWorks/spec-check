#!/usr/bin/env python3
"""cleanup_ffa.py — Post-process ffa_generated.rs → ffa_generated_clean.rs"""

import os
from cleanup_base import run_cleanup

BASE_DIR = os.path.dirname(os.path.abspath(__file__))

COMMANDS = [
    "FFA_ABORT",
    "FFA_CONSOLE_LOG",
    "FFA_EL3_INTR_HANDLE",
    "FFA_ERROR",
    "FFA_FEATURES",
    "FFA_ID_GET",
    "FFA_INTERRUPT",
    "FFA_MSG_SEND2",
    "FFA_MSG_SEND_DIRECT_REQ",
    "FFA_MSG_SEND_DIRECT_REQ2",
    "FFA_MSG_SEND_DIRECT_RESP",
    "FFA_MSG_SEND_DIRECT_RESP2",
    "FFA_MSG_WAIT",
    "FFA_NORMAL_WORLD_RESUME",
    "FFA_NOTIFICATION_BIND",
    "FFA_NOTIFICATION_BIND2",
    "FFA_NOTIFICATION_BITMAP_CREATE",
    "FFA_NOTIFICATION_BITMAP_DESTROY",
    "FFA_NOTIFICATION_GET",
    "FFA_NOTIFICATION_GET2",
    "FFA_NOTIFICATION_INFO_GET",
    "FFA_NOTIFICATION_SET",
    "FFA_NOTIFICATION_SET2",
    "FFA_NOTIFICATION_UNBIND",
    "FFA_NOTIFICATION_UNBIND2",
    "FFA_NS_RES_INFO_GET",
    "FFA_PARTITION_INFO_GET",
    "FFA_PARTITION_INFO_GET_REGS",
    "FFA_RUN",
    "FFA_RXTX_MAP",
    "FFA_RXTX_UNMAP",
    "FFA_RX_ACQUIRE",
    "FFA_RX_RELEASE",
    "FFA_SPM_ID_GET",
    "FFA_SUCCESS",
    "FFA_VERSION",
    "FFA_YIELD",
]

if __name__ == "__main__":
    run_cleanup(
        layer1_path=os.path.join(BASE_DIR, "boilerplate", "layer1_ffa.rs"),
        generated_path=os.path.join(BASE_DIR, "ffa_generated.rs"),
        output_path=os.path.join(BASE_DIR, "ffa_generated_clean.rs"),
        commands=COMMANDS,
        extra_ops=r"\bPartitionIs|\bMemHandle|\bFFA_|\bNotificationIs|\bRxtxIs|\bVersionIs",
    )
