pub open spec fn rmi_psmmu_irq_notify_spec(psmmu: Address, irq: RmiSmmuIrq, result: RmiCommandReturnCode, action: RmiSmmuAction, rd: Address, vsmmu: Address, msi_addr: Address, msi_data: Bits64, old_s: S, new_s: S) -> bool {
  (!PsmmuAddrIsValid(old_s, psmmu) ==> result == RMI_ERROR_INPUT)
  && (result == RMI_SUCCESS ==> PsmmuDevResponseToRmi(psmmu, irq, old_s) == PsmmuDevResponseToRmi(psmmu, irq, new_s))
  && (result == RMI_SUCCESS ==> (rd == PsmmuRipasResponseToRmi(psmmu, irq, old_s) && vsmmu == PsmmuVsmmuResponseToRmi(psmmu, irq, old_s)))
  && (result == RMI_SUCCESS ==> (msi_addr == PsmmuMsiResponseToRmi(psmmu, irq, old_s) && msi_data == PsmmuMsiDataToRmi(psmmu, irq, old_s)))
  && (result != RMI_SUCCESS
    ==> PsmmuDevResponseToRmi(new_s, psmmu, irq) == PsmmuDevResponseToRmi(old_s, psmmu, irq))
  && (result != RMI_SUCCESS
    ==> PsmmuRipasResponseToRmi(new_s, psmmu, irq) == PsmmuRipasResponseToRmi(old_s, psmmu, irq))
  && (result != RMI_SUCCESS
    ==> PsmmuVsmmuResponseToRmi(new_s, psmmu, irq) == PsmmuVsmmuResponseToRmi(old_s, psmmu, irq))
  && (result != RMI_SUCCESS
    ==> PsmmuMsiResponseToRmi(new_s, psmmu, irq) == PsmmuMsiResponseToRmi(old_s, psmmu, irq))
  && (result != RMI_SUCCESS
    ==> PsmmuMsiDataToRmi(new_s, psmmu, irq) == PsmmuMsiDataToRmi(old_s, psmmu, irq))
}