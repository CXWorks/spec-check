pub open spec fn rmi_psmmu_msi_config_spec(result: Result<(), RmiStatusCode>, psmmu: Address, gerr_addr: Address, gerr_data: u64, eventq_addr: Address, eventq_data: u64, priq_addr: Address, priq_data: u64, old_s: S, new_s: S) -> bool {
    (!PsmmuAddrIsValid(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PsmmuSupportsMsi(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!MsiAddrIsValid(old_s, gerr_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!MsiAddrIsValid(old_s, eventq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!MsiAddrIsValid(old_s, priq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((PsmmuAddrIsValid(old_s, psmmu)
        && PsmmuSupportsMsi(old_s, psmmu)
        && MsiAddrIsValid(old_s, gerr_addr)
        && MsiAddrIsValid(old_s, eventq_addr)
        && MsiAddrIsValid(old_s, priq_addr)) ==> result.is_Ok())
}