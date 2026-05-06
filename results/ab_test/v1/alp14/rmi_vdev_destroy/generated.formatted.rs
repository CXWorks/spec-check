pub open spec fn RMI_VDEV_DESTROY_spec(
    s: S,
    rd: Address,
    pdev_ptr: Address,
    vdev_ptr: Address,
    result: RmiCommandReturnCode,
) -> bool {
    let realm_pre = RealmAt(s, rd);
    let vdev_pre = VdevAt(s, vdev_ptr);
    let pdev_pre = PdevAt(s, pdev_ptr);
    let realm = RealmAt(s, rd);
    let pdev = PdevAt(s, pdev_ptr);

    // Failure conditions
    let da_supp_fail = !ImplFeatures(s).feat_da && ResultEqual(result, RMI_ERROR_NOT_SUPPORTED);
    let rd_align_fail = !AddrIsGranuleAligned(rd) && ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_fail = !PaIsDelegable(rd) && ResultEqual(result, RMI_ERROR_INPUT);
    let rd_gran_state_fail = GranuleAt(s, rd).state != RD && ResultEqual(result, RMI_ERROR_INPUT);
    let pdev_align_fail = !AddrIsGranuleAligned(pdev_ptr) && ResultEqual(result, RMI_ERROR_INPUT);
    let pdev_bound_fail = !PaIsDelegable(pdev_ptr) && ResultEqual(result, RMI_ERROR_INPUT);
    let pdev_gran_state_fail = GranuleAt(s, pdev_ptr).state != PDEV && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let vdev_align_fail = !AddrIsGranuleAligned(vdev_ptr) && ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_bound_fail = !PaIsDelegable(vdev_ptr) && ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_gran_state_fail = GranuleAt(s, vdev_ptr).state != VDEV && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let vdev_realm_fail = vdev_pre.realm != rd && ResultEqual(result, RMI_ERROR_DEVICE);
    let vdev_pdev_fail = vdev_pre.pdev != pdev_ptr && ResultEqual(result, RMI_ERROR_DEVICE);
    let vdev_state_fail = (vdev_pre.vdev_state != VDEV_NEW && vdev_pre.vdev_state != VDEV_UNLOCKED
        && vdev_pre.vdev_state != VDEV_ERROR) && ResultEqual(result, RMI_ERROR_DEVICE);
    let num_map_fail = vdev_pre.num_map != 0 && ResultEqual(result, RMI_ERROR_DEVICE);

    // Success conditions
    let gran_state_success = GranuleAt(s, vdev_ptr).state == DELEGATED;
    let aux_state_success = AuxStateEqual32(s, vdev_pre.aux, vdev_pre.num_aux, DELEGATED);
    let vdev_id_free_success = VdevIdIsFree(s, realm, vdev_pre.vdev_id);
    let tdi_id_free_success = TdiIdIsFree(s, vdev_pre.tdi_id, pdev_pre.segment_id);
    let realm_num_vdevs_success = realm.num_vdevs == realm_pre.num_vdevs - 1;
    let pdev_num_vdevs_success = pdev.num_vdevs == pdev_pre.num_vdevs - 1;
    let vsid_free_success = vdev_pre.vsmmu == FEATURE_TRUE ==> VsidIsFree(
        s,
        VsmmuAt(s, vdev_pre.vsmmu_addr),
        vdev_pre.vsid,
    );

    // Success case: no failure conditions and all success conditions hold
    let success = result.is_Ok() && gran_state_success && aux_state_success && vdev_id_free_success
        && tdi_id_free_success && realm_num_vdevs_success && pdev_num_vdevs_success
        && vsid_free_success;

    // Failure cases
    let failure = da_supp_fail || rd_align_fail || rd_bound_fail || rd_gran_state_fail
        || pdev_align_fail || pdev_bound_fail || pdev_gran_state_fail || vdev_align_fail
        || vdev_bound_fail || vdev_gran_state_fail || vdev_realm_fail || vdev_pdev_fail
        || vdev_state_fail || num_map_fail;

    success || failure
}