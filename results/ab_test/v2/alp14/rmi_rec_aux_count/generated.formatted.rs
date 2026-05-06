pub open spec fn rmi_rec_aux_count_spec(
    result: RmiCommandReturnCode,
    aux_count: u64,
    old_s: S,
    new_s: S,
    rd: Address,
) -> bool {
    // Failure conditions
    ((!AddrIsGranuleAligned(rd)) ==> result == RMI_ERROR_INPUT) && ((!PaIsDelegable(rd)) ==> result
        == RMI_ERROR_INPUT) && ((GranuleAt(old_s, rd).state != RD) ==> result
        == RMI_ERROR_INPUT)
    // Success condition
     && ((AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD) ==> (
    result == RMI_SUCCESS && aux_count == RecAuxCount(
        old_s,
        rd,
    )))
    // No state change
     && (new_s == old_s)
}