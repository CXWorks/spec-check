pub open spec fn vci_vxlator_get_info_spec(addr: Address, result: Vcicommandreturncode, top: Address, old_s: S, new_s: S) -> bool {
  ((addr % EXTENT_SIZE) != 0 ==> result == VCI_ERROR_INPUT)
  && (!IsProtectedLba(old_s, Currentvault(old_s), addr) ==> result == VCI_ERROR_INPUT)
  && (result == VCI_SUCCESS ==> top == get_vxlator_top(new_s, Currentvault(new_s), addr, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int))
  && ((!( (addr % EXTENT_SIZE) != 0) &&
       IsProtectedLba(old_s, Currentvault(old_s), addr))
    ==> result == VCI_SUCCESS)
}