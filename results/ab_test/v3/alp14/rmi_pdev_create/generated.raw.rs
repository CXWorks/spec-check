```verus
pub open spec fn RMI_PDEV_CREATE_spec(
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
    params_ptr: Address,
    result: RmiCommandReturnCode,
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let params = RmiPdevParamsAt(old_s, params_ptr);
    
    (
        // Failure: da_supp
        (!ImplFeatures(old_s).feat_da.is_FEATURE_TRUE()) ==> 
            ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)
    ) && (
        // Failure: pdev_align
        (!AddrIsGranuleAligned(pdev_ptr)) ==> 
            ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure: pdev_bound
        (!PaIsDelegableDram(old_s, pdev_ptr)) ==> 
            ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure: pdev_state
        (GranuleAt(old_s, pdev_ptr).state != RmmGranuleState::DELEGATED) ==> 
            ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure: params_align
        (!AddrIsGranuleAligned(params_ptr)) ==> 
            ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure: params_pas
        (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS)) ==> 
            ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure: params_valid
        (!RmiPdevParamsIsValid(old_s, params_ptr)) ==> 
            ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure: flags_supp
        (!RmiPdevFlagsSupported(old_s, params.flags)) ==> 
            ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure: num_aux
        (params.num_aux != VdevAuxCount(old_s, params.flags, RmiVdevFlags::default())) ==> 
            ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure: aux_align
        (!AuxAligned32(old_s, params.aux, params.num_aux)) ==> 
            ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure: aux_alias
        (AuxAlias32(old_s, pdev_ptr, params.aux, params.num_aux)) ==> 
            ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure: aux_state
        (!AuxStateEqual32(old_s, params.aux, params.num_aux, RmmGranuleState::DELEGATED)) ==> 
            ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Success conditions - all must hold when no failures occur
        (ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() &&
         AddrIsGranuleAligned(pdev_ptr) &&
         PaIsDelegableDram(old_s, pdev_ptr) &&
         GranuleAt(old_s, pdev_ptr).state == RmmGranuleState::DELEGATED &&
         AddrIsGranuleAligned(params_ptr) &&
         GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
         RmiPdevParamsIsValid(old_s, params_ptr) &&
         RmiPdevFlagsSupported(old_s, params.flags) &&
         params.num_aux == VdevAuxCount(old_s, params.flags, RmiVdevFlags::default()) &&
         AuxAligned32(old_s, params.aux, params.num_aux) &&
         !AuxAlias32(old_s, pdev_ptr, params.aux, params.num_aux) &&
         AuxStateEqual32(old_s, params.aux, params.num_aux, RmmGranuleState::DELEGATED)
        ) ==> (
            result.is_Ok() &&
            GranuleAt(new_s, pdev_ptr).state == RmmGranuleState::PDEV &&
            PdevAt(new_s, pdev_ptr).pdev_id == params.pdev_id &&
            PdevAt(new_s, pdev_ptr).spdm == params.flags.spdm &&
            PdevAt(new_s, pdev_ptr).ncoh_ide == params.flags.ncoh_ide &&
            PdevAt(new_s, pdev_ptr).ncoh_addr == params.flags.ncoh_addr &&
            PdevAt(new_s, pdev_ptr).coh_ide == params.flags.coh_ide &&
            PdevAt(new_s, pdev_ptr).coh_addr == params.flags.coh_addr &&
            PdevAt(new_s, pdev_ptr).segment_id == params.segment_id &&
            PdevAt(new_s, pdev_ptr).ecam_addr == params.ecam_addr &&
            PdevAt(new_s, pdev_ptr).root_id == params.root_id &&
            PdevAt(new_s, pdev_ptr).cert_id == params.cert_id &&
            PdevAt(new_s, pdev_ptr).rid_base == params.rid_base &&
            PdevAt(new_s, pdev_ptr).rid_top == params.rid_top &&
            PdevAt(new_s, pdev_ptr).hash_algo == params.hash_algo &&
            PdevAt(new_s, pdev_ptr).ncoh_ide_sid == params.ncoh_ide_sid &&
            PdevAt(new_s, pdev_ptr).ncoh_num_addr_range == params.ncoh_num_addr_range &&
            RmiAddressRangesEqual16(new_s, PdevAt(new_s, pdev_ptr).ncoh_addr_range, params.ncoh_addr_range, params.ncoh_num_addr_range) &&
            PdevAt(new_s, pdev_ptr).coh_num_addr_range == params.coh_num_addr_range &&
            RmiAddressRangesEqual4(new_s, PdevAt(new_s, pdev_ptr).coh_addr_range, params.coh_addr_range, params.coh_num_addr_range) &&
            PdevAt(new_s, pdev_ptr).state == RmmPdevState::PDEV_NEW &&
            PdevAt(new_s, pdev_ptr).comm_state == RmmDevCommState::DEV_COMM_PENDING &&
            PdevAt(new_s, pdev_ptr).num_vdevs == 0 &&
            AuxEqual32(new_s, PdevAt(new_s, pdev_ptr).aux, params.aux, VdevAuxCount(old_s, params.flags, RmiVdevFlags::default())) &&
            PdevAt(new_s, pdev_ptr).num_aux == VdevAuxCount(old_s, params.flags, RmiVdevFlags::default()) &&
            AuxStateEqual32(new_s, PdevAt(new_s, pdev_ptr).aux, VdevAuxCount(old_s, params.flags, RmiVdevFlags::default()), RmmGranuleState::PDEV_AUX) &&
            PdevAt(new_s, pdev_ptr).p2p_enabled == params.flags.p2p &&
            PdevAt(new_s, pdev_ptr).p2p_stream_valid == false
        )
    )
}
```