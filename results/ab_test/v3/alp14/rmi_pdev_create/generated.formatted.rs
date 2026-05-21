pub open spec fn rmi_pdev_create_spec(
    result: RmiCommandReturnCode,
    pdev_ptr: Address,
    params_ptr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let params = RmiPdevParamsAt(old_s, params_ptr);
    let pdev = PdevAt(old_s, pdev_ptr);
    (!ImplFeatures().feat_da_eq_feature_true() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegableDram(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        pdev_ptr,
    ).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(
        params_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!GranuleAccessPermitted(
        old_s,
        params_ptr,
        PAS_NS,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!RmiPdevParamsIsValid(old_s, params_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!RmiPdevFlagsSupported(old_s, params.flags)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (params.num_aux != PdevAuxCount(
        old_s,
        params.flags,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AuxAligned32(params.aux, params.num_aux)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (AuxAlias32(
        pdev_ptr,
        params.aux,
        params.num_aux,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AuxStateEqual32(
        old_s,
        params.aux,
        params.num_aux,
        DELEGATED,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((ImplFeatures().feat_da_eq_feature_true()
        && AddrIsGranuleAligned(pdev_ptr) && PaIsDelegableDram(pdev_ptr) && GranuleAt(
        old_s,
        pdev_ptr,
    ).state == DELEGATED && AddrIsGranuleAligned(params_ptr) && GranuleAccessPermitted(
        old_s,
        params_ptr,
        PAS_NS,
    ) && RmiPdevParamsIsValid(old_s, params_ptr) && RmiPdevFlagsSupported(old_s, params.flags)
        && params.num_aux == PdevAuxCount(old_s, params.flags) && AuxAligned32(
        params.aux,
        params.num_aux,
    ) && !AuxAlias32(pdev_ptr, params.aux, params.num_aux) && AuxStateEqual32(
        old_s,
        params.aux,
        params.num_aux,
        DELEGATED,
    )) ==> (result == RMI_SUCCESS && GranuleAt(new_s, pdev_ptr).state == PDEV && pdev.pdev_id
        == params.pdev_id && Equal(pdev.spdm, params.flags.spdm) && Equal(
        pdev.ncoh_ide,
        params.flags.ncoh_ide,
    ) && Equal(pdev.ncoh_addr, params.flags.ncoh_addr) && Equal(pdev.coh_ide, params.flags.coh_ide)
        && Equal(pdev.coh_addr, params.flags.coh_addr) && pdev.segment_id == params.segment_id
        && pdev.ecam_addr == params.ecam_addr && pdev.root_id == params.root_id && pdev.cert_id
        == params.cert_id && pdev.rid_base == params.rid_base && pdev.rid_top == params.rid_top
        && Equal(pdev.hash_algo, params.hash_algo) && pdev.ncoh_ide_sid == params.ncoh_ide_sid
        && pdev.ncoh_num_addr_range == params.ncoh_num_addr_range && RmiAddressRangesEqual16(
        new_s,
        pdev.ncoh_addr_range,
        params.ncoh_addr_range,
        params.ncoh_num_addr_range,
    ) && pdev.coh_num_addr_range == params.coh_num_addr_range && RmiAddressRangesEqual4(
        new_s,
        pdev.coh_addr_range,
        params.coh_addr_range,
        params.coh_num_addr_range,
    ) && pdev.state == PDEV_NEW && pdev.comm_state == DEV_COMM_PENDING && pdev.num_vdevs == 0
        && AuxEqual32(pdev.aux, params.aux, PdevAuxCount(old_s, params.flags)) && pdev.num_aux
        == PdevAuxCount(old_s, params.flags) && AuxStateEqual32(
        new_s,
        pdev.aux,
        PdevAuxCount(old_s, params.flags),
        PDEV_AUX,
    ) && Equal(pdev.p2p_enabled, params.flags.p2p) && pdev.p2p_stream_valid == RMM_FALSE))
}