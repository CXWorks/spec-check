pub open spec fn rmi_vdev_create_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_ptr).state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RmiVdevParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RmiVdevParamsAt(old_s, params_ptr).num_aux != VdevAuxCount(old_s, PdevFlags(old_s, PdevAt(old_s, pdev_ptr)),RmiVdevParamsAt(old_s, params_ptr).flags) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AuxAligned32(old_s, RmiVdevParamsAt(old_s, params_ptr).aux, RmiVdevParamsAt(old_s, params_ptr).num_aux as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AuxAlias32(old_s, vdev_ptr, RmiVdevParamsAt(old_s, params_ptr).aux, RmiVdevParamsAt(old_s, params_ptr).num_aux as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AuxStateEqual32(old_s, RmiVdevParamsAt(old_s, params_ptr).aux, RmiVdevParamsAt(old_s, params_ptr).num_aux as int, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!VdevIdIsFree(old_s, RealmAt(old_s, rd), RmiVdevParamsAt(old_s, params_ptr).vdev_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!TdiIdIsFree(old_s, RmiVdevParamsAt(old_s, params_ptr).tdi_id, PdevAt(old_s, pdev_ptr).segment_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((RmiVdevParamsAt(old_s, params_ptr).tdi_id) < (PdevAt(old_s, pdev_ptr).rid_base) || (RmiVdevParamsAt(old_s, params_ptr).tdi_id) >= (PdevAt(old_s, pdev_ptr).rid_top)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !AddrIsGranuleAligned(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PaIsDelegable(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && GranuleAt(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr).state != VSMMU) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !VsidIsFree(old_s, VsmmuAt(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr),RmiVdevParamsAt(old_s, params_ptr).vsid)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PdevVsmmuIsCompatible(old_s, PdevAt(old_s, pdev_ptr),VsmmuAt(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(new_s, pdev_ptr).num_vdevs + 1)
  && (result.is_Ok() ==> GranuleAt(new_s, vdev_ptr).state == VDEV)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).vdev_id == RmiVdevParamsAt(new_s, params_ptr).vdev_id)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).tdi_id == RmiVdevParamsAt(new_s, params_ptr).tdi_id)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).pdev == pdev_ptr)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).realm == rd)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_NEW)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).dma_state == VDEV_DMA_DISABLED)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_UNLOCK)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> AuxEqual32(new_s, VdevAt(new_s, vdev_ptr).aux, RmiVdevParamsAt(new_s, params_ptr).aux, VdevAuxCount(new_s, PdevFlags(new_s, PdevAt(new_s, pdev_ptr)),RmiVdevParamsAt(new_s, params_ptr).flags)))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).num_aux == VdevAuxCount(new_s, PdevFlags(new_s, PdevAt(new_s, pdev_ptr)),RmiVdevParamsAt(new_s, params_ptr).flags))
  && (result.is_Ok() ==> AuxStateEqual32(new_s, VdevAt(new_s, vdev_ptr).aux, VdevAuxCount(new_s, PdevFlags(new_s, PdevAt(new_s, pdev_ptr)),RmiVdevParamsAt(new_s, params_ptr).flags) as int, VDEV_AUX))
  && (result.is_Ok() ==> !TdiIdIsFree(new_s, RmiVdevParamsAt(new_s, params_ptr).tdi_id, PdevAt(new_s, pdev_ptr).segment_id))
  && (result.is_Ok() ==> Equal(VdevAt(new_s, vdev_ptr).vsmmu, RmiVdevParamsAt(new_s, params_ptr).flags.VSMMU))
  && (result.is_Ok() && RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE ==> VdevAt(new_s, vdev_ptr).vsmmu_addr == RmiVdevParamsAt(new_s, params_ptr).vsmmu_addr)
  && (result.is_Ok() && RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE ==> VdevAt(new_s, vdev_ptr).vsid == RmiVdevParamsAt(new_s, params_ptr).vsid)
  && (result.is_Ok() && RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE ==> !VsidIsFree(new_s,  VsmmuAt(new_s, RmiVdevParamsAt(new_s, params_ptr).vsmmu_addr),RmiVdevParamsAt(new_s, params_ptr).vsid))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).num_map == 0)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vdevs == RealmAt(new_s, rd).num_vdevs + 1)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).attest_info.lock_nonce == 0)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).attest_info.meas_nonce == 0)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).attest_info.report_nonce == 0)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).p2p_bound == FEATURE_FALSE)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_READY) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegableDram(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != DELEGATED) &&
       AddrIsGranuleAligned(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       RmiVdevParamsIsValid(old_s, params_ptr) &&
       !(RealmAt(old_s, rd).feat_da != FEATURE_TRUE) &&
       !(RmiVdevParamsAt(old_s, params_ptr).num_aux != VdevAuxCount(old_s, PdevFlags(old_s, PdevAt(old_s, pdev_ptr)),RmiVdevParamsAt(old_s, params_ptr).flags)) &&
       AuxAligned32(old_s, RmiVdevParamsAt(old_s, params_ptr).aux, RmiVdevParamsAt(old_s, params_ptr).num_aux as int) &&
       !(AuxAlias32(old_s, vdev_ptr, RmiVdevParamsAt(old_s, params_ptr).aux, RmiVdevParamsAt(old_s, params_ptr).num_aux as int)) &&
       AuxStateEqual32(old_s, RmiVdevParamsAt(old_s, params_ptr).aux, RmiVdevParamsAt(old_s, params_ptr).num_aux as int, DELEGATED) &&
       VdevIdIsFree(old_s, RealmAt(old_s, rd), RmiVdevParamsAt(old_s, params_ptr).vdev_id) &&
       TdiIdIsFree(old_s, RmiVdevParamsAt(old_s, params_ptr).tdi_id, PdevAt(old_s, pdev_ptr).segment_id) &&
       !(((RmiVdevParamsAt(old_s, params_ptr).tdi_id) < (PdevAt(old_s, pdev_ptr).rid_base) || (RmiVdevParamsAt(old_s, params_ptr).tdi_id) >= (PdevAt(old_s, pdev_ptr).rid_top))) &&
       !((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !AddrIsGranuleAligned(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr))) &&
       !((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PaIsDelegable(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr))) &&
       !((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && GranuleAt(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr).state != VSMMU)) &&
       !((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !VsidIsFree(old_s, VsmmuAt(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr),RmiVdevParamsAt(old_s, params_ptr).vsid))) &&
       !((RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PdevVsmmuIsCompatible(old_s, PdevAt(old_s, pdev_ptr),VsmmuAt(old_s, RmiVdevParamsAt(old_s, params_ptr).vsmmu_addr)))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(old_s, pdev_ptr).num_vdevs)
  && (result.is_Err()
    ==> GranuleAt(new_s, vdev_ptr).state == GranuleAt(old_s, vdev_ptr).state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_id == VdevAt(old_s, vdev_ptr).vdev_id)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).tdi_id == VdevAt(old_s, vdev_ptr).tdi_id)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).pdev == VdevAt(old_s, vdev_ptr).pdev)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).realm == VdevAt(old_s, vdev_ptr).realm)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).dma_state == VdevAt(old_s, vdev_ptr).dma_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).num_aux == VdevAt(old_s, vdev_ptr).num_aux)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vsmmu_addr == VdevAt(old_s, vdev_ptr).vsmmu_addr)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vsid == VdevAt(old_s, vdev_ptr).vsid)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).num_map == VdevAt(old_s, vdev_ptr).num_map)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vdevs == RealmAt(old_s, rd).num_vdevs)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).attest_info.lock_nonce == VdevAt(old_s, vdev_ptr).attest_info.lock_nonce)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).attest_info.meas_nonce == VdevAt(old_s, vdev_ptr).attest_info.meas_nonce)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).attest_info.report_nonce == VdevAt(old_s, vdev_ptr).attest_info.report_nonce)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).p2p_bound == VdevAt(old_s, vdev_ptr).p2p_bound)
  && (!(result.is_Ok() && (RmiVdevParamsAt(old_s, params_ptr).flags.VSMMU == RMI_FEATURE_TRUE)) ==> VdevAt(new_s, vdev_ptr).vsmmu_addr == VdevAt(old_s, vdev_ptr).vsmmu_addr)
}
