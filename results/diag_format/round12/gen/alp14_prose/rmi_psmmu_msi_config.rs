pub open spec fn rmi_psmmu_msi_config_spec(psmmu: Address, gerr_addr: Address, gerr_data: Bits64, eventq_addr: Address, eventq_data: Bits64, priq_addr: Address, priq_data: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (PSMMUAt(old_s, psmmu).is_none() ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PSMMUAt(old_s, psmmu).map_or(false, |p| p.supports_msis) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!IsValidMSIAddress(old_s, gerr_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!IsValidMSIAddress(old_s, eventq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!IsValidMSIAddress(old_s, priq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> SMMU_R_GERROR_IRQ_CFG0(new_s) == gerr_addr)
  && (result.is_Ok() ==> SMMU_R_GERROR_IRQ_CFG1(new_s) == gerr_data)
  && (result.is_Ok() ==> SMMU_R_EVENTQ_IRQ_CFG0(new_s) == eventq_addr)
  && (result.is_Ok() ==> SMMU_R_EVENTQ_IRQ_CFG1(new_s) == eventq_data)
  && (result.is_Ok() ==> SMMU_R_PRIQ_IRQ_CFG0(new_s) == priq_addr)
  && (result.is_Ok() ==> SMMU_R_PRIQ_IRQ_CFG1(new_s) == priq_data)
  && ((!(PSMMUAt(old_s, psmmu).is_none()) &&
       PSMMUAt(old_s, psmmu).map_or(true, |p| p.supports_msis) &&
       IsValidMSIAddress(old_s, gerr_addr) &&
       IsValidMSIAddress(old_s, eventq_addr) &&
       IsValidMSIAddress(old_s, priq_addr))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> SMMU_R_GERROR_IRQ_CFG0(new_s) == SMMU_R_GERROR_IRQ_CFG0(old_s))
  && (result.is_Err()
    ==> SMMU_R_GERROR_IRQ_CFG1(new_s) == SMMU_R_GERROR_IRQ_CFG1(old_s))
  && (result.is_Err()
    ==> SMMU_R_EVENTQ_IRQ_CFG0(new_s) == SMMU_R_EVENTQ_IRQ_CFG0(old_s))
  && (result.is_Err()
    ==> SMMU_R_EVENTQ_IRQ_CFG1(new_s) == SMMU_R_EVENTQ_IRQ_CFG1(old_s))
  && (result.is_Err()
    ==> SMMU_R_PRIQ_IRQ_CFG0(new_s) == SMMU_R_PRIQ_IRQ_CFG0(old_s))
  && (result.is_Err()
    ==> SMMU_R_PRIQ_IRQ_CFG1(new_s) == SMMU_R_PRIQ_IRQ_CFG1(old_s))
}