pub open spec fn RMI_PSCI_COMPLETE_spec(
    s: S,
    calling_rec_ptr: Address,
    target_rec_ptr: Address,
    status: PsciReturnCode,
) -> (result: Result<(), RmiCommandReturnCode>) {
    // Failure conditions
    if calling_rec_ptr == target_rec_ptr {
        Err(RmiCommandReturnCode::RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(calling_rec_ptr) {
        Err(RmiCommandReturnCode::RMI_ERROR_INPUT)
    } else if !PaIsDelegable(calling_rec_ptr) {
        Err(RmiCommandReturnCode::RMI_ERROR_INPUT)
    } else if GranuleAt(s, calling_rec_ptr).state != GranuleState::REC {
        Err(RmiCommandReturnCode::RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(target_rec_ptr) {
        Err(RmiCommandReturnCode::RMI_ERROR_INPUT)
    } else if !PaIsDelegable(target_rec_ptr) {
        Err(RmiCommandReturnCode::RMI_ERROR_INPUT)
    } else if GranuleAt(s, target_rec_ptr).state != GranuleState::REC {
        Err(RmiCommandReturnCode::RMI_ERROR_INPUT)
    } else {
        let calling_rec = RecAt(s, calling_rec_ptr);
        let target_rec = RecAt(s, target_rec_ptr);
        
        if calling_rec.pending != RecPending::REC_PENDING_PSCI {
            Err(RmiCommandReturnCode::RMI_ERROR_INPUT)
        } else if target_rec.owner != calling_rec.owner {
            Err(RmiCommandReturnCode::RMI_ERROR_INPUT)
        } else if target_rec.mpidr != calling_rec.gprs[1] {
            Err(RmiCommandReturnCode::RMI_ERROR_INPUT)
        } else if !PsciReturnCodePermitted(s, calling_rec, target_rec, status) {
            Err(RmiCommandReturnCode::RMI_ERROR_INPUT)
        } else {
            // Success conditions
            Ok(())
        }
    }
}
```

**Note:** This specification encodes the failure conditions. The success conditions describe the state mutations that occur when the function returns `Ok(())`. To fully capture the command semantics, you would typically need a separate function that describes the post-state, such as:

```rust
pub open spec fn RMI_PSCI_COMPLETE_post(
    s_pre: S,
    s_post: S,
    calling_rec_ptr: Address,
    target_rec_ptr: Address,
    status: PsciReturnCode,
) -> bool {
    let calling_rec = RecAt(s_pre, calling_rec_ptr);
    let target_rec = RecAt(s_pre, target_rec_ptr);
    let calling_rec_post = RecAt(s_post, calling_rec_ptr);
    let target_rec_post = RecAt(s_post, target_rec_ptr);
    
    // pending condition
    calling_rec_post.pending == RecPending::REC_PENDING_NONE
    // ... additional success conditions based on status and FID
}