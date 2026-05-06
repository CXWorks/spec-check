```rust
pub open spec fn RMI_REC_ENTER_spec(
    s: S,
    rec_ptr: Address,
    run_ptr: Address,
) -> (result: Result<(), RmiStatusCode>, run: RmiRecRun, rec: RmmRec) {
    let run = RmiRecRunAt(s, run_ptr);
    let rec = RecAt(s, rec_ptr);
    let realm = RealmAt(s, rec.owner);
    
    // Failure conditions checked in order of precedence
    if !AddrIsGranuleAligned(run_ptr) {
        (Err(RMI_ERROR_INPUT), run, rec)
    } else if !GranuleAccessPermitted(s, run_ptr, PAS_NS) {
        (Err(RMI_ERROR_INPUT), run, rec)
    } else if !AddrIsGranuleAligned(rec_ptr) {
        (Err(RMI_ERROR_INPUT), run, rec)
    } else if !PaIsDelegable(rec_ptr) {
        (Err(RMI_ERROR_INPUT), run, rec)
    } else if GranuleAt(s, rec_ptr).state != REC {
        (Err(RMI_ERROR_INPUT), run, rec)
    } else if realm.state == REALM_NEW {
        (Err(RMI_ERROR_REALM(0)), run, rec)
    } else if realm.state == REALM_SYSTEM_OFF {
        (Err(RMI_ERROR_REALM(1)), run, rec)
    } else if rec.state == REC_RUNNING {
        (Err(RMI_ERROR_REC), run, rec)
    } else if rec.flags.runnable == NOT_RUNNABLE {
        (Err(RMI_ERROR_REC), run, rec)
    } else if run.enter.flags.emul_mmio == RMI_EMULATED_MMIO && rec.emulatable_abort != EMULATABLE_ABORT {
        (Err(RMI_ERROR_REC), run, rec)
    } else if !Gicv3ConfigIsValid(s, run.enter.gicv3_hcr, run.enter.gicv3_lrs) {
        (Err(RMI_ERROR_REC), run, rec)
    } else if rec.pending != REC_PENDING_NONE {
        (Err(RMI_ERROR_REC), run, rec)
    } else {
        // Success: run.exit contains Realm exit syndrome information
        // rec.emulatable_abort is updated
        (Ok(()), run, rec)
    }
}
```