pub open spec fn ffa_interrupt_spec(function_id: UInt32, endpoint_vcpu_ids: UInt32, interrupt_id: UInt32, old_s: S, new_s: S) -> bool {
    (function_id == 0x84000062u32 || function_id == 0xC4000062u32)
    && (FfaInterruptReportsPreemptedRequest(old_s) ==> (
        ((endpoint_vcpu_ids >> 16u32) & 0xFFFFu32) == FfaPreemptedPartitionId(old_s)
        && (endpoint_vcpu_ids & 0xFFFFu32) == FfaPreemptedVcpuId(old_s)
        && interrupt_id == 0u32
    ))
    && (FfaInterruptDelegatesToWaitingCallee(old_s) ==> (
        endpoint_vcpu_ids == 0u32
        && interrupt_id == FfaPendingInterruptId(old_s)
    ))
    && ((!FfaInterruptReportsPreemptedRequest(old_s) && !FfaInterruptDelegatesToWaitingCallee(old_s)) ==> (
        endpoint_vcpu_ids == 0u32
        && interrupt_id == 0u32
    ))
}
