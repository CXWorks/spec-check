pub open spec fn vci_lba_state_get_spec(base: Address, top: Address, result: Vcicommandreturncode, out_top: Address, lbamode: Vcilbamode, old_s: S, new_s: S) -> bool {
  (result == VCI_SUCCESS ==> out_top > base)
  && (result == VCI_SUCCESS ==> out_top <= top)
  && ((!(base % extent_size(old_s) == 0) ||
       !(top % extent_size(old_s) == 0) ||
       !(top <= base) ||
       !address_range_in_protected_range(old_s, base, top))
    ==> result == VCI_ERROR_INPUT)
  && ((result != VCI_SUCCESS)
    ==> out_top == 0)
  && ((result != VCI_SUCCESS)
    ==> lbamode == 0)
}