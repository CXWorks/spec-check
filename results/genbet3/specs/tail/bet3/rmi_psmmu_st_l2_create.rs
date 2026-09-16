pub open spec fn rmi_psmmu_st_l2_create_spec(psmmu_ptr: Address, sid: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!PsmmuAddrIsValid(old_s, psmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PsmmuAt(old_s, psmmu_ptr).state != PSMMU_ACTIVE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((sid) >= pow2(PsmmuAt(old_s, psmmu_ptr).sid_size as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr), sid as int).level != 1 || PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr), sid as int).ste.state == PSMMU_ST_ENTRY_TABLE) ==> (ResultEqual(result, RMI_ERROR_PSMMU_ST) ==> (PsmmuStWalk(new_s, PsmmuAt(new_s, psmmu_ptr), sid as int).level.level == PsmmuStWalk(new_s, PsmmuAt(new_s, psmmu_ptr), sid as int).level))))
  && (sid does not identify the first entry in an L2ST. ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> PsmmuStWalk(new_s, PsmmuAt(new_s, psmmu_ptr), sid as int).ste.state == PSMMU_ST_ENTRY_TABLE)
  && ((!(Rmm(old_s).static.feat_da != FEATURE_TRUE) &&
       PsmmuAddrIsValid(old_s, psmmu_ptr) &&
       !(PsmmuAt(old_s, psmmu_ptr).state != PSMMU_ACTIVE) &&
       !((sid) >= pow2(PsmmuAt(old_s, psmmu_ptr).sid_size as nat)) &&
       !((PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr), sid as int).level != 1 || PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr), sid as int).ste.state == PSMMU_ST_ENTRY_TABLE)) &&
       !(sid does not identify the first entry in an L2ST.))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PsmmuStWalk(new_s, PsmmuAt(new_s, psmmu_ptr), sid as int).ste.state == PsmmuStWalk(old_s, PsmmuAt(old_s, psmmu_ptr), sid as int).ste.state)
}