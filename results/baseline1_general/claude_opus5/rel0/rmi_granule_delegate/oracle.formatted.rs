pub open spec fn rmi_granule_delegate_spec(
    addr: Address,
    result: Result<(), RmiStatusCode>,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(
        old_s,
        addr,
    ).state != UNDELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(old_s, addr).gpt
        != GPT_NS ==> ResultEqual(result, RMI_ERROR_INPUT)) && (result.is_Ok() ==> Granule(
        new_s,
        addr,
    ).state == DELEGATED) && (result.is_Ok() ==> Granule(new_s, addr).gpt == GPT_REALM) && ((
    AddrIsGranuleAligned(old_s, addr) && PaIsDelegable(old_s, addr) && !(Granule(old_s, addr).state
        != UNDELEGATED) && !(Granule(old_s, addr).gpt != GPT_NS)) ==> result.is_Ok()) && (
    result.is_Err() ==> Granule(new_s, addr).state == Granule(old_s, addr).state) && (
    result.is_Err() ==> Granule(new_s, addr).gpt == Granule(old_s, addr).gpt)
}