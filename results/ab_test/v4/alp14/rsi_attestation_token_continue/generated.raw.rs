pub open spec fn RSI_ATTESTATION_TOKEN_CONTINUE_spec(
    old_s: S,
    new_s: S,
    addr: Address,
    offset: u64,
    size: u64,
    result: RsiCommandReturnCode,
    len: u64
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    
    ((!AddrIsGranuleAligned(addr) ==> result == RSI_ERROR_INPUT) &&
     (!AddrIsProtected(old_s, addr, realm) ==> result == RSI_ERROR_INPUT) &&
     (walk.rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT) &&
     (offset >= RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT) &&
     (offset + size < offset ==> result == RSI_ERROR_INPUT) &&
     (offset + size > RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT) &&
     (rec.attest_state != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE) &&
     ((AddrIsGranuleAligned(addr) &&
       AddrIsProtected(old_s, addr, realm) &&
       walk.rtte.ripas != EMPTY &&
       offset < RMM_GRANULE_SIZE &&
       offset + size >= offset &&
       offset + size <= RMM_GRANULE_SIZE &&
       rec.attest_state == ATTEST_IN_PROGRESS) ==>
      (len == AttestationTokenWrite(old_s, addr, offset as int, size as int) &&
       ((TokenGenerationComplete(old_s, rec) ==>
         (result == RSI_SUCCESS &&
          new_s.realms[realm].recs[CurrentRecIndex(old_s)].attest_state == NO_ATTEST_IN_PROGRESS)) &&
        (!TokenGenerationComplete(old_s, rec) ==>
         result == RSI_INCOMPLETE)))))
}