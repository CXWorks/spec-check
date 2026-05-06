```verus
pub open spec fn RMI_GRANULE_DELEGATE_spec(s: S, addr: Address) -> (result: Result<(), RmiStatusCode>, s_post: S) {
    if !AddrIsGranuleAligned(s, addr) {
        (Err(RMI_ERROR_INPUT), s)
    } else if !PaIsDelegable(s, addr) {
        (Err(RMI_ERROR_INPUT), s)
    } else if GranuleAt(s, addr).state != UNDELEGATED {
        (Err(RMI_ERROR_INPUT), s)
    } else {
        let s_post = s.with_granule(addr, GranuleAt(s, addr).with_state(DELEGATED).with_gpt(GPT_REALM));
        (Ok(()), s_post)
    }
}
```

This specification function models the RMI_GRANULE_DELEGATE command by:

1. **Checking failure conditions in order:**
   - Verifying granule address alignment
   - Verifying the address is delegable
   - Verifying the granule is in UNDELEGATED state
   - All return `RMI_ERROR_INPUT` on failure

2. **On success:**
   - Returns `Ok(())`
   - Updates the granule state to DELEGATED
   - Sets the granule GPT to GPT_REALM
   - Returns both the result and the post-state

The function follows the pattern of input validation, state checks, and conditional state mutation based on the RMI specification.