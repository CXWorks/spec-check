pub open spec fn hci_extent_enroll_spec(addr: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  ((!(addr % extent_size(old_s) == 0)) ==> result == HCI_ERROR_INPUT)
  && (is_physical_address_that_can_be_enrolled(old_s, addr) ==> result == HCI_ERROR_INPUT)
  && (Extentat(old_s, addr).state == UNENROLLED ==> result == HCI_ERROR_INPUT)
  && (result == HCI_SUCCESS ==> Extentat(new_s, addr).state == ENROLLED)
  && (result == HCI_SUCCESS ==> Extentat(new_s, addr).guardmap == GUARDMAP_VAULT)
  && ((!(addr % extent_size(old_s) == 0) &&
       is_physical_address_that_can_be_enrolled(old_s, addr) &&
       !(Extentat(old_s, addr).state == UNENROLLED))
    ==> result == HCI_ERROR_INPUT)
  && (result != HCI_SUCCESS
    ==> Extentat(new_s, addr).state == Extentat(old_s, addr).state)
  && (result != HCI_SUCCESS
    ==> Extentat(new_s, addr).guardmap == Extentat(old_s, addr).guardmap)
  && (result == HCI_SUCCESS
    ==> Extentat(new_s, addr).guardmap == GUARDMAP_VAULT)
}