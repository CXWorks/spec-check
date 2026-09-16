pub open spec fn rmi_psmmu_deactivate_spec(psmmu_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_da != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!PsmmuAddrIsValid(old_s, psmmu_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (PsmmuAt(old_s, psmmu_ptr).state != PSMMU_ACTIVE ==> result.status == RMI_ERROR_INPUT)
  && (PsmmuL1StIsLive(old_s, PsmmuAt(old_s, psmmu_ptr)) ==> result.status == RMI_ERROR_INPUT)
  && (result.is_Ok() ==> PsmmuAt(new_s, psmmu_ptr).state == PSMMU_INACTIVE)
  && ((!(Rmm(old_s).static.feat_da != FEATURE_TRUE) &&
       PsmmuAddrIsValid(old_s, psmmu_ptr) &&
       !(PsmmuAt(old_s, psmmu_ptr).state != PSMMU_ACTIVE) &&
       !(PsmmuL1StIsLive(old_s, PsmmuAt(old_s, psmmu_ptr))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PsmmuAt(new_s, psmmu_ptr).state == PsmmuAt(old_s, psmmu_ptr).state)
}