pub open spec fn rmi_granule_undelegate_spec(
    result: Result<(), RmiStatusCode>,
    addr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(addr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(addr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, addr).state != DELEGATED
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((AddrIsGranuleAligned(addr) && PaIsDelegable(
        addr,
    ) && GranuleAt(old_s, addr).state == DELEGATED) ==> (result.is_Ok() && GranuleAt(
        new_s,
        addr,
    ).gpt != GPT_REALM && GranuleAt(new_s, addr).state == UNDELEGATED))
}