pub open spec fn rsi_attestation_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    let granule = GranuleAt(old_s, addr);
    let bytes_written = if walk.level == 0 {
        if offset == 0 && size == 0 { 0 }
        else if offset + size <= RMM_GRANULE_SIZE { size }
        else { RMM_GRANULE_SIZE - offset }
    } else {
        0
    };
    let bytes_to_write = if offset + size <= RMM_GRANULE_SIZE { size } else { RMM_GRANULE_SIZE - offset };
    let min_written = if offset >= RMM_GRANULE_SIZE { 0 } else { bytes_to_write };
    (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, addr, realm) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (GranuleAt(old_s, addr).state == RMM_GRANULE_EMPTY ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (offset >= RMM_GRANULE_SIZE ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (offset + size < offset ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (offset + size > RMM_GRANULE_SIZE ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (rec.attest_state != ATTEST_IN_PROGRESS ==> ResultEqual(result, RSI_ERROR_STATE))
    && (rec.attest_state == ATTEST_IN_PROGRESS ==> (ResultEqual(result, RSI_INCOMPLETE) && len == min_written && new_s.CurrentRec().attest_state == NO_ATTEST_IN_PROGRESS && new_s.GranuleAt(addr).content[0..min_written as usize] == old_s.GranuleAt(addr).content[offset as usize..(offset + min_written) as usize]))
}