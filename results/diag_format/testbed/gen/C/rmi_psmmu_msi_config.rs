pub open spec fn rmi_psmmu_msi_config_spec(psmmu: Address, gerr_addr: Address, gerr_data: Bits64, eventq_addr: Address, eventq_data: Bits64, priq_addr: Address, priq_data: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!PsmmuAddrIsValid(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PsmmuSupportsMsi(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MsiAddrIsValid(old_s, gerr_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MsiAddrIsValid(old_s, eventq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MsiAddrIsValid(old_s, priq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> PsmmuRegister(new_s, psmmu, SMMU_R_GERROR_IRQ_CFG0) == AddrWithNsBitSet(new_s, gerr_addr))
  && (result.is_Ok() ==> PsmmuRegister(new_s, psmmu, SMMU_R_GERROR_IRQ_CFG1) == gerr_data)
  && (result.is_Ok() ==> PsmmuRegister(new_s, psmmu, SMMU_R_EVENTQ_IRQ_CFG0) == AddrWithNsBitSet(new_s, eventq_addr))
  && (result.is_Ok() ==> PsmmuRegister(new_s, psmmu, SMMU_R_EVENTQ_IRQ_CFG1) == eventq_data)
  && (result.is_Ok() ==> PsmmuRegister(new_s, psmmu, SMMU_R_PRIQ_IRQ_CFG0) == AddrWithNsBitSet(new_s, priq_addr))
  && (result.is_Ok() ==> PsmmuRegister(new_s, psmmu, SMMU_R_PRIQ_IRQ_CFG1) == priq_data)
  && ((PsmmuAddrIsValid(old_s, psmmu) &&
       PsmmuSupportsMsi(old_s, psmmu) &&
       MsiAddrIsValid(old_s, gerr_addr) &&
       MsiAddrIsValid(old_s, eventq_addr) &&
       MsiAddrIsValid(old_s, priq_addr))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PsmmuRegister(new_s, psmmu, SMMU_R_GERROR_IRQ_CFG0) == PsmmuRegister(old_s, psmmu, SMMU_R_GERROR_IRQ_CFG0))
  && (result.is_Err()
    ==> PsmmuRegister(new_s, psmmu, SMMU_R_GERROR_IRQ_CFG1) == PsmmuRegister(old_s, psmmu, SMMU_R_GERROR_IRQ_CFG1))
  && (result.is_Err()
    ==> PsmmuRegister(new_s, psmmu, SMMU_R_EVENTQ_IRQ_CFG0) == PsmmuRegister(old_s, psmmu, SMMU_R_EVENTQ_IRQ_CFG0))
  && (result.is_Err()
    ==> PsmmuRegister(new_s, psmmu, SMMU_R_EVENTQ_IRQ_CFG1) == PsmmuRegister(old_s, psmmu, SMMU_R_EVENTQ_IRQ_CFG1))
  && (result.is_Err()
    ==> PsmmuRegister(new_s, psmmu, SMMU_R_PRIQ_IRQ_CFG0) == PsmmuRegister(old_s, psmmu, SMMU_R_PRIQ_IRQ_CFG0))
  && (result.is_Err()
    ==> PsmmuRegister(new_s, psmmu, SMMU_R_PRIQ_IRQ_CFG1) == PsmmuRegister(old_s, psmmu, SMMU_R_PRIQ_IRQ_CFG1))
}