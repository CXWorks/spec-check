pub open spec fn rmi_psmmu_msi_config_spec(psmmu: Address, gerr_addr: Address, gerr_data: Bits64, eventq_addr: Address, eventq_data: Bits64, priq_addr: Address, priq_data: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!PsmmuAt(old_s, psmmu).is_some() ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (PsmmuAt(old_s, psmmu).is_some() && !PsmmuAt(old_s, psmmu).supports_msi() ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!MsiAddrIsValid(old_s, gerr_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!MsiAddrIsValid(old_s, eventq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!MsiAddrIsValid(old_s, priq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (result.is_Ok() ==> (SmmuWrite(old_s, SMMU_R_GERROR_IRQ_CFG0, gerr_addr, NS_BIT_SET) && SmmuWrite(old_s, SMMU_R_GERROR_IRQ_CFG1, gerr_data) && SmmuWrite(old_s, SMMU_R_EVENTQ_IRQ_CFG0, eventq_addr, NS_BIT_SET) && SmmuWrite(old_s, SMMU_R_EVENTQ_IRQ_CFG1, eventq_data) && SmmuWrite(old_s, SMMU_R_PRIQ_IRQ_CFG0, priq_addr, NS_BIT_SET) && SmmuWrite(old_s, SMMU_R_PRIQ_IRQ_CFG1, priq_data)))
}