pub open spec fn RMI_PSMMU_MSI_CONFIG_spec(
    old_s: S,
    new_s: S,
    psmmu: Address,
    gerr_addr: Address,
    gerr_data: u64,
    eventq_addr: Address,
    eventq_data: u64,
    priq_addr: Address,
    priq_data: u64,
    result: Result<(), RmiStatusCode>,
) -> bool {
    // Failure conditions
    (!PsmmuAddrIsValid(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PsmmuSupportsMsi(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!MsiAddrIsValid(
        old_s,
        gerr_addr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!MsiAddrIsValid(old_s, eventq_addr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!MsiAddrIsValid(old_s, priq_addr)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Success conditions
    (result.is_Ok() ==> (PsmmuAddrIsValid(old_s, psmmu) && PsmmuSupportsMsi(old_s, psmmu)
        && MsiAddrIsValid(old_s, gerr_addr) && MsiAddrIsValid(old_s, eventq_addr) && MsiAddrIsValid(
        old_s,
        priq_addr,
    ) && PsmmuGerrAddrConfigured(new_s, psmmu, gerr_addr) && PsmmuGerrDataConfigured(
        new_s,
        psmmu,
        gerr_data,
    ) && PsmmuEventqAddrConfigured(new_s, psmmu, eventq_addr) && PsmmuEventqDataConfigured(
        new_s,
        psmmu,
        eventq_data,
    ) && PsmmuPriqAddrConfigured(new_s, psmmu, priq_addr) && PsmmuPriqDataConfigured(
        new_s,
        psmmu,
        priq_data,
    )))
}