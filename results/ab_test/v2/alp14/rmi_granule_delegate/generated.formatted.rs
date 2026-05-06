pub open spec fn rmi_granule_delegate_spec(
    result: RmiCommandReturnCode,
    addr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    // Failure conditions
    let gran_align_fail = !AddrIsGranuleAligned(addr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let gran_bound_fail = !PaIsDelegable(addr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let gran_state_fail = GranuleAt(old_s, addr).state != UNDELEGATED ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    // Success conditions
    let gran_state_success = (AddrIsGranuleAligned(addr) && PaIsDelegable(addr) && GranuleAt(
        old_s,
        addr,
    ).state == UNDELEGATED) ==> (GranuleAt(new_s, addr).state == DELEGATED && GranuleAt(
        new_s,
        addr,
    ).gpt == GPT_REALM && !ResultEqual(result, RMI_ERROR_INPUT));

    gran_align_fail && gran_bound_fail && gran_state_fail && gran_state_success
}