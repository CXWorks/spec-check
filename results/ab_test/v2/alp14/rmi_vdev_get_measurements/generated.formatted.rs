pub open spec fn rmi_vdev_get_measurements_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
) -> bool {
    let rd = old_s.x1;
    let vdev_ptr = old_s.x2;
    let params_ptr = old_s.x3;

    let realm = RealmAt(old_s, rd);
    let vdev = VdevAt(old_s, vdev_ptr);
    let params = RmiVdevMeasureParamsAt(old_s, params_ptr);

    // Failure conditions
    let da_supp_fail = !ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> ResultEqual(
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

    let vdev_realm_fail = vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT);

    let vdev_state_fail = (vdev.vdev_state != VDEV_LOCKED && vdev.vdev_state != VDEV_STARTED)
        ==> ResultEqual(result, RMI_ERROR_DEVICE);

    let comm_state_fail = vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    );

    let params_align_fail = !AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    let params_pas_fail = !GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    let indices_fail = (params.indices[0] == 1 || params.indices[255] == 1) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    // Success conditions
    let success_op = result.is_Ok() ==> vdev.op == VDEV_OP_GET_MEAS;
    let success_comm_state = result.is_Ok() ==> vdev.comm_state == DEV_COMM_PENDING;

    da_supp_fail && rd_align_fail && rd_bound_fail && rd_state_fail && vdev_align_fail
        && vdev_bound_fail && vdev_gran_state_fail && vdev_realm_fail && vdev_state_fail
        && comm_state_fail && params_align_fail && params_pas_fail && indices_fail && success_op
        && success_comm_state
}