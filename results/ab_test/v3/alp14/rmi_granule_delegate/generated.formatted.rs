pub open spec fn rmi_granule_delegate_spec(
    result: Result<(), RmiStatusCode>,
    addr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(addr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(addr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, addr).state != UNDELEGATED
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((AddrIsGranuleAligned(addr) && PaIsDelegable(
        addr,
    ) && GranuleAt(old_s, addr).state == UNDELEGATED) ==> (result.is_Ok() && GranuleAt(
        new_s,
        addr,
    ).state == DELEGATED && GranuleAt(new_s, addr).gpt == GPT_REALM))
}