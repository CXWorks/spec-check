pub open spec fn RMI_PSMMU_IRQ_NOTIFY_spec(
    old_s: S,
    new_s: S,
    psmmu: Address,
    irq: RmiSmmuIrq,
    result: RmiCommandReturnCode,
    action: RmiSmmuAction,
    rd: Address,
    vsmmu: Address,
    msi_addr: Address,
    msi_data: u64,
) -> bool {
    ((!PsmmuAddrIsValid(old_s, psmmu)) ==> (result == RMI_ERROR_INPUT)) && (old_s == new_s)
}