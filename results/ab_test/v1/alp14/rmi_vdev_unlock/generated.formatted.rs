pub open spec fn RMI_VDEV_UNLOCK_spec(s: S, rd: Address, vdev_ptr: Address) -> (result: RmiCommandReturnCode, s_post: S) {
    // Failure condition: da_supp
    if !ImplFeatures().feat_da.is_FEATURE_TRUE() {
        (RMI_ERROR_NOT_SUPPORTED, s)
    }
    // Failure condition: rd_align
    else if !AddrIsGranuleAligned(rd) {
        (RMI_ERROR_INPUT, s)
    }
    // Failure condition: vdev_align
    else if !AddrIsGranuleAligned(vdev_ptr) {
        (RMI_ERROR_INPUT, s)
    }
    // Failure condition: rd_bound
    else if !PaIsDelegable(rd) {
        (RMI_ERROR_INPUT, s)
    }
    // Failure condition: vdev_bound
    else if !PaIsDelegable(vdev_ptr) {
        (RMI_ERROR_INPUT, s)
    }
    // Failure condition: rd_state
    else if GranuleAt(s, rd).state != RD {
        (RMI_ERROR_INPUT, s)
    }
    // Failure condition: vdev_gran_state
    else if GranuleAt(s, vdev_ptr).state != VDEV {
        (RMI_ERROR_INPUT, s)
    }
    // Failure condition: vdev_realm
    else if VdevAt(s, vdev_ptr).realm != rd {
        (RMI_ERROR_INPUT, s)
    }
    // Failure condition: vdev_state
    else if VdevAt(s, vdev_ptr).vdev_state != VDEV_LOCKED
         && VdevAt(s, vdev_ptr).vdev_state != VDEV_STARTED
         && VdevAt(s, vdev_ptr).vdev_state != VDEV_ERROR {
        (RMI_ERROR_DEVICE, s)
    }
    // Failure condition: comm_state
    else if VdevAt(s, vdev_ptr).comm_state != DEV_COMM_IDLE {
        (RMI_ERROR_DEVICE, s)
    }
    // Failure condition: num_map
    else if VdevAt(s, vdev_ptr).num_map != 0 {
        (RMI_ERROR_DEVICE, s)
    }
    // Success case
    else {
        let s_post = update_vdev(
            s,
            vdev_ptr,
            |vdev: RmmVdev| RmmVdev {
                dma_state: VDEV_DMA_DISABLED,
                op: VDEV_OP_UNLOCK,
                comm_state: DEV_COMM_PENDING,
                ..vdev
            }
        );
        (RMI_SUCCESS, s_post)
    }
}