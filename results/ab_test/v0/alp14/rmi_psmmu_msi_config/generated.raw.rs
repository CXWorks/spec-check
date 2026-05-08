```verus
pub open spec fn RMI_PSMMU_MSI_CONFIG_spec(
    s: S,
    psmmu: Address,
    gerr_addr: Address,
    gerr_data: u64,
    eventq_addr: Address,
    eventq_data: u64,
    priq_addr: Address,
    priq_data: u64,
    result: Result<(), RmiStatusCode>
) -> bool {
    if !PsmmuAddrIsValid(s, psmmu) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !PsmmuSupportsMsi(s, psmmu) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !MsiAddrIsValid(s, gerr_addr) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !MsiAddrIsValid(s, eventq_addr) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !MsiAddrIsValid(s, priq_addr) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else {
        result.is_Ok()
    }
}
```