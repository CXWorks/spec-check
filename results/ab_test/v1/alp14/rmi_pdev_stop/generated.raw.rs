```rust
pub open spec fn RMI_PDEV_STOP_spec(s: S, pdev_ptr: Address) -> (result: RmiCommandReturnCode, s_out: S) {
    // Failure condition: da_supp
    if !ImplFeatures(s).feat_da.is_FEATURE_TRUE() {
        (RmiCommandReturnCode::RMI_ERROR_NOT_SUPPORTED, s)
    }
    // Failure condition: pdev_align
    else if !AddrIsGranuleAligned(pdev_ptr) {
        (RmiCommandReturnCode::RMI_ERROR_INPUT, s)
    }
    // Failure condition: pdev_bound
    else if !PaIsDelegable(pdev_ptr) {
        (RmiCommandReturnCode::RMI_ERROR_INPUT, s)
    }
    // Failure condition: pdev_gran_state
    else if GranuleAt(s, pdev_ptr).state != RmmGranuleState::PDEV {
        (RmiCommandReturnCode::RMI_ERROR_INPUT, s)
    }
    // Failure condition: pdev_state
    else if pdev.state == RmmPdevState::PDEV_COMMUNICATING 
         || pdev.state == RmmPdevState::PDEV_STOPPING 
         || pdev.state == RmmPdevState::PDEV_STOPPED {
        (RmiCommandReturnCode::RMI_ERROR_DEVICE, s)
    }
    // Failure condition: num_vdevs
    else if pdev.num_vdevs != 0 {
        (RmiCommandReturnCode::RMI_ERROR_DEVICE, s)
    }
    // Success conditions
    else {
        let pdev_out = pdev.state := RmmPdevState::PDEV_STOPPING;
        let pdev_out = pdev_out.comm_state := RmmDevCommState::DEV_COMM_PENDING;
        let s_out = s.set_PdevAt(pdev_ptr, pdev_out);
        (RmiCommandReturnCode::RMI_SUCCESS, s_out)
    }
}
where pdev := PdevAt(s, pdev_ptr)
```