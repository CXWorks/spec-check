```verus
pub open spec fn RMI_GRANULE_UNDELEGATE_spec(
    s: S,
    addr: Address,
) -> (result: Result<(), RmiStatusCode>) {
    if !AddrIsGranuleAligned(addr) {
        Err(RMI_ERROR_INPUT)
    } else if !PaIsDelegable(addr) {
        Err(RMI_ERROR_INPUT)
    } else if GranuleAt(s, addr).state != DELEGATED {
        Err(RMI_ERROR_INPUT)
    } else {
        Ok(())
    }
}
```

This specification function captures the RMI_GRANULE_UNDELEGATE command logic:

- **Input validation**: Checks three preconditions in order:
  1. Address must be granule-aligned (`gran_align`)
  2. Address must be delegable (`gran_bound`)
  3. Granule must be in DELEGATED state (`gran_state`)

- **Return type**: `Result<(), RmiStatusCode>` indicating either success (`Ok(())`) or failure with an error code

- **Success path**: If all preconditions pass, returns `Ok(())`, implying the postconditions:
  - Granule GPT changes from GPT_REALM to something else
  - Granule state changes from DELEGATED to UNDELEGATED
  - Granule contents are wiped

- **Failure path**: Returns `Err(RMI_ERROR_INPUT)` for any violated precondition