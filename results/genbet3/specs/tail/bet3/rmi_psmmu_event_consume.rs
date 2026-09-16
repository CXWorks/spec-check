pub open spec fn rmi_psmmu_event_consume_spec(psmmu_ptr: Address, irq: RmiPsmmuIrq, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!PsmmuAddrIsValid(old_s, psmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PsmmuAt(old_s, psmmu_ptr).state != PSMMU_ACTIVE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!(Rmm(old_s).static.feat_da != FEATURE_TRUE) &&
       PsmmuAddrIsValid(old_s, psmmu_ptr) &&
       !(PsmmuAt(old_s, psmmu_ptr).state != PSMMU_ACTIVE))
    ==> result.is_Ok())
}