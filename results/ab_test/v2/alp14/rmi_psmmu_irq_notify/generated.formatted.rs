pub open spec fn rmi_psmmu_irq_notify_spec(
    result: RmiCommandReturnCode,
    action: RmiSmmuAction,
    rd: Address,
    vsmmu: Address,
    msi_addr: Address,
    msi_data: u64,
    old_s: S,
    new_s: S,
    psmmu: Address,
    irq: RmiSmmuIrq,
) -> bool {
    // Failure condition: psmmu_valid
    (!PsmmuAddrIsValid(old_s, psmmu) ==> result == RMI_ERROR_INPUT)
}