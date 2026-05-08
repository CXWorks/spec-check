pub open spec fn RMI_GRANULE_UNDELEGATE_spec(
    old_s: S,
    new_s: S,
    addr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let gran_align_fail = !AddrIsGranuleAligned(addr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let gran_bound_fail = !PaIsDelegable(addr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let gran_state_fail = GranuleAt(old_s, addr).state != DELEGATED ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    let gran_gpt_success = result.is_Ok() ==> GranuleAt(new_s, addr).gpt != GPT_REALM;
    let gran_state_success = result.is_Ok() ==> GranuleAt(new_s, addr).state == UNDELEGATED;
    let gran_content_success = result.is_Ok() ==> GranuleContentsWiped(old_s, new_s, addr);

    let failure_conditions = gran_align_fail && gran_bound_fail && gran_state_fail;
    let success_conditions = (result.is_Ok() ==> (gran_gpt_success && gran_state_success
        && gran_content_success));

    failure_conditions && success_conditions
}