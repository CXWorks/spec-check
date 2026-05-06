pub open spec fn RMI_GRANULE_DELEGATE_spec(old_s: S, new_s: S, addr: Address, result: Result<(), RmiStatusCode>) -> bool {
    ((!AddrIsGranuleAligned(addr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(addr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) &&
    (GranuleAt(old_s, addr).state != RmmGranuleState::UNDELEGATED ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) &&
    (result.is_Ok() ==> (
        GranuleAt(new_s, addr).state == RmmGranuleState::DELEGATED &&
        GranuleAt(new_s, addr).gpt == RmmGpt::GPT_REALM
    )))
}