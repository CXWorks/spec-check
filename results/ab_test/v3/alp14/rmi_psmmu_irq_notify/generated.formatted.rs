pub open spec fn rmi_psmmu_irq_notify_spec(
    result: RmiCommandReturnCode,
    psmmu: Address,
    irq: RmiSmmuIrq,
    action: RmiSmmuAction,
    rd: Address,
    vsmmu: Address,
    msi_addr: Address,
    msi_data: u64,
    old_s: S,
    new_s: S,
) -> bool {
    (!PsmmuAddrIsValid(psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (PsmmuAddrIsValid(psmmu)
        ==> result == RMI_SUCCESS)
}