pub open spec fn rmi_vdev_create_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> result.status == RMI_ERROR_INPUT)
  && (PdevAt(old_s, pdev_ptr).state != PDEV_READY ==> result.status == RMI_ERROR_INPUT)
  && (!PdevAt(old_s, pdev_ptr).category IN { PDEV_ENDPOINT_ACCEL_OFF_CHIP, PDEV_ENDPOINT_ACCEL_ON_CHIP } ==> result.status == RMI_ERROR_DEVICE)
  && (!PdevStreamsForVdev(old_s, PdevAt(old_s, pdev_ptr)) ==> result.status == RMI_ERROR_DEVICE)
  && (PdevAt(old_s, pdev_ptr).num_vdevs == PdevAt(old_s, pdev_ptr).max_num_vdevs ==> result.status == RMI_ERROR_DEVICE)
  && (RealmAt(old_s, rd).num_vdevs == RealmAt(old_s, rd).max_num_vdevs ==> result.status == RMI_ERROR_DEVICE)
  && (!AddrIsRmiGranuleAligned(old_s, vdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsPopulatedConventional(old_s, vdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTrackedFine(old_s, vdev_ptr) ==> (result.status == RMI_ERROR_TRACKING && result.data.level_addr.level == TrackingToRmiResult(old_s, vdev_ptr) && result.data.level_addr.addr == AddrShiftRmiGranule(old_s, vdev_ptr)))
  && (GranuleAt(old_s, vdev_ptr).state != GRAN_DELEGATED ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!RmiVdevParamsIsValid(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!RmiAddrRangesValid8(old_s, PdevAt(old_s, pdev_ptr).addr_range, PdevAt(old_s, pdev_ptr).num_addr_range) ==> result.status == RMI_ERROR_INPUT)
  && (!VdevAddrRangesInPdevStreams(old_s, PdevAt(old_s, pdev_ptr), PdevAt(old_s, pdev_ptr).addr_range, PdevAt(old_s, pdev_ptr).num_addr_range) ==> result.status == RMI_ERROR_INPUT)
  && (!PdevAddrRangeIsFree(old_s, PdevAt(old_s, pdev_ptr), PdevAt(old_s, pdev_ptr).addr_range, PdevAt(old_s, pdev_ptr).num_addr_range) ==> result.status == RMI_ERROR_INPUT)
  && (RealmAt(old_s, rd).feat_da != FEATURE_TRUE ==> result.status == RMI_ERROR_REALM(0))
  && (!VdevIdIsFree(old_s, RealmAt(old_s, rd), PdevAt(old_s, pdev_ptr).vdev_id) ==> result.status == RMI_ERROR_INPUT)
  && (!TdiIdIsFree(old_s, PdevAt(old_s, pdev_ptr).tdi_id, PdevAt(old_s, pdev_ptr).routing_id) ==> result.status == RMI_ERROR_INPUT)
  && (stream_result.valid != RMM_TRUE ==> result.status == RMI_ERROR_INPUT)
  && ((PdevAt(old_s, pdev_ptr).tdi_id < PdevAt(old_s, pdev_ptr).rid_base || PdevAt(old_s, pdev_ptr).tdi_id >= PdevAt(old_s, pdev_ptr).rid_top) ==> result.status == RMI_ERROR_INPUT)
  && ((st_walk.level != 2 || st_walk.ste.state != PSMMU_ST_ENTRY_INVALID) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !AddrIsRmiGranuleAligned(old_s, PdevAt(old_s, pdev_ptr).vsmmu_addr)) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PaIsTracked(old_s, PdevAt(old_s, pdev_ptr).vsmmu_addr)) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE && GranuleAt(old_s, PdevAt(old_s, pdev_ptr).vsmmu_addr).state != GRAN_VSMMU) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !VsidIsFree(old_s, VsmmuAt(old_s, PdevAt(old_s, pdev_ptr).vsmmu_addr), PdevAt(old_s, pdev_ptr).vsid)) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PdevVsmmuIsCompatible(old_s, PdevAt(old_s, pdev_ptr), VsmmuAt(old_s, PdevAt(old_s, pdev_ptr).vsmmu_addr))) ==> result.status == RMI_ERROR_INPUT)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(new_s, pdev_ptr).num_vdevs + 1)
  && (result.is_Ok() ==> PdevAddrRangeAdded(new_s, PdevAt(new_s, pdev_ptr), PdevAt(new_s, pdev_ptr).num_vdevs, PdevAt(new_s, pdev_ptr).addr_range, PdevAt(new_s, pdev_ptr).num_addr_range))
  && (result.is_Ok() ==> GranuleAt(new_s, vdev_ptr).state == GRAN_VDEV)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).vdev_id == PdevAt(new_s, pdev_ptr).vdev_id)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).tdi_id == PdevAt(new_s, pdev_ptr).tdi_id)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).pdev == pdev_ptr)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).pdev_index == PdevAt(new_s, pdev_ptr).num_vdevs)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).realm == rd)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_NEW)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).dma_state == VDEV_DMA_DISABLED)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).started_once == RMM_FALSE)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_INIT)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> !TdiIdIsFree(new_s, PdevAt(new_s, pdev_ptr).tdi_id, PdevAt(new_s, pdev_ptr).routing_id))
  && (result.is_Ok() && PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE ==> Equal(VdevAt(new_s, vdev_ptr).vsmmu, PdevAt(new_s, pdev_ptr).flags.VSMMU))
  && (result.is_Ok() && PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE ==> VdevAt(new_s, vdev_ptr).vsmmu_addr == PdevAt(new_s, pdev_ptr).vsmmu_addr)
  && (result.is_Ok() && PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE ==> VdevAt(new_s, vdev_ptr).vsmmu_reg_base == VsmmuAt(new_s, PdevAt(new_s, pdev_ptr).vsmmu_addr).reg_base)
  && (result.is_Ok() && PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE ==> VdevAt(new_s, vdev_ptr).vsid == PdevAt(new_s, pdev_ptr).vsid)
  && (result.is_Ok() && PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE ==> !VsidIsFree(new_s, VsmmuAt(new_s, PdevAt(new_s, pdev_ptr).vsmmu_addr), PdevAt(new_s, pdev_ptr).vsid))
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vdevs == RealmAt(new_s, rd).num_vdevs + 1)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).freshness.lock_seq == 0)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).freshness.meas_seq == 0)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).freshness.report_seq == 0)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).num_addr_range == PdevAt(new_s, pdev_ptr).num_addr_range)
  && (result.is_Ok() ==> RmiAddrRangesEqual8(new_s, VdevAt(new_s, vdev_ptr).addr_range, PdevAt(new_s, pdev_ptr).addr_range, PdevAt(new_s, pdev_ptr).num_addr_range))
  && (result.is_Ok() ==> st_walk.ste.state == PSMMU_ST_ENTRY_VALID)
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, pdev_ptr) &&
       PaIsTracked(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_READY) &&
       PdevAt(old_s, pdev_ptr).category IN { PDEV_ENDPOINT_ACCEL_OFF_CHIP, PDEV_ENDPOINT_ACCEL_ON_CHIP } &&
       PdevStreamsForVdev(old_s, PdevAt(old_s, pdev_ptr)) &&
       !(PdevAt(old_s, pdev_ptr).num_vdevs == PdevAt(old_s, pdev_ptr).max_num_vdevs) &&
       !(RealmAt(old_s, rd).num_vdevs == RealmAt(old_s, rd).max_num_vdevs) &&
       AddrIsRmiGranuleAligned(old_s, vdev_ptr) &&
       PaIsPopulatedConventional(old_s, vdev_ptr) &&
       PaIsTrackedFine(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != GRAN_DELEGATED) &&
       AddrIsRmiGranuleAligned(old_s, params_ptr) &&
       NonSecureAccessPermitted(old_s, params_ptr) &&
       RmiVdevParamsIsValid(old_s, params_ptr) &&
       RmiAddrRangesValid8(old_s, PdevAt(old_s, pdev_ptr).addr_range, PdevAt(old_s, pdev_ptr).num_addr_range) &&
       VdevAddrRangesInPdevStreams(old_s, PdevAt(old_s, pdev_ptr), PdevAt(old_s, pdev_ptr).addr_range, PdevAt(old_s, pdev_ptr).num_addr_range) &&
       PdevAddrRangeIsFree(old_s, PdevAt(old_s, pdev_ptr), PdevAt(old_s, pdev_ptr).addr_range, PdevAt(old_s, pdev_ptr).num_addr_range) &&
       !(RealmAt(old_s, rd).feat_da != FEATURE_TRUE) &&
       VdevIdIsFree(old_s, RealmAt(old_s, rd), PdevAt(old_s, pdev_ptr).vdev_id) &&
       TdiIdIsFree(old_s, PdevAt(old_s, pdev_ptr).tdi_id, PdevAt(old_s, pdev_ptr).routing_id) &&
       stream_result.valid == RMM_TRUE &&
       !((PdevAt(old_s, pdev_ptr).tdi_id < PdevAt(old_s, pdev_ptr).rid_base || PdevAt(old_s, pdev_ptr).tdi_id >= PdevAt(old_s, pdev_ptr).rid_top)) &&
       !((st_walk.level != 2 || st_walk.ste.state != PSMMU_ST_ENTRY_INVALID)) &&
       !((PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !AddrIsRmiGranuleAligned(old_s, PdevAt(old_s, pdev_ptr).vsmmu_addr))) &&
       !((PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PaIsTracked(old_s, PdevAt(old_s, pdev_ptr).vsmmu_addr))) &&
       !((PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE && GranuleAt(old_s, PdevAt(old_s, pdev_ptr).vsmmu_addr).state != GRAN_VSMMU)) &&
       !((PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !VsidIsFree(old_s, VsmmuAt(old_s, PdevAt(old_s, pdev_ptr).vsmmu_addr), PdevAt(old_s, pdev_ptr).vsid))) &&
       !((PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PdevVsmmuIsCompatible(old_s, PdevAt(old_s, pdev_ptr), VsmmuAt(old_s, PdevAt(old_s, pdev_ptr).vsmmu_addr)))))
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
    ==> VdevAt(new_s, vdev_ptr).pdev_index == VdevAt(old_s, vdev_ptr).pdev_index)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).realm == VdevAt(old_s, vdev_ptr).realm)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).dma_state == VdevAt(old_s, vdev_ptr).dma_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).started_once == VdevAt(old_s, vdev_ptr).started_once)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> TdiIdIsFree(new_s, PdevAt(new_s, pdev_ptr).tdi_id, PdevAt(new_s, pdev_ptr).routing_id) == TdiIdIsFree(old_s, PdevAt(old_s, pdev_ptr).tdi_id, PdevAt(old_s, pdev_ptr).routing_id))
  && (result.is_Err() && PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE
    ==> Equal(VdevAt(new_s, vdev_ptr).vsmmu, VdevAt(old_s, vdev_ptr).vsmmu))
  && (result.is_Err() && PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE
    ==> VdevAt(new_s, vdev_ptr).vsmmu_addr == VdevAt(old_s, vdev_ptr).vsmmu_addr)
  && (result.is_Err() && PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE
    ==> VdevAt(new_s, vdev_ptr).vsmmu_reg_base == VdevAt(old_s, vdev_ptr).vsmmu_reg_base)
  && (result.is_Err() && PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE
    ==> VdevAt(new_s, vdev_ptr).vsid == VdevAt(old_s, vdev_ptr).vsid)
  && (result.is_Err() && PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE
    ==> VsidIsFree(new_s, VsmmuAt(new_s, PdevAt(new_s, pdev_ptr).vsmmu_addr), PdevAt(new_s, pdev_ptr).vsid) == VsidIsFree(old_s, VsmmuAt(old_s, PdevAt(old_s, pdev_ptr).vsmmu_addr), PdevAt(old_s, pdev_ptr).vsid))
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vdevs == RealmAt(old_s, rd).num_vdevs)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).freshness.lock_seq == VdevAt(old_s, vdev_ptr).freshness.lock_seq)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).freshness.meas_seq == VdevAt(old_s, vdev_ptr).freshness.meas_seq)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).freshness.report_seq == VdevAt(old_s, vdev_ptr).freshness.report_seq)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).num_addr_range == VdevAt(old_s, vdev_ptr).num_addr_range)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).addr_range == VdevAt(old_s, vdev_ptr).addr_range)
  && (result.is_Err()
    ==> st_walk.ste.state == st_walk.ste.state)
  && (!(result.is_Ok() && (PdevAt(old_s, pdev_ptr).flags.VSMMU == RMI_FEATURE_TRUE && !PdevVsmmuIsCompatible(old_s, PdevAt(old_s, pdev_ptr), VsmmuAt(old_s, PdevAt(old_s, pdev_ptr).vsmmu_addr)))) ==> PdevVsmmuIsCompatible(new_s, PdevAt(new_s, pdev_ptr), VsmmuAt(new_s, PdevAt(new_s, pdev_ptr).vsmmu_addr)))
}