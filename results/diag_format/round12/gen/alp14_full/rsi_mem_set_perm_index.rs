pub open spec fn rsi_mem_set_perm_index_spec(base: UInt64, top: UInt64, perm_index: UInt64, cookie: UInt64, result: RsiCommandReturnCode, new_base: UInt64, response: bool, new_cookie: UInt64, old_s: S, new_s: S) -> bool {
  ((!(base % GRANULE_SIZE == 0) || !(top % GRANULE_SIZE == 0) || !(top <= base) || !AddressRangeInProtectedIpaSpace(old_s, base, top) || !(perm_index >= RMM_NUM_PERM_OVERLAY_INDICES) || !CookieIsValid(old_s, cookie)) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> RealmAt(new_s, current_realm_id(new_s)).overlay_lock_state[perm_index as int] == MEM_PERM_LOCKED)
  && (result == RSI_SUCCESS ==> new_base == RealmAt(new_s, current_realm_id(new_s)).s2ap_addr)
  && (result == RSI_SUCCESS ==> response == RealmAt(new_s, current_realm_id(new_s)).s2ap_response)
  && (result == RSI_SUCCESS ==> new_cookie != cookie)
  && ((!(base % GRANULE_SIZE == 0) && !(top % GRANULE_SIZE == 0) && !(top <= base) && AddressRangeInProtectedIpaSpace(old_s, base, top) && !(perm_index >= RMM_NUM_PERM_OVERLAY_INDICES) && CookieIsValid(old_s, cookie))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RealmAt(new_s, current_realm_id(new_s)).overlay_lock_state[perm_index as int] == RealmAt(old_s, current_realm_id(old_s)).overlay_lock_state[perm_index as int])
  && (result != RSI_SUCCESS
    ==> new_base == 0)
  && (result != RSI_SUCCESS
    ==> response == false)
  && (result != RSI_SUCCESS
    ==> new_cookie == cookie)
  && (result == RSI_SUCCESS
    ==> RealmAt(new_s, current_realm_id(new_s)).overlay_lock_state[perm_index as int] == RealmAt(old_s, current_realm_id(old_s)).overlay_lock_state[perm_index as int])
}