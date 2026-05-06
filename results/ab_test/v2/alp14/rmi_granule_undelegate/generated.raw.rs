```verus
pub open spec fn rmi_granule_undelegate_spec(result: RmiCommandReturnCode, addr: Address, old_s: S, new_s: S) -> bool {
    // Failure conditions
    ((!AddrIsGranuleAligned(addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!PaIsDelegable(addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((GranuleAt(old_s, addr).state != DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
    
    // Success conditions
    && ((AddrIsGranuleAligned(addr) 
        && PaIsDelegable(addr) 
        && GranuleAt(old_s, addr).state == DELEGATED)
        ==> (result.is_Ok() 
            && GranuleAt(new_s, addr).gpt != GPT_REALM
            && GranuleAt(new_s, addr).state == UNDELEGATED
            && GranuleContentsWiped(old_s, new_s, addr)))
}
```