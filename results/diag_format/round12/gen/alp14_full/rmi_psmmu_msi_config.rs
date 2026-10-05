pub open spec fn rmi_psmmu_msi_config_spec(psmmu: UInt64, gerr_addr: UInt64, gerr_data: UInt64, eventq_addr: UInt64, eventq_data: UInt64, priq_addr: UInt64, priq_data: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (PsmmuAt(old_s, psmmu).is_none() ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) ==> PsmmuAt(new_s, psmmu).is_none())
  && (result.is_Ok() ==> PsmmuAt(new_s, psmmu).is_some())
  && ((!(PsmmuAt(old_s, psmmu).is_none()))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> result.is_Ok())
}