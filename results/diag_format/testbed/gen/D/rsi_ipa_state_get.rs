pub open spec fn rsi_ipa_state_get_spec(base: Address, top: Address, result: RsiCommandReturnCode, out_top: Address, ripas: RsiRipas, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    (!AddrIsGranuleAligned(old_s, realm, base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, realm, top) ==> result == RSI_ERROR_INPUT)
    && (top <= base ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(old_s, realm, base, top) ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> out_top > base)
    && (result == RSI_SUCCESS ==> out_top <= top)
    && (result == RSI_SUCCESS ==> forall addr: Address | AddrInRange(old_s, addr, base, out_top - base) ==> RipasAt(old_s, realm, addr) == ripas)
}