pub open spec fn rmi_vdev_create_spec(
    result: RmiCommandReturnCode,
    rd: Address,
    pdev_ptr: Address,
    vdev_ptr: Address,
    params_ptr: Address,
    old_s: S,
    new_s: S
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let pdev = PdevAt(old_s, pdev_ptr);
    let num_vdevs_pre = pdev.num_vdevs;
    let vdev = VdevAt(old_s, vdev_ptr);
    let params = RmiVdevParamsAt(old_s, params_ptr);
    let num_aux = VdevAuxCount(old_s, RmiPdevFlagsDecode(old_s, pdev.flags), RmiVdevFlagsDecode(old_s, params.flags));
    
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (pdev.state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RmiVdevParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm_pre.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_REALM))
    && (params.num_aux != num_aux ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AuxAligned32(old_s, params.aux, params.num_aux) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (AuxAlias32(old_s, vdev_ptr, params.aux, params.num_aux) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AuxStateEqual32(old_s, params.aux, params.num_aux, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!VdevIdIsFree(old_s, realm_pre, params.vdev_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!TdiIdIsFree(old_s, params.tdi_id, pdev.segment_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((params.tdi_id as int) < (pdev.rid_base as int) || (params.tdi_id as int) >= (pdev.rid_top as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((params.flags.VSMMU == RMI_FEATURE_TRUE && !AddrIsGranuleAligned(params.vsmmu_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((params.flags.VSMMU == RMI_FEATURE_TRUE && !PaIsDelegable(params.vsmmu_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((params.flags.VSMMU == RMI_FEATURE_TRUE && GranuleAt(old_s, params.vsmmu_addr).state != VSMMU) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((params.flags.VSMMU == RMI_FEATURE_TRUE && !VsidIsFree(old_s, VsmmuAt(old_s, params.vsmmu_addr), params.vsid)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((params.flags.VSMMU == RMI_FEATURE_TRUE && !PdevVsmmuIsCompatible(old_s, pdev, VsmmuAt(old_s, params.vsmmu_addr))) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (
        (result.is_Ok() ==>
            PdevAt(new_s, pdev_ptr).num_vdevs == num_vdevs_pre + 1
            && GranuleAt(new_s, vdev_ptr).state == VDEV
            && VdevAt(new_s, vdev_ptr).vdev_id == params.vdev_id
            && VdevAt(new_s, vdev_ptr).tdi_id == params.tdi_id
            && VdevAt(new_s, vdev_ptr).pdev == pdev_ptr
            && VdevAt(new_s, vdev_ptr).realm == rd
            && VdevAt(new_s, vdev_ptr).vdev_state == VDEV_NEW
            && VdevAt(new_s, vdev_ptr).dma_state == VDEV_DMA_DISABLED
            && VdevAt(new_s, vdev_ptr).op == VDEV_OP_UNLOCK
            && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING
            && AuxEqual32(old_s, VdevAt(new_s, vdev_ptr).aux, params.aux, num_aux)
            && VdevAt(new_s, vdev_ptr).num_aux == num_aux
            && AuxStateEqual32(old_s, VdevAt(new_s, vdev_ptr).aux, num_aux, VDEV_AUX)
            && !TdiIdIsFree(new_s, params.tdi_id, pdev.segment_id)
            && Equal(old_s, VdevAt(new_s, vdev_ptr).vsmmu, params.flags.VSMMU)
            && (params.flags.VSMMU == RMI_FEATURE_TRUE ==> VdevAt(new_s, vdev_ptr).vsmmu_addr == params.vsmmu_addr)
            && (params.flags.VSMMU == RMI_FEATURE_TRUE ==> VdevAt(new_s, vdev_ptr).vsid == params.vsid)
            && (params.flags.VSMMU == RMI_FEATURE_TRUE ==> !VsidIsFree(new_s, VsmmuAt(new_s, params.vsmmu_addr), params.vsid))
            && VdevAt(new_s, vdev_ptr).num_map == 0
            && RealmAt(new_s, rd).num_vdevs == realm_pre.num_vdevs + 1
            && VdevAt(new_s, vdev_ptr).attest_info.lock_nonce == 0
            && VdevAt(new_s, vdev_ptr).attest_info.meas_nonce == 0
            && Vdev