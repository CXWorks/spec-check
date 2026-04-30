use vstd::prelude::*;

verus! {

// ---------------------------------------------------------------------------
// Primitive type aliases
// ---------------------------------------------------------------------------

type UInt32  = u32;
type UInt64  = u64;
type Int32   = i32;
type Bits64  = u64;

// ---------------------------------------------------------------------------
// SCMI return codes  (Table 5 in DEN0056F v4.0, §3.1.4)
// Scope: Base Protocol + Power Domain Protocol
// ---------------------------------------------------------------------------

pub spec const SCMI_SUCCESS:             int = 0;
pub spec const SCMI_NOT_SUPPORTED:       int = -1;
pub spec const SCMI_INVALID_PARAMETERS:  int = -2;
pub spec const SCMI_DENIED:              int = -3;
pub spec const SCMI_NOT_FOUND:           int = -4;
pub spec const SCMI_OUT_OF_RANGE:        int = -5;
pub spec const SCMI_BUSY:                int = -6;
pub spec const SCMI_COMMS_ERROR:         int = -7;
pub spec const SCMI_GENERIC_ERROR:       int = -8;
pub spec const SCMI_HARDWARE_ERROR:      int = -9;
pub spec const SCMI_PROTOCOL_ERROR:      int = -10;

// ---------------------------------------------------------------------------
// SCMI protocol IDs  (DEN0056F v4.0)
// ---------------------------------------------------------------------------

pub spec const SCMI_BASE_PROTOCOL_ID:         int = 0x10;
pub spec const SCMI_POWER_DOMAIN_PROTOCOL_ID: int = 0x11;

// ---------------------------------------------------------------------------
// Base protocol message IDs  (§3.2.2.x)
// ---------------------------------------------------------------------------

pub spec const SCMI_MSG_PROTOCOL_VERSION:                    int = 0x0;
pub spec const SCMI_MSG_PROTOCOL_ATTRIBUTES:                 int = 0x1;
pub spec const SCMI_MSG_PROTOCOL_MESSAGE_ATTRIBUTES:         int = 0x2;
pub spec const SCMI_BASE_MSG_DISCOVER_VENDOR:                int = 0x3;
pub spec const SCMI_BASE_MSG_DISCOVER_SUB_VENDOR:            int = 0x4;
pub spec const SCMI_BASE_MSG_DISCOVER_IMPLEMENTATION_VERSION: int = 0x5;
pub spec const SCMI_BASE_MSG_DISCOVER_LIST_PROTOCOLS:        int = 0x6;
pub spec const SCMI_BASE_MSG_DISCOVER_AGENT:                 int = 0x7;
pub spec const SCMI_BASE_MSG_NEGOTIATE_PROTOCOL_VERSION:     int = 0x10;
pub spec const SCMI_BASE_MSG_NOTIFY_ERRORS:                  int = 0x8;
pub spec const SCMI_BASE_MSG_SET_DEVICE_PERMISSIONS:         int = 0x9;
pub spec const SCMI_BASE_MSG_SET_PROTOCOL_PERMISSIONS:       int = 0xA;
pub spec const SCMI_BASE_MSG_RESET_AGENT_CONFIGURATION:      int = 0xB;

// ---------------------------------------------------------------------------
// Power Domain protocol message IDs  (§3.3.2.x)
// ---------------------------------------------------------------------------

pub spec const SCMI_POWER_MSG_POWER_DOMAIN_ATTRIBUTES:              int = 0x3;
pub spec const SCMI_POWER_MSG_POWER_STATE_SET:                      int = 0x4;
pub spec const SCMI_POWER_MSG_POWER_STATE_GET:                      int = 0x5;
pub spec const SCMI_POWER_MSG_POWER_STATE_NOTIFY:                   int = 0x6;
pub spec const SCMI_POWER_MSG_POWER_STATE_CHANGE_REQUESTED_NOTIFY:  int = 0x7;
pub spec const SCMI_POWER_MSG_POWER_DOMAIN_NAME_GET:                int = 0x8;

// ---------------------------------------------------------------------------
// SCMI status / error code wrapper (used in Result<T, SCMIStatusCode>)
// ---------------------------------------------------------------------------

pub struct SCMIStatusCode(pub int);
impl SCMIStatusCode {
    pub open spec fn as_int(self) -> int { self.0 }
}

// Alias used by some model outputs
type SCMI_ERROR = SCMIStatusCode;

// ---------------------------------------------------------------------------
// Global state (Base + Power Domain SCMI state)
// ---------------------------------------------------------------------------

pub struct S {
    /// Number of agents in the system.
    pub num_agents: u32,
    /// Number of power domains.
    pub num_power_domains: u32,
    /// Platform-supported protocol count.
    pub num_protocols: u32,
    /// Whether the SCMI platform is initialized.
    pub initialized: bool,
}

// ---------------------------------------------------------------------------
// Uninterpreted helper predicates used by SCMI command specs
// ---------------------------------------------------------------------------

/// Returns true if the agent with the given ID exists.
pub open spec fn AgentExists(s: S, agent_id: Bits64) -> bool;

/// Returns true if the agent is a privileged (management) agent.
pub open spec fn AgentIsPrivileged(s: S, agent_id: Bits64) -> bool;

/// Returns true if the power domain with the given ID exists.
pub open spec fn PowerDomainExists(s: S, domain_id: Bits64) -> bool;

/// Returns true if the power domain supports power state notifications.
pub open spec fn PowerDomainSupportsNotify(s: S, domain_id: Bits64) -> bool;

/// Returns true if the given protocol ID is supported by the platform.
pub open spec fn ProtocolIsSupported(s: S, protocol_id: Bits64) -> bool;

/// Returns true if the given message ID is supported within the given protocol.
pub open spec fn MessageIsSupported(s: S, protocol_id: Bits64, msg_id: Bits64) -> bool;

/// Returns true if the caller agent has access permission to the given device.
pub open spec fn AgentHasDevicePermission(s: S, agent_id: Bits64, device_id: Bits64) -> bool;

/// Returns true if the requested power state is valid for the given domain.
pub open spec fn PowerStateIsValid(s: S, domain_id: Bits64, power_state: Bits64) -> bool;

} // verus!
