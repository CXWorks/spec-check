pub open spec fn rmi_psmmu_irq_notify_spec(psmmu: Address, irq: RmiSmmuIrq, result: Result<(), RmiStatusCode>, action: RmiSmmuAction, rd: Address, vsmmu: Address, msi_addr: Address, msi_data: Bits64, old_s: S, new_s: S) -> bool {
  (!PsmmuAddrIsValid(old_s, psmmu) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PsmmuAddrIsValid(old_s, psmmu))
    ==> result.is_Ok())
}