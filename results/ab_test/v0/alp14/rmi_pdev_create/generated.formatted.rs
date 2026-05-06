pub open spec fn RMI_PDEV_CREATE_spec(s: S, pdev_ptr: Address, params_ptr: Address) -> bool {
    let pdev = PdevAt(s, pdev_ptr);
    let params = RmiPdevParamsAt(s, params_ptr);
    let granule_pdev = GranuleAt(s, pdev_ptr);

    // Failure conditions (checked in order)
    (ImplFeatures(s).feat_da != FEATURE_TRUE) ==> false  // da_supp
     && (AddrIsGranuleAligned(s, pdev_ptr))  // pdev_align
     && (PaIsDelegableDram(s, pdev_ptr))  // pdev_bound
     && (granule_pdev.state == DELEGATED)  // pdev_state
     && (AddrIsGranuleAligned(s, params_ptr))  // params_align
     && (GranuleAccessPermitted(s, params_ptr, PAS_NS))  // params_pas
     && (RmiPdevParamsIsValid(s, params_ptr))  // params_valid
     && (RmiPdevFlagsSupported(s, params.flags))  // flags_supp
     && (params.num_aux == PdevAuxCount(s, params.flags))  // num_aux
     && (AuxAligned32(s, params.aux, params.num_aux))  // aux_align
     && (!AuxAlias32(s, pdev_ptr, params.aux, params.num_aux))  // aux_alias
     && (AuxStateEqual32(s, params.aux, params.num_aux, DELEGATED))  // aux_state
    // Success conditions
     && (GranuleAt(s, pdev_ptr).state == PDEV)  // gran_state
     && (pdev.pdev_id == params.pdev_id)  // pdev_id
     && (Equal(pdev.spdm, params.flags.spdm))  // spdm
     && (Equal(pdev.ncoh_ide, params.flags.ncoh_ide))  // ncoh_ide
     && (Equal(pdev.ncoh_addr, params.flags.ncoh_addr))  // ncoh_addr
     && (Equal(pdev.coh_ide, params.flags.coh_ide))  // coh_ide
     && (Equal(pdev.coh_addr, params.flags.coh_addr))  // coh_addr
     && (pdev.segment_id == params.segment_id)  // segment_id
     && (pdev.ecam_addr == params.ecam_addr)  // ecam_addr
     && (pdev.root_id == params.root_id)  // root_id
     && (pdev.cert_id == params.cert_id)  // cert_id
     && (pdev.rid_base == params.rid_base)  // rid_base
     && (pdev.rid_top == params.rid_top)  // rid_top
     && (Equal(pdev.hash_algo, params.hash_algo))  // hash_algo
     && (pdev.ncoh_ide_sid == params.ncoh_ide_sid)  // ncoh_ide_sid
     && (pdev.ncoh_num_addr_range
        == params.ncoh_num_addr_range)  // ncoh_num_addr_range
     && (RmiAddressRangesEqual16(
        s,
        pdev.ncoh_addr_range,
        params.ncoh_addr_range,
        params.ncoh_num_addr_range,
    ))  // ncoh_addr_range
     && (pdev.coh_num_addr_range == params.coh_num_addr_range)  // coh_num_addr_range
     && (RmiAddressRangesEqual4(
        s,
        pdev.coh_addr_range,
        params.coh_addr_range,
        params.coh_num_addr_range,
    ))  // coh_addr_range
     && (pdev.state == PDEV_NEW)  // state
     && (pdev.comm_state == DEV_COMM_PENDING)  // comm_state
     && (pdev.num_vdevs == 0)  // num_vdevs
     && (AuxEqual32(s, pdev.aux, params.aux, PdevAuxCount(s, params.flags)))  // aux
     && (pdev.num_aux == PdevAuxCount(s, params.flags))  // num_aux
     && (AuxStateEqual32(
        s,
        pdev.aux,
        PdevAuxCount(s, params.flags),
        PDEV_AUX,
    ))  // aux_state
     && (Equal(pdev.p2p_enabled, params.flags.p2p))  // p2p_enabled
     && (pdev.p2p_stream_valid == RMM_FALSE)  // p2p_stream_valid

}