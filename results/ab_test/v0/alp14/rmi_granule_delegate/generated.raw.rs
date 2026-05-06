```verus
pub open spec fn RMI_GRANULE_DELEGATE_spec(s: S, addr: Address, result: Result<(), RmiStatusCode>) -> bool {
  (
    // Failure condition: gran_align
    (!AddrIsGranuleAligned(addr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    // Failure condition: gran_bound
    (!PaIsDelegable(addr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    // Failure condition: gran_state (pre-condition)
    (GranuleAt(s, addr).state != RmmGranuleState::UNDELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    // Success conditions
    (
      (AddrIsGranuleAligned(addr) &&
       PaIsDelegable(addr) &&
       GranuleAt(s, addr).state == RmmGranuleState::UNDELEGATED) ==>
      (result.is_Ok() &&
       GranuleAt(s, addr).state == RmmGranuleState::DELEGATED &&
       GranuleAt(s, addr).gpt == RmmGpt::GPT_REALM)
    )
  )
}
```