pub open spec fn rmi_pdev_create_spec(result: Result<(), RmiStatusCode>, pdev_ptr: Address, params_ptr: Address, old_s: S, new_s: S) -> bool {
    // da_supp failure
    (!ImplFeatures(old_s).feat_da ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_NOT_SUPPORTED))
    // pdev_align failure
    && (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // pdev_bound failure
    && (!PaIsDelegableDram(pdev_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // pdev_state failure
    && (GranuleAt(old_s, pdev_ptr).state != RmmGranuleState::DELEGATED ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // params_align failure
    && (!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // params_pas failure
    && (!GranuleAccessPermitted(old_s, params_ptr, RmmAddressSpace::PAS_NS) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // params_valid failure
    && (!RmiPdevParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // flags_supp failure
    && (!RmiPdevFlagsSupported(old_s, RmiPdevParamsAt(old_s, params_ptr).flags) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // num_aux failure
    && (RmiPdevParamsAt(old_s, params_ptr).num_aux != VdevAuxCount(old_s, RmiPdevParamsAt(old_s, params_ptr).flags) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // aux_align failure
    && (!AuxAligned32(old_s, RmiPdevParamsAt(old_s, params_ptr).aux, RmiPdevParamsAt(old_s, params_ptr).num_aux) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // aux_alias failure
    && (AuxAlias32(old_s, pdev_ptr, RmiPdevParamsAt(old_s, params_ptr).aux, RmiPdevParamsAt(old_s, params_ptr).num_aux) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // aux_state failure
    && (!AuxStateEqual32(old_s, RmiPdevParamsAt(old_s, params_ptr).aux, RmiPdevParamsAt(old_s, params_ptr).num_aux, RmmGranuleState::DELEGATED) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // Success condition: all preconditions pass
    && (
      (ImplFeatures(old_s).feat_da &&
       AddrIsGranuleAligned(pdev_ptr) &&
       PaIsDelegableDram(pdev_ptr) &&
       GranuleAt(old_s, pdev_ptr).state == RmmGranuleState::DELEGATED &&
       AddrIsGranuleAligned(params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, RmmAddressSpace::PAS_NS) &&
       RmiPdevParamsIsValid(old_s, params_ptr) &&
       RmiPdevFlagsSupported(old_s, RmiPdevParamsAt(old_s, params_ptr).flags) &&
       RmiPdevParamsAt(old_s, params_ptr).num_aux == VdevAuxCount(old_s, RmiPdevParamsAt(old_s, params_ptr).flags) &&
       AuxAligned32(old_s, RmiPdevParamsAt(old_s, params_ptr).aux, RmiPdevParamsAt(old_s, params_ptr).num_aux) &&
       !AuxAlias32(old_s, pdev_ptr, RmiPdevParamsAt(old_s, params_ptr).aux, RmiPdevParamsAt(old_s, params_ptr).num_aux) &&
       AuxStateEqual32(old_s, RmiPdevParamsAt(old_s, params_ptr).aux, RmiPdevParamsAt(old_s, params_ptr).num_aux, RmmGranuleState::DELEGATED))
      ==>
      (result.is_Ok() &&
       GranuleAt(new_s, pdev_ptr).state == RmmGranuleState::PDEV &&
       PdevAt(new_s, pdev_ptr).pdev_id == RmiPdevParamsAt(old_s, params_ptr).pdev_id &&
       PdevAt(new_s, pdev_ptr).spdm == RmiPdevParamsAt(old_s, params_ptr).flags.spdm &&
       PdevAt(new_s, pdev_ptr).ncoh_ide == RmiPdevParamsAt(old_s, params_ptr).flags.ncoh_ide &&
       PdevAt(new_s, pdev_ptr).ncoh_addr == RmiPdevParamsAt(old_s, params_ptr).flags.ncoh_addr &&
       PdevAt(new_s, pdev_ptr).coh_ide == RmiPdevParamsAt(old_s, params_ptr).flags.coh_ide &&
       PdevAt(new_s, pdev_ptr).coh_addr == RmiPdevParamsAt(old_s, params_ptr).flags.coh_addr &&
       PdevAt(new_s, pdev_ptr).segment_id == RmiPdevParamsAt(old_s, params_ptr).segment_id &&
       PdevAt(new_s, pdev_ptr).ecam_addr == RmiPdevParamsAt(old_s, params_ptr).ecam_addr &&
       PdevAt(new_s, pdev_ptr).root_id == RmiPdevParamsAt(old_s, params_ptr).root_id &&
       PdevAt(new_s, pdev_ptr).cert_id == RmiPdevParamsAt(old_s, params_ptr).cert_id &&
       PdevAt(new_s, pdev_ptr).rid_base == RmiPdevParamsAt(old_s, params_ptr).rid_base &&
       PdevAt(new_s, pdev_ptr).rid_top == RmiPdevParamsAt(old_s, params_ptr).rid_top &&
       PdevAt(new_s, pdev_ptr).hash_algo == RmiPdevParamsAt(old_s, params_ptr).hash_algo &&
       PdevAt(new_s, pdev_ptr).ncoh_ide_sid == RmiPdevParamsAt(old_s, params_ptr).ncoh_ide_sid &&
       PdevAt(new_s, pdev_ptr).ncoh_num_addr_range == RmiPdevParamsAt(old_s, params_ptr).ncoh_num_addr_range &&
       RmiAddressRangesEqual16(new_s, PdevAt(new_s, pdev_ptr).ncoh_addr_range, RmiPdevParamsAt(old_s, params_ptr).ncoh_addr_range, RmiPdevParamsAt(old_s, params_ptr).ncoh_num_addr_range) &&
       PdevAt(new_s, pdev_ptr).coh_num_addr_range == RmiPdevParamsAt(old_s, params_ptr).coh_num_addr_range &&
       RmiAddressRangesEqual4(new_s, PdevAt(new_s, pdev_ptr).coh_addr_range, RmiPdevPar