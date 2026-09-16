pub open spec fn rmi_psmmu_st_l2_destroy_spec(psmmu_ptr: Address, sid: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!PsmmuAddrIsValid(old_s, psmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((sid) >= pow2(PsmmuAt(old_s, psmmu_ptr).sid_size as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (sid does not identify the first entry in an L2ST. ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PsmmuAt(old_s, psmmu_ptr).state != PSMMU_ACTIVE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr), sid as int).ste.state == PSMMU_ST_ENTRY_INVALID ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PsmmuL2StIsLive(old_s, PsmmuAt(old_s, psmmu_ptr), sid) ==> (ResultEqual(result, RMI_ERROR_PSMMU_ST_L2_DESTROY) ==> (PsmmuStWalk(new_s, PsmmuAt(new_s, psmmu_ptr), sid as int).level.level == PsmmuStWalk(new_s, PsmmuAt(new_s, psmmu_ptr), sid as int).level))))
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && (result.is_Ok() ==> PsmmuStWalk(new_s, PsmmuAt(new_s, psmmu_ptr), sid as int).ste.state == PSMMU_ST_ENTRY_INVALID)
  && ((!(Rmm(old_s).static.feat_da != FEATURE_TRUE) &&
       PsmmuAddrIsValid(old_s, psmmu_ptr) &&
       !((sid) >= pow2(PsmmuAt(old_s, psmmu_ptr).sid_size as nat)) &&
       !(sid does not identify the first entry in an L2ST.) &&
       !(PsmmuAt(old_s, psmmu_ptr).state != PSMMU_ACTIVE) &&
       !(PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr), sid as int).ste.state == PSMMU_ST_ENTRY_INVALID) &&
       !(PsmmuL2StIsLive(old_s, PsmmuAt(old_s, psmmu_ptr), sid)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PsmmuStWalk(new_s, PsmmuAt(new_s, psmmu_ptr), sid as int).ste.state == PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr), sid as int).ste.state)
}