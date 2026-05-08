pub open spec fn RMI_REC_AUX_COUNT_spec(
    old_s: S,
    rd: Address,
    result: Result<(), RmiStatusCode>,
    aux_count: u64,
) -> bool {
    // Failure condition: rd_align
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_bound
    (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_state
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Success condition: aux_count
    ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state
        == RD) ==> (result.is_Ok() && aux_count == RecAuxCount(old_s, rd)))
}