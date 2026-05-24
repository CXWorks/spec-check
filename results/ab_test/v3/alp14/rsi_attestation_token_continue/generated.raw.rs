pub open spec fn rsi_attestation_token_continue_spec(result: RsiCommandReturnCode, len: u64, addr: Address, offset: u64, size: u64, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    
    (!AddrIsGranuleAligned(addr) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(addr, realm) ==> result == RSI_ERROR_INPUT)
    && (walk.rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
    && (offset >= RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
    && (offset + size < offset ==> result == RSI_ERROR_INPUT)
    && (offset + size > RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
    && (old_rec.attest_state != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE)
    && (
      (AddrIsGranuleAligned(addr)
        && AddrIsProtected(addr, realm)
        && walk.rtte.ripas != EMPTY
        && offset < RMM_GRANULE_SIZE
        && offset + size >= offset
        && offset + size <= RMM_GRANULE_SIZE
        && old_rec.attest_state == ATTEST_IN_PROGRESS)
      ==>
      (
        (result == RSI_INCOMPLETE)
        || (result.is_Ok() && len == AttestationTokenWrite(addr, offset, size))
      )
    )
}