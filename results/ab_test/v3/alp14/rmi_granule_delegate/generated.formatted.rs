pub open spec fn RMI_GRANULE_DELEGATE_spec(
    old_s: S,
    new_s: S,
    addr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    (
    // Failure: gran_align
    (!AddrIsGranuleAligned(addr) ==> ResultEqual(result, RMI_ERROR_INPUT))) && (
    // Failure: gran_bound
    (!PaIsDelegable(addr) ==> ResultEqual(result, RMI_ERROR_INPUT))) && (
    // Failure: gran_state (input)
    (GranuleAt(old_s, addr).state != UNDELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))) && (
    // Success: gran_state
    (result.is_Ok() ==> GranuleAt(new_s, addr).state == DELEGATED)) && (
    // Success: gran_gpt
    (result.is_Ok() ==> GranuleAt(new_s, addr).gpt == GPT_REALM)) && (
    // If any precondition fails, result must be error
    ((AddrIsGranuleAligned(addr) && PaIsDelegable(addr) && GranuleAt(old_s, addr).state
        == UNDELEGATED) || result.is_Err()))
}