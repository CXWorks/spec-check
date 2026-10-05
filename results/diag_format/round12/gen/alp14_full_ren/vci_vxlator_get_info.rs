pub open spec fn vci_vxlator_get_info_spec(addr: UInt64, result: RsiCommandReturnCode, top: UInt64, old_s: S, new_s: S) -> bool {
  ((addr % EXTENT_SIZE) != 0 ==> result == VCI_ERROR_INPUT)
  && (!IsProtectedLba(old_s, addr) ==> result == VCI_ERROR_INPUT)
  && (result == VCI_ERROR_INPUT ==> result == VCI_ERROR_INPUT)
  && (result == VCI_SUCCESS ==> top == <top value from BLT entry>)
  && ((!(addr % EXTENT_SIZE) != 0 &&
       IsProtectedLba(old_s, addr) &&
       result == VCI_SUCCESS)
    ==> result == VCI_SUCCESS)
}