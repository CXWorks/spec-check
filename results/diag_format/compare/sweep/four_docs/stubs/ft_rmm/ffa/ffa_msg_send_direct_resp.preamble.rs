use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt = u64;
pub type Int32 = i32;
pub type FfaFunctionId = u32;
pub type ImpdefArgs = u64;

pub enum FfaCommandReturnCode {
    InvalidParameters,
    Denied,
    NotSupported,
    Aborted,
    Busy,
    Retry,
}

pub enum RunState {
    Running,
    Waiting,
    Blocked,
    Preempted,
    Aborted,
}

pub struct Message {
    pub source: UInt16,
    pub dest: UInt16,
    pub msg_type: UInt,
    pub payload: Seq<u64>,
}

pub struct EndpointState {
    pub id: UInt16,
    pub message: Message,
    pub run_state: RunState,
}

pub struct S {
    pub endpoints: Map<UInt16, EndpointState>,
    pub current_caller: UInt16,
    pub implemented_abis: Set<FfaFunctionId>,
}

pub spec const INVALID_PARAMETERS: FfaCommandReturnCode = FfaCommandReturnCode::InvalidParameters;
pub spec const DENIED: FfaCommandReturnCode = FfaCommandReturnCode::Denied;
pub spec const NOT_SUPPORTED: FfaCommandReturnCode = FfaCommandReturnCode::NotSupported;
pub spec const ABORTED: FfaCommandReturnCode = FfaCommandReturnCode::Aborted;

pub spec const FFA_SUCCESS: Result<(), FfaCommandReturnCode> = Ok(());

pub spec const FFA_MSG_SEND_DIRECT_RESP: FfaFunctionId = 0x8400_0070u32;

pub spec const impdef_args: ImpdefArgs = 0u64;

pub open spec fn IsValidEndpointId(s: S, id: UInt16) -> bool;

pub open spec fn ResultEqual(result: Result<(), FfaCommandReturnCode>, code: FfaCommandReturnCode) -> bool;

pub open spec fn AreValidMessageFlags(s: S, msg_type: UInt, flags_rsvd: UInt, fwk_msg_type: UInt) -> bool;

pub open spec fn CalleeCanHandleRequest(s: S) -> bool;

pub open spec fn CallerMayInvokeAbi(s: S, abi: FfaFunctionId) -> bool;

pub open spec fn EndpointSupportsDirectRespReceipt(s: S, id: UInt16) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, abi: FfaFunctionId) -> bool;

pub open spec fn EndpointAborted(s: S, id: UInt16) -> bool;

pub open spec fn DirectRespDelivered(s: S, source_id: UInt16, dest_id: UInt16, msg_type: UInt, fwk_msg_type: UInt, args: ImpdefArgs) -> bool;

pub open spec fn EndpointRan(s: S, id: UInt16) -> bool;

pub open spec fn NewMessageAvailableFor(s: S, id: UInt16) -> bool;

pub open spec fn SuccessReportedAsMsgWait(s: S, result: Result<(), FfaCommandReturnCode>) -> bool;

pub open spec fn Endpoint(s: S, id: UInt16) -> EndpointState;

} // verus!
