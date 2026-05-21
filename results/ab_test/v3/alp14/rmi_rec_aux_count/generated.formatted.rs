pub open spec fn rmi_rec_aux_count_spec(
    result: RmiCommandReturnCode,
    aux_count: u64,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD ==> (result == RMI_SUCCESS
        && aux_count == RecAuxCount(old_s, rd))) && old_s == new_s
}