pub open spec fn ffa_rx_release_spec(result: UInt32, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_RX_RELEASE, CurrentFfaInstance()) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsRxTxPairRegisteredByHypervisor(old_s, vm_id(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!CallerOwnsRxBuffer(old_s, TargetRxBuffer(old_s, vm_id(old_s))) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, FFA_SUCCESS) ==> !CallerOwnsRxBuffer(new_s, TargetRxBuffer(new_s, vm_id(old_s))))
    && (ResultEqual(result, FFA_SUCCESS) ==> TargetRxBuffer(new_s, vm_id(old_s)).owner == 0)
}