pub open spec fn RMI_VDEV_CREATE_spec(s: S, old_s: S, rd: Address, pdev_ptr: Address, vdev_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let realm = RealmAt(s, rd);
    let pdev = PdevAt(s, pdev_ptr);
    let num_vdevs_pre = PdevAt(old_s, pdev_ptr).num_vdevs;
    let vdev = VdevAt(s, vdev_ptr);
    let params = RmiVdevParamsAt(old_s, params_ptr);
    let num_aux = VdevAuxCount(old_s, RmiPdevFlagsDecode(old_s, ToBits64(0)), RmiVdevFlagsDecode(old_s, ToBits64(0)));
    
    ((!ImplFeatures().feat_da ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED as int)) &&
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (GranuleAt(old_s, rd).state != RD as int ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (GranuleAt(old_s, pdev_ptr).state != PDEV as int ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (PdevAt(old_s, pdev_ptr).state != PDEV_READY as int ==> ResultEqual(result, RMI_ERROR_DEVICE as int)) &&
    (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (!PaIsDelegableDram(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (GranuleAt(old_s, vdev_ptr).state != DELEGATED as int ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (!GranuleAccessPermitted(params_ptr, PAS_NS as int) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (!RmiVdevParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (realm_pre.feat_da != FEATURE_TRUE as int ==> ResultEqual(result, RMI_ERROR_REALM as int)) &&
    (params.num_aux != num_aux ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (!AuxAligned32(params.aux, params.num_aux) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (AuxAlias32(vdev_ptr, params.aux, params.num_aux) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (!AuxStateEqual32(old_s, params.aux, params.num_aux, DELEGATED as int) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (!VdevIdIsFree(old_s, realm_pre, params.vdev_id) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (!TdiIdIsFree(old_s, params.tdi_id, PdevAt(old_s, pdev_ptr).segment_id) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    ((params.tdi_id as int < PdevAt(old_s, pdev_ptr).rid_base as int || params.tdi_id as int >= PdevAt(old_s, pdev_ptr).rid_top as int) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    ((params.flags.VSMMU == RMI_FEATURE_TRUE as int && !AddrIsGranuleAligned(params.vsmmu_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    ((params.flags.VSMMU == RMI_FEATURE_TRUE as int && !PaIsDelegable(params.vsmmu_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    ((params.flags.VSMMU == RMI_FEATURE_TRUE as int && GranuleAt(old_s, params.vsmmu_addr).state != VSMMU as int) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    ((params.flags.VSMMU == RMI_FEATURE_TRUE as int && !VsidIsFree(old_s, VsmmuAt(old_s, params.vsmmu_addr), params.vsid)) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    ((params.flags.VSMMU == RMI_FEATURE_TRUE as int && !PdevVsmmuIsCompatible(old_s, PdevAt(old_s, pdev_ptr), VsmmuAt(old_s, params.vsmmu_addr))) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (result.is_Ok() ==> 
        pdev.num_vdevs == num_vdevs_pre + 1 &&
        GranuleAt(s, vdev_ptr).state == VDEV as int &&
        vdev.vdev_id == params.vdev_id &&
        vdev.tdi_id == params.tdi_id &&
        vdev.pdev == pdev_ptr &&
        vdev.realm == rd &&
        vdev.vdev_state == VDEV_NEW as int &&
        vdev.dma_state == VDEV_DMA_DISABLED as int &&
        vdev.op == VDEV_OP_UNLOCK as int &&
        vdev.comm_state == DEV_COMM_PENDING as int &&
        AuxEqual32(vdev.aux, params.aux, num_aux) &&
        vdev.num_aux == num_aux &&
        AuxStateEqual32(s, vdev.aux, num_aux, VDEV_AUX as int) &&
        !TdiIdIsFree(s, params.tdi_id, PdevAt(s, pdev_ptr).segment_id) &&
        vdev.vsmmu == params.flags.VSMMU &&
        (params.flags.VSMMU == RMI_FEATURE_TRUE as int ==> vdev.vsmmu_addr == params.vsmmu_addr) &&
        (params.flags.VSMMU == RMI_FEATURE_TRUE as int ==> vdev.vsid == params.vsid) &&
        (params.flags.VSMMU == RMI_FEATURE_TRUE as int ==> !VsidIsFree(s, VsmmuAt(s, params.vsmmu_addr), params.vsid)) &&
        vdev.num_map == 0 &&
        realm.num_vdevs == realm_pre.num_vdevs + 1 &&
        vdev.attest_info.lock_nonce == 0 &&
        vdev.attest_info.meas_nonce == 0 &&
        vdev.attest_info.report_nonce == 0 &&
        vdev.p2p_bound == FEATURE_FALSE as int))
}