pub open spec fn rmi_rec_aux_count_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    aux_count: UInt64,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(old_s, rd).state != RD ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && Granule(old_s, rd).state
        == RD ==> (result.is_Ok() && aux_count as int == RecAuxCount(old_s, rd))) && old_s == new_s
}