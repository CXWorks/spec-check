pub open spec fn rmi_psmmu_irq_notify_spec(psmmu_ptr: Address, irqs: RmiPsmmuIrqSet, result: Result<(), RmiStatusCode>, flags: RmiPsmmuIrqResult, event_num: UInt64, sid: Bits64, fetch_addr: Address, input_addr: Address, syndrome: Bits64, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> result.is_Err())
  && (!PsmmuAddrIsValid(old_s, psmmu_ptr) ==> result == RMI_ERROR_INPUT)
  && (PsmmuAt(old_s, psmmu_ptr).state != PSMMU_ACTIVE ==> result == RMI_ERROR_INPUT)
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       PsmmuAddrIsValid(old_s, psmmu_ptr) &&
       !(PsmmuAt(old_s, psmmu_ptr).state != PSMMU_ACTIVE))
    ==> result.is_Ok())
}