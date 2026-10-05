pub open spec fn vci_mem_set_perm_index_spec(base: UInt64, top: UInt64, perm_index: UInt64, cookie: UInt64, result: RsiCommandReturnCode, new_base: UInt64, response: bool, new_cookie: UInt64, old_s: S, new_s: S) -> bool {
  ((!(base % EXTENT_SIZE == 0) || !(top % EXTENT_SIZE == 0) || !(top <= base) || !((base..=top).all(|addr| addr < VaultAt(new_s, 0).protected_lba_space.1)) || !(perm_index >= KEEPER_NUM_PERM_OVERLAY_INDICES) || !CookieIsValid(new_s, cookie)) ==> result == VCI_ERROR_INPUT)
  && (result == VCI_SUCCESS ==> VaultAt(new_s, 0).overlay_lock_state[perm_index as int] == MEM_PERM_LOCKED)
  && (result == VCI_SUCCESS ==> new_base == WorkerAt(new_s, 0).xap_addr)
  && (result == VCI_SUCCESS ==> response == WorkerAt(new_s, 0).stage_2_access_permission_response)
  && (result == VCI_SUCCESS ==> new_cookie != cookie)
  && ((!(base % EXTENT_SIZE == 0) && !(top % EXTENT_SIZE == 0) && !(top <= base) && ((base..=top).all(|addr| addr < VaultAt(new_s, 0).protected_lba_space.1)) && !(perm_index >= KEEPER_NUM_PERM_OVERLAY_INDICES) && CookieIsValid(new_s, cookie))
    ==> result == VCI_SUCCESS)
  && (result != VCI_SUCCESS
    ==> VaultAt(new_s, 0).overlay_lock_state[perm_index as int] == VaultAt(old_s, 0).overlay_lock_state[perm_index as int])
  && (result != VCI_SUCCESS
    ==> new_base == WorkerAt(old_s, 0).xap_addr)
  && (result != VCI_SUCCESS
    ==> response == WorkerAt(old_s, 0).stage_2_access_permission_response)
  && (result != VCI_SUCCESS
    ==> new_cookie == cookie)
  && (VaultAt(new_s, 0).overlay_lock_state[perm_index as int] == VaultAt(old_s, 0).overlay_lock_state[perm_index as int]
    ==> result != VCI_SUCCESS)
}