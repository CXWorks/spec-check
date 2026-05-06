pub open spec fn RMI_VDEV_START_spec(s: S, rd: Address, vdev_ptr: Address) -> Result<
    (),
    RmiStatusCode,
> {
    // Failure condition: da_supp
    if !ImplFeatures().feat_da != FEATURE_TRUE {
        return Err(RMI_ERROR_NOT_SUPPORTED);
    }
    // Failure condition: rd_align

    if !AddrIsGranuleAligned(rd) {
        return Err(RMI_ERROR_INPUT);
    }
    // Failure condition: rd_bound

    if !PaIsDelegable(rd) {
        return Err(RMI_ERROR_INPUT);
    }
    // Failure condition: rd_state

    if GranuleAt(s, rd).state != RD {
        return Err(RMI_ERROR_INPUT);
    }
    // Failure condition: vdev_align

    if !AddrIsGranuleAligned(vdev_ptr) {
        return Err(RMI_ERROR_INPUT);
    }
    // Failure condition: vdev_bound

    if !PaIsDelegable(vdev_ptr) {
        return Err(RMI_ERROR_INPUT);
    }
    // Failure condition: vdev_gran_state

    if GranuleAt(s, vdev_ptr).state != VDEV {
        return Err(RMI_ERROR_INPUT);
    }
    // Failure condition: vdev_realm

    let realm = RealmAt(s, rd);
    let vdev = VdevAt(s, vdev_ptr);
    if vdev.realm != rd {
        return Err(RMI_ERROR_INPUT);
    }
    // Failure condition: vdev_state

    if vdev.vdev_state != VDEV_LOCKED {
        return Err(RMI_ERROR_DEVICE);
    }
    // Failure condition: comm_state

    if vdev.comm_state != DEV_COMM_IDLE {
        return Err(RMI_ERROR_DEVICE);
    }
    // Success conditions
    // post: vdev.op == VDEV_OP_START
    // post: vdev.comm_state == DEV_COMM_PENDING

    Ok(())
}