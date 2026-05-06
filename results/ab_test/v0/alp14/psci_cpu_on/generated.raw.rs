```verus
pub open spec fn PSCI_CPU_ON_spec(
    s: S,
    fid: u64,
    target_cpu: u64,
    entry_point_address: Address,
    context_id: u32,
    result: PsciReturnCode,
) -> bool {
    let realm = CurrentRealm(s);
    let target_rec = RecFromMpidr(s, target_cpu);
    
    (
        // Failure condition: entry point not protected
        (
            !AddrIsProtected(s, entry_point_address, realm) ==>
            result == PsciReturnCode::PSCI_INVALID_ADDRESS
        ) &&
        // Failure condition: target_cpu not used
        (
            !MpidrIsUsed(s, target_cpu) ==>
            result == PsciReturnCode::PSCI_INVALID_PARAMETERS
        ) &&
        // Failure condition: target_rec already runnable
        (
            target_rec.flags.runnable == RmmRecRunnable::RUNNABLE ==>
            result == PsciReturnCode::PSCI_ALREADY_ON
        ) &&
        // Success condition: if all preconditions pass, result is success
        (
            (
                AddrIsProtected(s, entry_point_address, realm) &&
                MpidrIsUsed(s, target_cpu) &&
                target_rec.flags.runnable != RmmRecRunnable::RUNNABLE
            ) ==>
            (
                result == PsciReturnCode::PSCI_SUCCESS &&
                target_rec.pc == ToBits64(entry_point_address as int) &&
                target_rec.flags.runnable == RmmRecRunnable::RUNNABLE
            )
        )
    )
}
```