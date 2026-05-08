```rust
pub open spec fn PSCI_CPU_ON_spec(
    s: S,
    target_cpu: u64,
    entry_point_address: Address,
    context_id: u32,
) -> (result: PsciReturnCode, s_out: S) {
    let realm = CurrentRealm(s);
    let target_rec = RecFromMpidr(s, target_cpu);
    
    if !AddrIsProtected(s, entry_point_address, realm) {
        (PSCI_INVALID_ADDRESS, s)
    } else if !MpidrIsUsed(s, target_cpu) {
        (PSCI_INVALID_PARAMETERS, s)
    } else if target_rec.flags.runnable == RUNNABLE {
        (PSCI_ALREADY_ON, s)
    } else {
        let s_out = s.update(
            target_rec.update(
                pc := ToBits64(s, UInt(entry_point_address)),
                flags := target_rec.flags.update(runnable := RUNNABLE),
            ),
        );
        (PSCI_SUCCESS, s_out)
    }
}
```

This function:
1. Extracts the current realm and target REC from the input parameters
2. Checks failure conditions in order:
   - Invalid entry point address (not in protected memory)
   - Invalid MPIDR (target CPU not in use)
   - Target REC already runnable
3. On success, updates the target REC with:
   - `pc` set to the entry point address
   - `runnable` flag set to RUNNABLE
4. Returns the result code and updated state