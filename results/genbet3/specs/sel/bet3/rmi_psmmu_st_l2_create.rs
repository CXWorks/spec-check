pub open spec fn rmi_psmmu_st_l2_create_spec(psmmu_ptr: Address, sid: Bits64, result: RmiResult, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_da != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!PsmmuAddrIsValid(old_s, psmmu_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (PsmmuAt(old_s, psmmu_ptr).state != PSMMU_ACTIVE ==> result.status == RMI_ERROR_INPUT)
  && ((sid) >= pow2(PsmmuAt(old_s, psmmu_ptr).sid_size as nat) ==> result.status == RMI_ERROR_INPUT)
  && ((PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr),sid).level != 1 || PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr),sid).ste.state == PSMMU_ST_ENTRY_TABLE) ==> (result.status == RMI_ERROR_PSMMU_ST && result.data.level.level == PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr),sid).level))
  && (result.status == RMI_SUCCESS ==> PsmmuStWalk(new_s, PsmmuAt(new_s, psmmu_ptr),sid).ste.state == PSMMU_ST_ENTRY_TABLE)
  && ((!(Rmm(old_s).static.feat_da != FEATURE_TRUE) &&
       PsmmuAddrIsValid(old_s, psmmu_ptr) &&
       !(PsmmuAt(old_s, psmmu_ptr).state != PSMMU_ACTIVE) &&
       !((sid) >= pow2(PsmmuAt(old_s, psmmu_ptr).sid_size as nat)) &&
       !((PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr),sid).level != 1 || PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr),sid).ste.state == PSMMU_ST_ENTRY_TABLE)))
    ==> result.status == RMI_SUCCESS)
  && (result.status != RMI_SUCCESS
    ==> PsmmuStWalk(new_s, PsmmuAt(new_s, psmmu_ptr),sid).ste.state == PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr),sid).ste.state)
}