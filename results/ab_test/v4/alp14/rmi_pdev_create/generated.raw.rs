pub open spec fn RMI_PDEV_CREATE_spec(
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
    params_ptr: Address,
    result: RmiCommandReturnCode
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let params = RmiPdevParamsAt(old_s, params_ptr);
    let params_flags = params.flags;
    
    (
        (!ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
        (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!PaIsDelegableDram(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (GranuleAt(old_s, pdev_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!RmiPdevParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!RmiPdevFlagsSupported(old_s, params_flags) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (params.num_aux != VdevAuxCount(old_s, params_flags) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!AuxAligned32(old_s, params.aux, params.num_aux) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (AuxAlias32(old_s, pdev_ptr, params.aux, params.num_aux) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!AuxStateEqual32(old_s, params.aux, params.num_aux, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (result.is_Ok() ==> (
            GranuleAt(new_s, pdev_ptr).state == PDEV &&
            pdev.pdev_id == params.pdev_id &&
            pdev.spdm == params_flags.spdm &&
            pdev.ncoh_ide == params_flags.ncoh_ide &&
            pdev.ncoh_addr == params_flags.ncoh_addr &&
            pdev.coh_ide == params_flags.coh_ide &&
            pdev.coh_addr == params_flags.coh_addr &&
            pdev.segment_id == params.segment_id &&
            pdev.ecam_addr == params.ecam_addr &&
            pdev.root_id == params.root_id &&
            pdev.cert_id == params.cert_id &&
            pdev.rid_base == params.rid_base &&
            pdev.rid_top == params.rid_top &&
            pdev.hash_algo == params.hash_algo &&
            pdev.ncoh_ide_sid == params.ncoh_ide_sid &&
            pdev.ncoh_num_addr_range == params.ncoh_num_addr_range &&
            RmiAddressRangesEqual16(old_s, pdev.ncoh_addr_range, params.ncoh_addr_range, params.ncoh_num_addr_range) &&
            pdev.coh_num_addr_range == params.coh_num_addr_range &&
            RmiAddressRangesEqual4(old_s, pdev.coh_addr_range, params.coh_addr_range, params.coh_num_addr_range) &&
            pdev.state == PDEV_NEW &&
            pdev.comm_state == DEV_COMM_PENDING &&
            pdev.num_vdevs == 0 &&
            AuxEqual32(old_s, pdev.aux, params.aux, VdevAuxCount(old_s, params_flags)) &&
            pdev.num_aux == VdevAuxCount(old_s, params_flags) &&
            AuxStateEqual32(old_s, pdev.aux, VdevAuxCount(old_s, params_flags), PDEV_AUX) &&
            pdev.p2p_enabled == params_flags.p2p &&
            pdev.p2p_stream_valid == RMM_FALSE
        ))
    )
}