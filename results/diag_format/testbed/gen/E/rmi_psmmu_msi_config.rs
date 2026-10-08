pub open spec fn rmi_psmmu_msi_config_spec(psmmu: Address, gerr_addr: Address, gerr_data: Bits64, eventq_addr: Address, eventq_data: Bits64, priq_addr: Address, priq_data: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!VsmmuIsLive(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!VsmmuSupportsMsi(old_s, psmmu, gerr_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MsiAddrIsValid(old_s, gerr_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MsiAddrIsValid(old_s, eventq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!MsiAddrIsValid(old_s, priq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> SMMU_R_GERROR_IRQ_CFG0(new_s, VsmmuAt(new_s, psmmu)) == gerr_addr)
  && (result.is_Ok() ==> SMMU_R_GERROR_IRQ_CFG1(new_s, VsmmuAt(new_s, psmmu)) == gerr_data)
  && (result.is_Ok() ==> SMMU_R_EVENTQ_IRQ_CFG0(new_s, VsmmuAt(new_s, psmmu)) == eventq_addr)
  && (result.is_Ok() ==> SMMU_R_EVENTQ_IRQ_CFG1(new_s, VsmmuAt(new_s, psmmu)) == eventq_data)
  && (result.is_Ok() ==> SMMU_R_PRIQ_IRQ_CFG0(new_s, VsmmuAt(new_s, psmmu)) == priq_addr)
  && (result.is_Ok() ==> SMMU_R_PRIQ_IRQ_CFG1(new_s, VsmmuAt(new_s, psmmu)) == priq_data)
  && ((!(VsmmuIsLive(old_s, psmmu)) &&
       VsmmuSupportsMsi(old_s, psmmu, gerr_addr) &&
       MsiAddrIsValid(old_s, gerr_addr) &&
       MsiAddrIsValid(old_s, eventq_addr) &&
       MsiAddrIsValid(old_s, priq_addr))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> SMMU_R_GERROR_IRQ_CFG0(new_s, VsmmuAt(new_s, psmmu)) == SMMU_R_GERROR_IRQ_CFG0(old_s, VsmmuAt(old_s, psmmu)))
  && (result.is_Err()
    ==> SMMU_R_GERROR_IRQ_CFG1(new_s, VsmmuAt(new_s, psmmu)) == SMMU_R_GERROR_IRQ_CFG1(old_s, VsmmuAt(old_s, psmmu)))
  && (result.is_Err()
    ==> SMMU_R_EVENTQ_IRQ_CFG0(new_s, VsmmuAt(new_s, psmmu)) == SMMU_R_EVENTQ_IRQ_CFG0(old_s, VsmmuAt(old_s, psmmu)))
  && (result.is_Err()
    ==> SMMU_R_EVENTQ_IRQ_CFG1(new_s, VsmmuAt(new_s, psmmu)) == SMMU_R_EVENTQ_IRQ_CFG1(old_s, VsmmuAt(old_s, psmmu)))
  && (result.is_Err()
    ==> SMMU_R_PRIQ_IRQ_CFG0(new_s, VsmmuAt(new_s, psmmu)) == SMMU_R_PRIQ_IRQ_CFG0(old_s, VsmmuAt(old_s, psmmu)))
  && (result.is_Err()
    ==> SMMU_R_PRIQ_IRQ_CFG1(new_s, VsmmuAt(new_s, psmmu)) == SMMU_R_PRIQ_IRQ_CFG1(old_s, VsmmuAt(old_s, psmmu)))
}