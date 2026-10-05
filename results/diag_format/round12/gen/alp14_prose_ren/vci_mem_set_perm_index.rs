pub open spec fn vci_mem_set_perm_index_spec(base: Address, top: Address, perm_index: UInt64, cookie: Bits64, result: Vcicommandreturncode, new_base: Address, response: Vciresponse, new_cookie: Bits64, old_s: S, new_s: S) -> bool {
  ((!(base % extent_size(old_s) == 0)) ==> result == VCI_ERROR_INPUT)
  && ((!(top % extent_size(old_s) == 0)) ==> result == VCI_ERROR_INPUT)
  && ((!(top <= base)) ==> result == VCI_ERROR_INPUT)
  && (result == VCI_SUCCESS ==> Currentvault(new_s).overlay_locked[perm_index as int] == MEM_PERM_LOCKED)
  && (result == VCI_SUCCESS ==> new_base == Currentworker(new_s).xap_addr)
  && (result == VCI_SUCCESS ==> new_cookie != cookie)
  && ((result != VCI_SUCCESS)
    ==> Currentvault(new_s).overlay_locked[perm_index as int] == Currentvault(old_s).overlay_locked[perm_index as int])
  && (result == VCI_SUCCESS
    ==> Currentvault(new_s).overlay_locked[perm_index as int] == MEM_PERM_LOCKED)
}