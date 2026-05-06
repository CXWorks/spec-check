pub open spec fn rmi_pdev_create_spec(
    pdev_ptr: Address,
    params_ptr: Address,
    result: Result<(), RmiStatusCode>,
    old_s: S,
    new_s: S,
) -> bool {
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegableDram(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        pdev_ptr,
    ).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(
        old_s,
        params_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!GranuleAccessPermitted(
        old_s,
        params_ptr,
        PAS_NS,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!RmiPdevParamsIsValid(old_s, params_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!RmiPdevFlagsSupported(
        old_s,
        RmiPdevParamsAt(old_s, params_ptr).flags,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (RmiPdevParamsAt(old_s, params_ptr).num_aux
        != PdevAuxCount(old_s, RmiPdevParamsAt(old_s, params_ptr).flags) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (!AuxAligned32(
        old_s,
        RmiPdevParamsAt(old_s, params_ptr).aux,
        RmiPdevParamsAt(old_s, params_ptr).num_aux as int,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (AuxAlias32(
        old_s,
        pdev_ptr,
        RmiPdevParamsAt(old_s, params_ptr).aux,
        RmiPdevParamsAt(old_s, params_ptr).num_aux as int,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AuxStateEqual32(
        old_s,
        RmiPdevParamsAt(old_s, params_ptr).aux,
        RmiPdevParamsAt(old_s, params_ptr).num_aux as int,
        DELEGATED,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (result.is_Ok() ==> GranuleAt(
        new_s,
        pdev_ptr,
    ).state == PDEV) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).pdev_id == RmiPdevParamsAt(
        new_s,
        params_ptr,
    ).pdev_id) && (result.is_Ok() ==> Equal(
        PdevAt(new_s, pdev_ptr).spdm,
        RmiPdevParamsAt(new_s, params_ptr).flags.spdm,
    )) && (result.is_Ok() ==> Equal(
        PdevAt(new_s, pdev_ptr).ncoh_ide,
        RmiPdevParamsAt(new_s, params_ptr).flags.ncoh_ide,
    )) && (result.is_Ok() ==> Equal(
        PdevAt(new_s, pdev_ptr).ncoh_addr,
        RmiPdevParamsAt(new_s, params_ptr).flags.ncoh_addr,
    )) && (result.is_Ok() ==> Equal(
        PdevAt(new_s, pdev_ptr).coh_ide,
        RmiPdevParamsAt(new_s, params_ptr).flags.coh_ide,
    )) && (result.is_Ok() ==> Equal(
        PdevAt(new_s, pdev_ptr).coh_addr,
        RmiPdevParamsAt(new_s, params_ptr).flags.coh_addr,
    )) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).segment_id == RmiPdevParamsAt(
        new_s,
        params_ptr,
    ).segment_id) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).ecam_addr == RmiPdevParamsAt(
        new_s,
        params_ptr,
    ).ecam_addr) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).root_id == RmiPdevParamsAt(
        new_s,
        params_ptr,
    ).root_id) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).cert_id == RmiPdevParamsAt(
        new_s,
        params_ptr,
    ).cert_id) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).rid_base == RmiPdevParamsAt(
        new_s,
        params_ptr,
    ).rid_base) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).rid_top == RmiPdevParamsAt(
        new_s,
        params_ptr,
    ).rid_top) && (result.is_Ok() ==> Equal(
        PdevAt(new_s, pdev_ptr).hash_algo,
        RmiPdevParamsAt(new_s, params_ptr).hash_algo,
    )) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).ncoh_ide_sid == RmiPdevParamsAt(
        new_s,
        params_ptr,
    ).ncoh_ide_sid) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).ncoh_num_addr_range
        == RmiPdevParamsAt(new_s, params_ptr).ncoh_num_addr_range) && (result.is_Ok()
        ==> RmiAddressRangesEqual16(
        new_s,
        PdevAt(new_s, pdev_ptr).ncoh_addr_range,
        RmiPdevParamsAt(new_s, params_ptr).ncoh_addr_range,
        RmiPdevParamsAt(new_s, params_ptr).ncoh_num_addr_range as int,
    )) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).coh_num_addr_range == RmiPdevParamsAt(
        new_s,
        params_ptr,
    ).coh_num_addr_range) && (result.is_Ok() ==> RmiAddressRangesEqual4(
        new_s,
        PdevAt(new_s, pdev_ptr).coh_addr_range,
        RmiPdevParamsAt(new_s, params_ptr).coh_addr_range,
        RmiPdevParamsAt(new_s, params_ptr).coh_num_addr_range as int,
    )) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEW) && (result.is_Ok()
        ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING) && (result.is_Ok() ==> PdevAt(
        new_s,
        pdev_ptr,
    ).num_vdevs == 0) && (result.is_Ok() ==> AuxEqual32(
        new_s,
        PdevAt(new_s, pdev_ptr).aux,
        RmiPdevParamsAt(new_s, params_ptr).aux,
        PdevAuxCount(new_s, RmiPdevParamsAt(new_s, params_ptr).flags),
    )) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).num_aux == PdevAuxCount(
        new_s,
        RmiPdevParamsAt(new_s, params_ptr).flags,
    )) && (result.is_Ok() ==> AuxStateEqual32(
        new_s,
        PdevAt(new_s, pdev_ptr).aux,
        PdevAuxCount(new_s, RmiPdevParamsAt(new_s, params_ptr).flags) as int,
        PDEV_AUX,
    )) && (result.is_Ok() ==> Equal(
        PdevAt(new_s, pdev_ptr).p2p_enabled,
        RmiPdevParamsAt(new_s, params_ptr).flags.p2p,
    )) && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).p2p_stream_valid == RMM_FALSE) && ((!(
    ImplFeatures(old_s).feat_da != FEATURE_TRUE) && AddrIsGranuleAligned(old_s, pdev_ptr)
        && PaIsDelegableDram(old_s, pdev_ptr) && !(GranuleAt(old_s, pdev_ptr).state != DELEGATED)
        && AddrIsGranuleAligned(old_s, params_ptr) && GranuleAccessPermitted(
        old_s,
        params_ptr,
        PAS_NS,
    ) && RmiPdevParamsIsValid(old_s, params_ptr) && RmiPdevFlagsSupported(
        old_s,
        RmiPdevParamsAt(old_s, params_ptr).flags,
    ) && !(RmiPdevParamsAt(old_s, params_ptr).num_aux != PdevAuxCount(
        old_s,
        RmiPdevParamsAt(old_s, params_ptr).flags,
    )) && AuxAligned32(
        old_s,
        RmiPdevParamsAt(old_s, params_ptr).aux,
        RmiPdevParamsAt(old_s, params_ptr).num_aux as int,
    ) && !(AuxAlias32(
        old_s,
        pdev_ptr,
        RmiPdevParamsAt(old_s, params_ptr).aux,
        RmiPdevParamsAt(old_s, params_ptr).num_aux as int,
    )) && AuxStateEqual32(
        old_s,
        RmiPdevParamsAt(old_s, params_ptr).aux,
        RmiPdevParamsAt(old_s, params_ptr).num_aux as int,
        DELEGATED,
    )) ==> result.is_Ok()) && (result.is_Err() ==> GranuleAt(new_s, pdev_ptr).state == GranuleAt(
        old_s,
        pdev_ptr,
    ).state) && (result.is_Err() ==> PdevAt(new_s, pdev_ptr).pdev_id == PdevAt(
        old_s,
        pdev_ptr,
    ).pdev_id) && (result.is_Err() ==> PdevAt(new_s, pdev_ptr).segment_id == PdevAt(
        old_s,
        pdev_ptr,
    ).segment_id) && (result.is_Err() ==> PdevAt(new_s, pdev_ptr).ecam_addr == PdevAt(
        old_s,
        pdev_ptr,
    ).ecam_addr) && (result.is_Err() ==> PdevAt(new_s, pdev_ptr).root_id == PdevAt(
        old_s,
        pdev_ptr,
    ).root_id) && (result.is_Err() ==> PdevAt(new_s, pdev_ptr).cert_id == PdevAt(
        old_s,
        pdev_ptr,
    ).cert_id) && (result.is_Err() ==> PdevAt(new_s, pdev_ptr).rid_base == PdevAt(
        old_s,
        pdev_ptr,
    ).rid_base) && (result.is_Err() ==> PdevAt(new_s, pdev_ptr).rid_top == PdevAt(
        old_s,
        pdev_ptr,
    ).rid_top) && (result.is_Err() ==> PdevAt(new_s, pdev_ptr).ncoh_ide_sid == PdevAt(
        old_s,
        pdev_ptr,
    ).ncoh_ide_sid) && (result.is_Err() ==> PdevAt(new_s, pdev_ptr).ncoh_num_addr_range == PdevAt(
        old_s,
        pdev_ptr,
    ).ncoh_num_addr_range) && (result.is_Err() ==> PdevAt(new_s, pdev_ptr).coh_num_addr_range
        == PdevAt(old_s, pdev_ptr).coh_num_addr_range) && (result.is_Err() ==> PdevAt(
        new_s,
        pdev_ptr,
    ).state == PdevAt(old_s, pdev_ptr).state) && (result.is_Err() ==> PdevAt(
        new_s,
        pdev_ptr,
    ).comm_state == PdevAt(old_s, pdev_ptr).comm_state) && (result.is_Err() ==> PdevAt(
        new_s,
        pdev_ptr,
    ).num_vdevs == PdevAt(old_s, pdev_ptr).num_vdevs) && (result.is_Err() ==> PdevAt(
        new_s,
        pdev_ptr,
    ).num_aux == PdevAt(old_s, pdev_ptr).num_aux) && (result.is_Err() ==> PdevAt(
        new_s,
        pdev_ptr,
    ).p2p_stream_valid == PdevAt(old_s, pdev_ptr).p2p_stream_valid)
}