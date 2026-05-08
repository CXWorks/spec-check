```verus
pub open spec fn PSCI_CPU_ON_spec(
    old_s: S,
    new_s: S,
    target_cpu: u64,
    entry_point_address: Address,
    context_id: u32,
    result: PsciReturnCode
) -> bool {
    let realm = CurrentRealm(old_s);
    let target_rec = RecFromMpidr(old_s, target_cpu);
    
    (
        // Failure: entry not protected
        (!AddrIsProtected(old_s, entry_point_address, realm) ==> result == PSCI_INVALID_ADDRESS) &&
        
        // Failure: MPIDR not used
        (!MpidrIsUsed(old_s, target_cpu) ==> result == PSCI_INVALID_PARAMETERS) &&
        
        // Failure: target REC already runnable
        (target_rec.flags.runnable == RUNNABLE ==> result == PSCI_ALREADY_ON) &&
        
        // Success: all conditions met
        (
            (AddrIsProtected(old_s, entry_point_address, realm) &&
             MpidrIsUsed(old_s, target_cpu) &&
             target_rec.flags.runnable != RUNNABLE)
            ==>
            (
                result == PSCI_SUCCESS &&
                new_s.recs[target_rec].pc == ToBits64(UInt(entry_point_address)) &&
                new_s.recs[target_rec].flags.runnable == RUNNABLE
            )
        )
    )
}
```