pub open spec fn rmi_granule_undelegate_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, addr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, addr) && PaIsDelegable(old_s, addr) && GranuleAt(old_s, addr).state == DELEGATED)
      ==> (result.is_Ok() && GranuleAt(new_s, addr).gpt != GPT_REALM && GranuleAt(new_s, addr).state == UNDELEGATED))
}