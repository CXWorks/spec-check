pub open spec fn vci_lba_state_get_spec(base: UInt64, top: UInt64, result: RsiCommandReturnCode, out_top: UInt64, lbamode: UInt8, old_s: S, new_s: S) -> bool {
  ((!(base % EXTENT_SIZE == 0)) ==> result == VCI_ERROR_INPUT)
  && ((!(top % EXTENT_SIZE == 0)) ==> result == VCI_ERROR_INPUT)
  && ((!(top > base)) ==> result == VCI_ERROR_INPUT)
  && (result == VCI_SUCCESS ==> out_top > base)
  && (result == VCI_SUCCESS ==> out_top <= top)
  && ((result != VCI_SUCCESS)
    ==> out_top == 0)
  && ((result != VCI_SUCCESS)
    ==> lbamode == 0)
}