pub open spec fn rsi_mem_set_perm_index_spec(base: Address, top: Address, perm_index: UInt64, cookie: Bits64, result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, new_cookie: Bits64, old_s: S, new_s: S) -> bool {
    (!((base as int) % RMM_GRANULE_SIZE != 0) ==> result == RSI_ERROR_INPUT)
    && (!((top as int) % RMM_GRANULE_SIZE != 0) ==> result == RSI_ERROR_INPUT)
    && (!(top < base) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
    && (!(perm_index < RMM_NUM_PERM_OVERLAY_INDICES) ==> result == RSI_ERROR_INPUT)
    && (!CookieIsValid(old_s, cookie) ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> new_s.RealmAt(CurrentRealm(old_s)).overlay_locked[perm_index as usize] == MEM_PERM_LOCKED)
    && (result == RSI_SUCCESS ==> new_base == old_s.CurrentRec().s2ap_addr)
    && (result == RSI_SUCCESS ==> response == RecS2APResponseToRsi(old_s, old_s.CurrentRec()))
    && (result == RSI_SUCCESS ==> new_cookie != cookie)
}