```verus
pub open spec fn RMI_GRANULE_UNDELEGATE_spec(s: S, addr: Address, result: Result<(), RmiStatusCode>) -> bool {
  let gran = GranuleAt(s, addr);
  
  // Failure conditions
  if !AddrIsGranuleAligned(s, addr) {
    ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)
  } else if !PaIsDelegable(s, addr) {
    ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)
  } else if gran.state != RmmGranuleState::DELEGATED {
    ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)
  } else {
    // Success conditions
    result.is_Ok() &&
    GranuleAt(s, addr).gpt != RmmGpt::GPT_REALM &&
    GranuleAt(s, addr).state == RmmGranuleState::UNDELEGATED
  }
}
```