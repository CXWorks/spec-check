pub open spec fn RMI_PSMMU_MSI_CONFIG_spec(
    s: S,
    psmmu: Address,
    gerr_addr: Address,
    gerr_data: u64,
    eventq_addr: Address,
    eventq_data: u64,
    priq_addr: Address,
    priq_data: u64,
    result: Result<(), RmiStatusCode>,
) -> bool {
    ((!PsmmuAddrIsValid(s, psmmu)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!PsmmuSupportsMsi(
        s,
        psmmu,
    )) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!MsiAddrIsValid(s, gerr_addr))
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!MsiAddrIsValid(s, eventq_addr))
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!MsiAddrIsValid(s, priq_addr))
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((PsmmuAddrIsValid(s, psmmu)
        && PsmmuSupportsMsi(s, psmmu) && MsiAddrIsValid(s, gerr_addr) && MsiAddrIsValid(
        s,
        eventq_addr,
    ) && MsiAddrIsValid(s, priq_addr)) ==> (result.is_Ok() && PsmmuGerrAddrConfigured(
        s,
        psmmu,
        gerr_addr,
    ) && PsmmuGerrDataConfigured(s, psmmu, gerr_data) && PsmmuEventqAddrConfigured(
        s,
        psmmu,
        eventq_addr,
    ) && PsmmuEventqDataConfigured(s, psmmu, eventq_data) && PsmmuPriqAddrConfigured(
        s,
        psmmu,
        priq_addr,
    ) && PsmmuPriqDataConfigured(s, psmmu, priq_data)))
}