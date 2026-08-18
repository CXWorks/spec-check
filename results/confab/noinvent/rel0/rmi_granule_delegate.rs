pub open spec fn rmi_granule_delegate_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, addr: Address) -> bool {
    (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, addr).state != UNDELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, addr).gpt != GPT_NS ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, addr)
        && PaIsDelegable(old_s, addr)
        && Granule(old_s, addr).state == UNDELEGATED
        && Granule(old_s, addr).gpt == GPT_NS)
        ==> (result.is_Ok()
            && Granule(new_s, addr).state == DELEGATED
            && Granule(new_s, addr).gpt == GPT_REALM))
}