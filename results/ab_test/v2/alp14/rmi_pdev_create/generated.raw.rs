```verus
pub open spec fn rmi_pdev_create_spec(result: RmiCommandReturnCode, old_s: S, new_s: S, 
                                      pdev_ptr: Address, params_ptr: Address) -> bool {
    // Failure conditions
    let da_not_supported = !old_s.impl_features().feat_da.is_FEATURE_TRUE();
    let pdev_not_aligned = !AddrIsGranuleAligned(pdev_ptr);
    let pdev_not_delegable = !PaIsDelegableDram(pdev_ptr);
    let pdev_not_delegated = GranuleAt(old_s, pdev_ptr).state != DELEGATED;
    let params_not_aligned = !AddrIsGranuleAligned(params_ptr);
    let params_no_access = !GranuleAccessPermitted(old_s, params_ptr, PAS_NS);
    let params = RmiPdevParamsAt(old_s, params_ptr);
    let params_not_valid = !RmiPdevParamsIsValid(old_s, params_ptr);
    let flags_not_supported = !RmiPdevFlagsSupported(old_s, params.flags);
    let num_aux_mismatch = params.num_aux != VdevAuxCount(old_s, params.flags, RmiVdevFlags::default());
    let aux_not_aligned = !AuxAligned32(old_s, params.aux, params.num_aux);
    let aux_aliased = AuxAlias32(old_s, pdev_ptr, params.aux, params.num_aux);
    let aux_not_delegated = !AuxStateEqual32(old_s, params.aux, params.num_aux, DELEGATED);

    let pdev = PdevAt(old_s, pdev_ptr);

    // Failure condition ordering: da_supp has priority
    let result_matches_failure = 
        (da_not_supported ==> result == RMI_ERROR_NOT_SUPPORTED) &&
        (!da_not_supported ==> (
            (pdev_not_aligned || pdev_not_delegable || pdev_not_delegated || params_not_aligned ||
             params_no_access || params_not_valid || flags_not_supported || num_aux_mismatch ||
             aux_not_aligned || aux_aliased || aux_not_delegated)
            ==> result == RMI_ERROR_INPUT
        ));

    // Success conditions
    let success_conditions = 
        (!da_not_supported && !pdev_not_aligned && !pdev_not_delegable && !pdev_not_delegated &&
         !params_not_aligned && !params_no_access && !params_not_valid && !flags_not_supported &&
         !num_aux_mismatch && !aux_not_aligned && !aux_aliased && !aux_not_delegated) ==> (
            result == RMI_SUCCESS &&
            GranuleAt(new_s, pdev_ptr).state == PDEV &&
            pdev.pdev_id == params.pdev_id &&
            pdev.spdm == params.flags.spdm &&
            pdev.ncoh_ide == params.flags.ncoh_ide &&
            pdev.ncoh_addr == params.flags.ncoh_addr &&
            pdev.coh_ide == params.flags.coh_ide &&
            pdev.coh_addr == params.flags.coh_addr &&
            pdev.segment_id == params.segment_id &&
            pdev.ecam_addr == params.ecam_addr &&
            pdev.root_id == params.root_id &&
            pdev.cert_id == params.cert_id &&
            pdev.rid_base == params.rid_base &&
            pdev.rid_top == params.rid_top &&
            pdev.hash_algo == params.hash_algo &&
            pdev.ncoh_ide_sid == params.ncoh_ide_sid &&
            pdev.ncoh_num_addr_range == params.ncoh_num_addr_range &&
            RmiAddressRangesEqual16(old_s, pdev.ncoh_addr_range, params.ncoh_addr_range, 
                                    params.ncoh_num_addr_range) &&
            pdev.coh_num_addr_range == params.coh_num_addr_range &&
            RmiAddressRangesEqual4(old_s, pdev.coh_addr_range, params.coh_addr_range,
                                   params.coh_num_addr_range) &&
            pdev.state == PDEV_NEW &&
            pdev.comm_state == DEV_COMM_PENDING &&
            pdev.num_vdevs == 0 &&
            AuxEqual32(old_s, pdev.aux, params.aux, VdevAuxCount(old_s, params.flags, RmiVdevFlags::default())) &&
            pdev.num_aux == VdevAuxCount(old_s, params.flags, RmiVdevFlags::default()) &&
            AuxStateEqual32(old_s, pdev.aux, VdevAuxCount(old_s, params.flags, RmiVdevFlags::default()), PDEV_AUX) &&
            pdev.p2p_enabled == params.flags.p2p &&
            pdev.p2p_stream_valid == RMM_FALSE
        );

    result_matches_failure && success_conditions
}
```