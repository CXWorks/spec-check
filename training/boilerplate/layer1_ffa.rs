use vstd::prelude::*;

verus! {

// ---------------------------------------------------------------------------
// Primitive type aliases
// ---------------------------------------------------------------------------

type UInt32  = u32;
type UInt64  = u64;
type Int32   = i32;
type Bits64  = u64;
type Address = u64;

// ---------------------------------------------------------------------------
// FF-A error codes  (Table 12.2 in DEN0077A v1.3, §12.2)
// All values are 32-bit signed integers.
// ---------------------------------------------------------------------------

pub spec const FFA_SUCCESS:              int = 0;
pub spec const FFA_NOT_SUPPORTED:        int = -1;
pub spec const FFA_INVALID_PARAMETERS:   int = -2;
pub spec const FFA_NO_MEMORY:            int = -3;
pub spec const FFA_BUSY:                 int = -4;
pub spec const FFA_INTERRUPTED:          int = -5;
pub spec const FFA_DENIED:               int = -6;
pub spec const FFA_RETRY:                int = -7;
pub spec const FFA_ABORTED:              int = -8;
pub spec const FFA_NO_DATA:              int = -9;

// ---------------------------------------------------------------------------
// FF-A function IDs  (DEN0077A v1.3)
// ---------------------------------------------------------------------------

// Chapter 12 — Status
pub spec const FFA_ERROR_FID:                          int = 0x8400_006F;
pub spec const FFA_SUCCESS_32_FID:                     int = 0x8400_006E;
pub spec const FFA_SUCCESS_64_FID:                     int = 0xC400_006E;

// Chapter 13 — Setup and discovery
pub spec const FFA_VERSION_FID:                        int = 0x8400_0063;
pub spec const FFA_FEATURES_FID:                       int = 0x8400_0064;
pub spec const FFA_RX_ACQUIRE_FID:                     int = 0x8400_0084;
pub spec const FFA_RX_RELEASE_FID:                     int = 0x8400_0065;
pub spec const FFA_RXTX_MAP_32_FID:                    int = 0x8400_0066;
pub spec const FFA_RXTX_MAP_64_FID:                    int = 0xC400_0066;
pub spec const FFA_RXTX_UNMAP_FID:                     int = 0x8400_0067;
pub spec const FFA_PARTITION_INFO_GET_FID:              int = 0x8400_0068;
pub spec const FFA_PARTITION_INFO_GET_REGS_FID:         int = 0xC400_008B;
pub spec const FFA_ID_GET_FID:                         int = 0x8400_0069;
pub spec const FFA_SPM_ID_GET_FID:                     int = 0x8400_0085;
pub spec const FFA_CONSOLE_LOG_32_FID:                 int = 0x8400_008A;
pub spec const FFA_CONSOLE_LOG_64_FID:                 int = 0xC400_008A;
pub spec const FFA_NS_RES_INFO_GET_FID:                int = 0xC400_008D;
pub spec const FFA_ABORT_FID:                          int = 0x8400_008C;

// Chapter 14 — CPU cycle management
pub spec const FFA_MSG_WAIT_FID:                       int = 0x8400_006B;
pub spec const FFA_YIELD_FID:                          int = 0x8400_006C;
pub spec const FFA_RUN_FID:                            int = 0x8400_006D;
pub spec const FFA_INTERRUPT_FID:                      int = 0x8400_0075;
pub spec const FFA_NORMAL_WORLD_RESUME_FID:            int = 0x8400_007C;

// Chapter 15 — Messaging
pub spec const FFA_MSG_SEND2_FID:                      int = 0x8400_0086;
pub spec const FFA_MSG_SEND_DIRECT_REQ_32_FID:         int = 0x8400_006F;
pub spec const FFA_MSG_SEND_DIRECT_REQ_64_FID:         int = 0xC400_006F;
pub spec const FFA_MSG_SEND_DIRECT_RESP_32_FID:        int = 0x8400_0070;
pub spec const FFA_MSG_SEND_DIRECT_RESP_64_FID:        int = 0xC400_0070;
pub spec const FFA_MSG_SEND_DIRECT_REQ2_64_FID:        int = 0xC400_0087;
pub spec const FFA_MSG_SEND_DIRECT_RESP2_64_FID:       int = 0xC400_0088;

// Chapter 16 — Notifications
pub spec const FFA_NOTIFICATION_BITMAP_CREATE_FID:     int = 0x8400_007D;
pub spec const FFA_NOTIFICATION_BITMAP_DESTROY_FID:    int = 0x8400_007E;
pub spec const FFA_NOTIFICATION_BIND_FID:              int = 0x8400_007F;
pub spec const FFA_NOTIFICATION_UNBIND_FID:            int = 0x8400_0080;
pub spec const FFA_NOTIFICATION_SET_FID:               int = 0x8400_0081;
pub spec const FFA_NOTIFICATION_GET_FID:               int = 0x8400_0082;
pub spec const FFA_NOTIFICATION_BIND2_FID:             int = 0x8400_0089;
pub spec const FFA_NOTIFICATION_UNBIND2_FID:           int = 0x8400_008E;
pub spec const FFA_NOTIFICATION_SET2_FID:              int = 0x8400_008F;
pub spec const FFA_NOTIFICATION_GET2_FID:              int = 0x8400_0090;
pub spec const FFA_NOTIFICATION_INFO_GET_32_FID:       int = 0x8400_0079;
pub spec const FFA_NOTIFICATION_INFO_GET_64_FID:       int = 0xC400_0079;

// Chapter 17 — Interrupt management
pub spec const FFA_EL3_INTR_HANDLE_FID:               int = 0x8400_007A;

// ---------------------------------------------------------------------------
// FF-A partition / instance state
// ---------------------------------------------------------------------------

pub spec const FFA_INSTANCE_NS_PHYSICAL:    int = 0;
pub spec const FFA_INSTANCE_NS_VIRTUAL:     int = 1;
pub spec const FFA_INSTANCE_S_PHYSICAL:     int = 2;
pub spec const FFA_INSTANCE_S_VIRTUAL:      int = 3;

// ---------------------------------------------------------------------------
// Global state
// ---------------------------------------------------------------------------

pub struct S {
    /// Whether the FF-A framework is initialized.
    pub initialized: bool,
    /// Current FF-A instance type (NS_PHYSICAL, NS_VIRTUAL, etc.)
    pub instance_type: u32,
    /// Number of partitions currently registered.
    pub num_partitions: u32,
    /// Whether the caller's RX buffer is currently owned by the caller.
    pub rx_buffer_owned: bool,
    /// Whether the caller's TX buffer is currently owned by the caller.
    pub tx_buffer_owned: bool,
}

// ---------------------------------------------------------------------------
// Uninterpreted helper predicates used by FF-A function specs
// ---------------------------------------------------------------------------

/// Returns true if the partition with the given ID exists.
pub open spec fn PartitionExists(s: S, partition_id: Bits64) -> bool;

/// Returns true if the partition is a secure partition (SP).
pub open spec fn PartitionIsSecure(s: S, partition_id: Bits64) -> bool;

/// Returns true if the caller is operating from Normal World context.
pub open spec fn CallerIsNormalWorld(s: S) -> bool;

/// Returns true if the caller's RX buffer is owned by the caller (available for use).
pub open spec fn RxBufferOwned(s: S) -> bool;

/// Returns true if the caller's TX buffer is owned by the caller.
pub open spec fn TxBufferOwned(s: S) -> bool;

/// Returns true if the RXTX buffers have been mapped (FFA_RXTX_MAP called).
pub open spec fn RxtxIsMapped(s: S, partition_id: Bits64) -> bool;

/// Returns true if the memory handle is valid and accessible.
pub open spec fn MemoryHandleValid(s: S, handle: Bits64) -> bool;

/// Returns true if the notification bitmap has been created for the given partition.
pub open spec fn NotificationBitmapExists(s: S, partition_id: Bits64) -> bool;

/// Returns true if the calling VM/SP has the given notification bound.
pub open spec fn NotificationIsBound(s: S, partition_id: Bits64, notif_id: Bits64) -> bool;

/// Returns true if the FF-A version is supported.
pub open spec fn VersionIsSupported(s: S, major: Bits64, minor: Bits64) -> bool;

// FF-A error type alias used by some model outputs
pub struct FfaError(pub int);
impl FfaError {
    pub open spec fn as_int(self) -> int { self.0 }
}

// Type aliases used by some model outputs
type Int32 = i32;

} // verus!
