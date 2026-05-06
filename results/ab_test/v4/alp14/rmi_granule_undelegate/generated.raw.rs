pub open spec fn RMI_GRANULE_UNDELEGATE_spec(old_s: S, new_s: S, addr: Address, result: Result<(), RmiStatusCode>) -> bool {
    ((!AddrIsGranuleAligned(addr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
     (!PaIsDelegable(addr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
     (GranuleAt(old_s, addr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
     (result.is_Ok() ==> (
       GranuleAt(new_s, addr).gpt != GPT_REALM &&
       GranuleAt(new_s, addr).state == UNDELEGATED
     )))
}