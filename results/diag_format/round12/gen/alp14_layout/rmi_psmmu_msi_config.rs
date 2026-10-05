pub open spec fn rmi_psmmu_msi_config_spec(psmmu: Address, gerr_addr: Address, gerr_data: Bits64, eventq_addr: Address, eventq_data: Bits64, priq_addr: Address, priq_data: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (PsmmuAddrIsValid(old_s, psmmu) ==> result.is_Ok())
  && (PsmmuSupportsMsi(old_s, psmmu) ==> result.is_Ok())
  && (MsiAddrIsValid(old_s, gerr_addr) ==> result.is_Ok())
  && (MsiAddrIsValid(old_s, eventq_addr) ==> result.is_Ok())
  && (MsiAddrIsValid(old_s, priq_addr) ==> result.is_Ok())
  && (result.is_Ok() ==> PsmmuAddrIsValid(new_s, psmmu))
  && (result.is_Ok() ==> PsmmuSupportsMsi(new_s, psmmu))
  && (result.is_Ok() ==> MsiAddrIsValid(new_s, gerr_addr))
  && (result.is_Ok() ==> MsiAddrIsValid(new_s, eventq_addr))
  && (result.is_Ok() ==> MsiAddrIsValid(new_s, priq_addr))
  && ((!(PsmmuAddrIsValid(old_s, psmmu)) ||
       PsmmuSupportsMsi(old_s, psmmu) ||
       MsiAddrIsValid(old_s, gerr_addr) ||
       MsiAddrIsValid(old_s, eventq_addr) ||
       MsiAddrIsValid(old_s, priq_addr))
    ==> result.is_Err())
  && (result.is_Err()
    ==> PsmmuAddrIsValid(new_s, psmmu))
  && (result.is_Err()
    ==> PsmmuSupportsMsi(new_s, psmmu))
  && (result.is_Err()
    ==> MsiAddrIsValid(new_s, gerr_addr))
  && (result.is_Err()
    ==> MsiAddrIsValid(new_s, eventq_addr))
  && (result.is_Err()
    ==> MsiAddrIsValid(new_s, priq_addr))
}