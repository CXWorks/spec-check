pub open spec fn RMI_PSMMU_IRQ_NOTIFY_spec(
    s: S,
    psmmu: Address,
    irq: RmiSmmuIrq,
    result: RmiCommandReturnCode,
    action: RmiSmmuAction,
    rd: Address,
    vsmmu: Address,
    msi_addr: Address,
    msi_data: u64,
) -> bool {
    if !PsmmuAddrIsValid(s, psmmu) {
        result == RMI_ERROR_INPUT
    } else {
        true
    }
}