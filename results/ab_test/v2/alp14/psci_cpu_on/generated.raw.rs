```rust
pub open spec fn psci_cpu_on_spec(
    result: PsciReturnCode,
    old_s: S,
    new_s: S,
    target_cpu: u64,
    entry_point_address: Address,
    context_id: u32
) -> bool {
    let realm = CurrentRealm(old_s);
    let target_rec = RecFromMpidr(old_s, target_cpu);
    
    // Failure condition: entry
    ((!AddrIsProtected(old_s, entry_point_address, realm)) ==> result == PSCI_INVALID_ADDRESS)
    
    // Failure condition: mpidr
    && ((!MpidrIsUsed(old_s, target_cpu)) ==> result == PSCI_INVALID_PARAMETERS)
    
    // Failure condition: runnable
    && ((target_rec.flags.runnable == RUNNABLE) ==> result == PSCI_ALREADY_ON)
    
    // Success condition: entry
    && ((result == PSCI_SUCCESS) ==> (
        RecFromMpidr(new_s, target_cpu).pc == ToBits64(UInt(entry_point_address))
    ))
    
    // Success condition: runnable
    && ((result == PSCI_SUCCESS) ==> (
        RecFromMpidr(new_s, target_cpu).flags.runnable == RUNNABLE
    ))
}
```