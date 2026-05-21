pub open spec fn rmi_psmmu_msi_config_spec(
    result: RmiCommandReturnCode,
    psmmu: Address,
    gerr_addr: Address,
    gerr_data: u64,
    eventq_addr: Address,
    eventq_data: u64,
    priq_addr: Address,
    priq_data: u64,
    old_s: S,
    new_s: S,
) -> bool {
    (!PsmmuAddrIsValid(psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PsmmuSupportsMsi(psmmu)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!MsiAddrIsValid(gerr_addr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (!MsiAddrIsValid(eventq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !MsiAddrIsValid(priq_addr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((PsmmuAddrIsValid(
        psmmu,
    ) && PsmmuSupportsMsi(psmmu) && MsiAddrIsValid(gerr_addr) && MsiAddrIsValid(eventq_addr)
        && MsiAddrIsValid(priq_addr)) ==> (result == RMI_OK))
}