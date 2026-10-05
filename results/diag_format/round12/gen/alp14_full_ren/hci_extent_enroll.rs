pub open spec fn hci_extent_enroll_spec(addr: Address, result: Result<(), HciStatusCode>, old_s: S, new_s: S) -> bool {
  ((addr % EXTENT_SIZE) != 0 ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result.is_Ok() ==> ExtentAt(new_s, addr).state == ENROLLED)
  && (result.is_Ok() ==> ExtentProtectionTable(new_s, addr) == GUARDMAP_VAULT)
  && ((!(addr % EXTENT_SIZE) != 0 &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_INPUT)
    ==> result.is_Ok() &&
       ExtentAt(new_s, addr).state == ENROLLED &&
       ExtentProtectionTable(new_s, addr) == GUARDMAP_VAULT)
  && (result.is_Err()
    ==> ExtentAt(new_s, addr).state == ExtentAt(old_s, addr).state)
  && (result.is_Err()
    ==> ExtentProtectionTable(new_s, addr) == ExtentProtectionTable(old_s, addr))
}