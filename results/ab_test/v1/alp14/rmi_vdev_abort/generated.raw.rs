```rust
pub open spec fn RMI_VDEV_ABORT_spec(s: S, vdev_ptr: Address) -> (result: RmiCommandReturnCode, vdev: RmmVdev) {
    let vdev = VdevAt(s, vdev_ptr);
    
    // Failure condition: da_supp
    if !ImplFeatures(s).feat_da {
        (RMI_ERROR_NOT_SUPPORTED, vdev)
    }
    // Failure condition: vdev_align
    else if !AddrIsGranuleAligned(vdev_ptr) {
        (RMI_ERROR_INPUT, vdev)
    }
    // Failure condition: vdev_bound
    else if !PaIsDelegable(vdev_ptr) {
        (RMI_ERROR_INPUT, vdev)
    }
    // Failure condition: vdev_gran_state
    else if GranuleAt(s, vdev_ptr).state != VDEV {
        (RMI_ERROR_INPUT, vdev)
    }
    // Failure condition: comm_state
    else if vdev.comm_state == DEV_COMM_IDLE {
        (RMI_ERROR_DEVICE, vdev)
    }
    // Success conditions
    else {
        let vdev_result = RmmVdev {
            vdev_state: VDEV_ERROR,
            comm_state: DEV_COMM_IDLE,
            ..vdev
        };
        (RMI_SUCCESS, vdev_result)
    }
}
```