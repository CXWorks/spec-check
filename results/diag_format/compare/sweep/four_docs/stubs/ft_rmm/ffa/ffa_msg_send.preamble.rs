use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;

pub const FFA_SUCCESS: UInt32 = 0x84000061u32;
pub const FFA_ERROR: UInt32 = 0x84000060u32;

pub struct S {
    pub rx_buffers: Map<UInt16, Seq<u8>>,
    pub tx_buffers: Map<UInt16, Seq<u8>>,
    pub pending_message_notified: Map<UInt16, bool>,
}

pub open spec fn RxBufferOf(s: S, vm_id: UInt16) -> Seq<u8>;

pub open spec fn TxBufferOf(s: S, vm_id: UInt16) -> Seq<u8>;

pub open spec fn SchedulerInformedOfPendingMessage(s: S, vm_id: UInt16) -> bool;

pub open spec fn ContainsMessageCopiedFrom(rx: Seq<u8>, tx: Seq<u8>, msg_size: UInt32) -> bool;

} // verus!
