pub open spec fn rmi_vdev_unlock_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    rd: Address,
    vdev_ptr: Address,
) -> bool {
    // Failure conditions
    let da_supp_fail = !ImplFeatures(old_s).feat_da.eq_FEATURE_TRUE() ==> ResultEqual(
        result,
        RMI_ERROR_NOT_SUPPORTED,
    );

    let rd_align_fail = !AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT);

    let rd_bound_fail = !PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT);

    let rd_state_fail = GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT);

    let vdev_align_fail = !AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    let vdev_bound_fail = !PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);

    let vdev_gran_state_fail = GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    let realm = RealmAt(old_s, rd);
    let vdev = VdevAt(old_s, vdev_ptr);

    let vdev_realm_fail = vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT);

    let vdev_state_fail = (vdev.vdev_state != VDEV_LOCKED && vdev.vdev_state != VDEV_STARTED
        && vdev.vdev_state != VDEV_ERROR) ==> ResultEqual(result, RMI_ERROR_DEVICE);

    let comm_state_fail = vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    );

    let num_map_fail = vdev.num_map != 0 ==> ResultEqual(result, RMI_ERROR_DEVICE);

    // Success conditions
    let success_dma_state = result.is_Ok() ==> VdevAt(new_s, vdev_ptr).dma_state
        == VDEV_DMA_DISABLED;

    let success_op = result.is_Ok() ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_UNLOCK;

    let success_comm_state = result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state
        == DEV_COMM_PENDING;

    // Footprint constraints
    let footprint_op = VdevAt(new_s, vdev_ptr).op == VDEV_OP_UNLOCK;
    let footprint_comm_state = VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING;

    da_supp_fail && rd_align_fail && rd_bound_fail && rd_state_fail && vdev_align_fail
        && vdev_bound_fail && vdev_gran_state_fail && vdev_realm_fail && vdev_state_fail
        && comm_state_fail && num_map_fail && success_dma_state && success_op && success_comm_state
}