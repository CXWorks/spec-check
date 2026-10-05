pub open spec fn hci_pxlator_msi_config_spec(pxlator: Address, ferr_addr: Address, ferr_data: Bits64, eventq_addr: Address, eventq_data: Bits64, pgreqq_addr: Address, pgreqq_data: Bits64, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_INPUT ==> PXLATOR(old_s, pxlator).msi_supported == false)
  && (result == HCI_ERROR_INPUT ==> is_valid_msi_address(old_s, ferr_addr) == false)
  && (result == HCI_ERROR_INPUT ==> is_valid_msi_address(old_s, eventq_addr) == false)
  && (result == HCI_ERROR_INPUT ==> is_valid_msi_address(old_s, pgreqq_addr) == false)
  && (result == HCI_SUCCESS ==> XLATOR_R_FERROR_IRQ_CFG0(new_s, ferr_addr) & PUB == 1)
  && (result == HCI_SUCCESS ==> XLATOR_R_FERROR_IRQ_CFG1(new_s, ferr_addr) == ferr_data)
  && (result == HCI_SUCCESS ==> XLATOR_R_EVENTQ_IRQ_CFG0(new_s, eventq_addr) & PUB == 1)
  && (result == HCI_SUCCESS ==> XLATOR_R_EVENTQ_IRQ_CFG1(new_s, eventq_addr) == eventq_data)
  && (result == HCI_SUCCESS ==> XLATOR_R_PGREQQ_IRQ_CFG0(new_s, pgreqq_addr) & PUB == 1)
  && (result == HCI_SUCCESS ==> XLATOR_R_PGREQQ_IRQ_CFG1(new_s, pgreqq_addr) == pgreqq_data)
  && ((!(result == HCI_ERROR_INPUT ==> PXLATOR(old_s, pxlator).msi_supported == false))
    && (result != HCI_ERROR_INPUT || is_valid_msi_address(old_s, ferr_addr))
    && (result != HCI_ERROR_INPUT || is_valid_msi_address(old_s, eventq_addr))
    && (result != HCI_ERROR_INPUT || is_valid_msi_address(old_s, pgreqq_addr))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> XLATOR_R_FERROR_IRQ_CFG0(new_s, ferr_addr) & PUB == XLATOR_R_FERROR_IRQ_CFG0(old_s, ferr_addr) & PUB)
  && (result != HCI_SUCCESS
    ==> XLATOR_R_FERROR_IRQ_CFG1(new_s, ferr_addr) == XLATOR_R_FERROR_IRQ_CFG1(old_s, ferr_addr))
  && (result != HCI_SUCCESS
    ==> XLATOR_R_EVENTQ_IRQ_CFG0(new_s, eventq_addr) & PUB == XLATOR_R_EVENTQ_IRQ_CFG0(old_s, eventq_addr) & PUB)
  && (result != HCI_SUCCESS
    ==> XLATOR_R_EVENTQ_IRQ_CFG1(new_s, eventq_addr) == XLATOR_R_EVENTQ_IRQ_CFG1(old_s, eventq_addr))
  && (result != HCI_SUCCESS
    ==> XLATOR_R_PGREQQ_IRQ_CFG0(new_s, pgreqq_addr) & PUB == XLATOR_R_PGREQQ_IRQ_CFG0(old_s, pgreqq_addr) & PUB)
  && (result != HCI_SUCCESS
    ==> XLATOR_R_PGREQQ_IRQ_CFG1(new_s, pgreqq_addr) == XLATOR_R_PGREQQ_IRQ_CFG1(old_s, pgreqq_addr))
}