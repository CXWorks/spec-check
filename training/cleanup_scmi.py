#!/usr/bin/env python3
"""cleanup_scmi.py — Post-process scmi_generated.rs → scmi_generated_clean.rs"""

import os
from cleanup_base import run_cleanup

BASE_DIR = os.path.dirname(os.path.abspath(__file__))

# Base Protocol (§3.2) + Power Domain Protocol (§3.3) commands
# Shared names (PROTOCOL_VERSION etc.) are mapped by the first occurrence
# in the model output; the cleanup matcher uses cmd_lower-in-fn_name logic.
COMMANDS = [
    "BASE_DISCOVER_AGENT",
    "BASE_DISCOVER_IMPLEMENTATION_VERSION",
    "BASE_DISCOVER_LIST_PROTOCOLS",
    "BASE_DISCOVER_SUB_VENDOR",
    "BASE_DISCOVER_VENDOR",
    "BASE_ERROR_EVENT",
    "BASE_NOTIFY_ERRORS",
    "BASE_RESET_AGENT_CONFIGURATION",
    "BASE_SET_DEVICE_PERMISSIONS",
    "BASE_SET_PROTOCOL_PERMISSIONS",
    "NEGOTIATE_PROTOCOL_VERSION",
    "POWER_DOMAIN_ATTRIBUTES",
    "POWER_DOMAIN_NAME_GET",
    "POWER_STATE_CHANGE_REQUESTED",
    "POWER_STATE_CHANGE_REQUESTED_NOTIFY",
    "POWER_STATE_CHANGED",
    "POWER_STATE_GET",
    "POWER_STATE_NOTIFY",
    "POWER_STATE_SET",
    "PROTOCOL_ATTRIBUTES",
    "PROTOCOL_MESSAGE_ATTRIBUTES",
    "PROTOCOL_VERSION",
]

if __name__ == "__main__":
    run_cleanup(
        layer1_path=os.path.join(BASE_DIR, "boilerplate", "layer1_scmi.rs"),
        generated_path=os.path.join(BASE_DIR, "scmi_generated.rs"),
        output_path=os.path.join(BASE_DIR, "scmi_generated_clean.rs"),
        commands=COMMANDS,
        extra_ops=r"\bAgentIs|\bPowerDomainIs|\bProtocolIs|\bSCMI_",
    )
