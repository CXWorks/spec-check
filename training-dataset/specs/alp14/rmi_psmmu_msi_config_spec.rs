pub open spec fn rmi_psmmu_msi_config_spec(psmmu: Address, gerr_addr: Address, gerr_data: Bits64, eventq_addr: Address, eventq_data: Bits64, priq_addr: Address, priq_data: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!PsmmuAddrIsValid(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PsmmuSupportsMsi(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MsiAddrIsValid(old_s, gerr_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MsiAddrIsValid(old_s, eventq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MsiAddrIsValid(old_s, priq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> gerr_addr is programmed to SMMU_R_GERROR_IRQ_CFG0 with NS bit)
  && (result.is_Ok() ==> eventq_addr is programmed to SMMU_R_EVENTQ_IRQ_CFG0 with NS)
  && ((PsmmuAddrIsValid(old_s, psmmu) &&
       PsmmuSupportsMsi(old_s, psmmu) &&
       MsiAddrIsValid(old_s, gerr_addr) &&
       MsiAddrIsValid(old_s, eventq_addr) &&
       MsiAddrIsValid(old_s, priq_addr))
    ==> result.is_Ok())
}
