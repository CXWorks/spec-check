pub open spec fn RMI_VDEV_CREATE_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    pdev_ptr: Address,
    vdev_ptr: Address,
    params_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let pdev = PdevAt(old_s, pdev_ptr);
    let num_vdevs_pre = pdev.num_vdevs;
    let vdev = VdevAt(new_s, vdev_ptr);
    let params = RmiVdevParamsAt(old_s, params_ptr);
    let num_aux = VdevAuxCount(old_s, PdevFlags(pdev), params.flags);

    // Failure conditions
    ((!ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (pdev.state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegableDram(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(old_s, vdev_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!GranuleAccessPermitted(params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!RmiVdevParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (realm_pre.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_REALM)) &&
    (params.num_aux != num_aux ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AuxAligned32(params.aux, params.num_aux) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (AuxAlias32(vdev_ptr, params.aux, params.num_aux) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AuxStateEqual32(old_s, params.aux, params.num_aux, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!VdevIdIsFree(old_s, realm_pre, params.vdev_id) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!TdiIdIsFree(old_s, params.tdi_id, pdev.segment_id) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((UInt(params.tdi_id) < UInt(pdev.rid_base) || UInt(params.tdi_id) >= UInt(pdev.rid_top)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((params.flags.VSMMU == RMI_FEATURE_TRUE && !AddrIsGranuleAligned(params.vsmmu_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((params.flags.VSMMU == RMI_FEATURE_TRUE && !PaIsDelegable(params.vsmmu_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((params.flags.VSMMU == RMI_FEATURE_TRUE && GranuleAt(old_s, params.vsmmu_addr).state != VSMMU) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((params.flags.VSMMU == RMI_FEATURE_TRUE && !VsidIsFree(old_s, VsmmuAt(old_s, params.vsmmu_addr), params.vsid)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((params.flags.VSMMU == RMI_FEATURE_TRUE && !PdevVsmmuIsCompatible(pdev, VsmmuAt(old_s, params.vsmmu_addr))) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&

    // Success conditions
    (result.is_Ok() ==> (
        PdevAt(new_s, pdev_ptr).num_vdevs == num_vdevs_pre + 1 &&
        GranuleAt(new_s, vdev_ptr).state == VDEV &&
        vdev.vdev_id == params.vdev_id &&
        vdev.tdi_id == params.tdi_id &&
        vdev.pdev == pdev_ptr &&
        vdev.realm == rd &&
        vdev.vdev_state == VDEV_NEW &&
        vdev.dma_state == VDEV_DMA_DISABLED &&
        vdev.op == VDEV_OP_UNLOCK &&
        vdev.comm_state == DEV_COMM_PENDING &&
        AuxEqual32(vdev.aux, params.aux, num_aux) &&
        vdev.num_aux == num_aux &&
        AuxStateEqual32(new_s, vdev.aux, num_aux, VDEV_AUX) &&
        !TdiIdIsFree(new_s, params.tdi_id, pdev.segment_id) &&
        vdev.vsmmu == params.flags.VSMMU &&
        (params.flags.VSMMU == RMI_FEATURE_TRUE ==> vdev.vsmmu_addr == params.vsmmu_addr) &&
        (params.flags.VSMMU == RMI_FEATURE_TRUE ==> vdev.vsid == params.vsid) &&
        (params.flags.VSMMU == RMI_FEATURE_TRUE ==> !VsidIsFree(new_s, VsmmuAt(new_s, params.vsmmu_addr), params.vsid)) &&
        vdev.num_map == 0 &&
        RealmAt(new_s, rd).num_vdevs == realm_pre.num_vdevs + 1 &&
        vdev.attest_info.lock_nonce == 0 &&
        vdev.attest_info.meas_nonce == 0 &&
        vdev.attest_info.report_nonce == 0 &&
        vdev.p2p_bound == FEATURE_FALSE
    ))
}