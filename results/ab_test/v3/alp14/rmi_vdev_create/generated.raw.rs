pub open spec fn rmi_vdev_create_spec(result: RmiCommandReturnCode, old_s: S, new_s: S, rd: Address, pdev_ptr: Address, vdev_ptr: Address, params_ptr: Address) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let realm = RealmAt(new_s, rd);
    let pdev = PdevAt(old_s, pdev_ptr);
    let num_vdevs_pre = pdev.num_vdevs;
    let vdev = VdevAt(new_s, vdev_ptr);
    let params = RmiVdevParamsAt(old_s, params_ptr);
    let num_aux = VdevAuxCount(old_s, PdevFlags(old_s, pdev), params.flags);

    (!ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (pdev.state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RmiVdevParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm_pre.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_REALM))
    && (params.num_aux != num_aux ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AuxAligned32(old_s, params.aux, params.num_aux) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (AuxAlias32(old_s, vdev_ptr, params.aux, params.num_aux) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AuxStateEqual32(old_s, params.aux, params.num_aux, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!VdevIdIsFree(old_s, realm_pre, params.vdev_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!TdiIdIsFree(old_s, params.tdi_id, pdev.segment_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((UInt(params.tdi_id) < UInt(pdev.rid_base)) || (UInt(params.tdi_id) >= UInt(pdev.rid_top))) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((params.flags.VSMMU == RMI_FEATURE_TRUE && !AddrIsGranuleAligned(old_s, params.vsmmu_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((params.flags.VSMMU == RMI_FEATURE_TRUE && !PaIsDelegable(old_s, params.vsmmu_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((params.flags.VSMMU == RMI_FEATURE_TRUE && GranuleAt(old_s, params.vsmmu_addr).state != VSMMU) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((params.flags.VSMMU == RMI_FEATURE_TRUE && !VsidIsFree(old_s, VsmmuAt(old_s, params.vsmmu_addr), params.vsid)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((params.flags.VSMMU == RMI_FEATURE_TRUE && !PdevVsmmuIsCompatible(old_s, pdev, VsmmuAt(old_s, params.vsmmu_addr))) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ImplFeatures(old_s).feat_da == FEATURE_TRUE && AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD && AddrIsGranuleAligned(old_s, pdev_ptr) && PaIsDelegable(old_s, pdev_ptr) && GranuleAt(old_s, pdev_ptr).state == PDEV && pdev.state == PDEV_READY && AddrIsGranuleAligned(old_s, vdev_ptr) && PaIsDelegableDram(old_s, vdev_ptr) && GranuleAt(old_s, vdev_ptr).state == DELEGATED && AddrIsGranuleAligned(old_s, params_ptr) && GranuleAccessPermitted(old_s, params_ptr, PAS_NS) && RmiVdevParamsIsValid(old_s, params_ptr) && realm_pre.feat_da == FEATURE_TRUE && params.num_aux == num_aux && AuxAligned32(old_s, params.aux, params.num_aux) && !AuxAlias32(old_s, vdev_ptr, params.aux, params.num_aux) && AuxStateEqual32(old_s, params.aux, params.num_aux, DELEGATED) && VdevIdIsFree(old_s, realm_pre, params.vdev_id) && TdiIdIsFree(old_s, params.tdi_id, pdev.segment_id) && UInt(params.tdi_id) >= UInt(pdev.rid_base) && UInt(params.tdi_id) < UInt(pdev.rid_top) && (params.flags.VSMMU != RMI_FEATURE_TRUE || (AddrIsGranuleAligned(old_s, params.vsmmu_addr) && PaIsDelegable(old_s, params.vsmmu_addr) && GranuleAt(old_s, params.vsmmu_addr).state == VSMMU && VsidIsFree(old_s, VsmmuAt(old_s, params.vsmmu_addr), params.vsid) && PdevVsmmuIsCompatible(old_s, pdev, VsmmuAt(old_s, params.vsmmu_addr)))) ==> (result.is_Ok() && GranuleAt(new_s, vdev_ptr).state == VDEV && vdev.vdev_id == params.vdev_id && vdev.tdi_id == params.tdi_id && vdev.pdev == pdev_ptr && vdev.realm == rd && vdev.vdev_state == VDEV_NEW && v