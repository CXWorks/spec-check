pub open spec fn rmi_granule_undelegate_spec(result: Result<(), RmiStatusCode>, addr: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, addr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, addr)
        && PaIsDelegable(old_s, addr)
        && Granule(old_s, addr).state == DELEGATED)
        ==> (result.is_Ok()
            && Granule(new_s, addr).gpt == GPT_NS
            && Granule(new_s, addr).state == UNDELEGATED))
}