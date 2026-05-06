# RMI_MEC_SET_PRIVATE Specification

Based on the command specification provided, here's the Verus specification function:

```verus
pub open spec fn RMI_MEC_SET_PRIVATE_spec(s: S, mecid: u64) -> Result<(), RmiStatusCode> {
    if UInt(mecid) > UInt(ImplFeatures().max_mecid) {
        Err(RMI_ERROR_INPUT)
    } else if MecState(mecid) != MEC_STATE_SHARED {
        Err(RMI_ERROR_INPUT)
    } else if MecMembers(mecid) != 0 {
        Err(RMI_ERROR_INPUT)
    } else {
        Ok(())
    }
}

pub open spec fn RMI_MEC_SET_PRIVATE_success(s: S, mecid: u64) -> bool {
    RMI_MEC_SET_PRIVATE_spec(s, mecid).is_Ok() ==>
        MecState(mecid) == MEC_STATE_PRIVATE_UNASSIGNED
}
```

**Key features:**

1. **Input validation** - Checks failure conditions in order:
   - `mecid_bound`: MECID exceeds maximum supported value
   - `state`: MEC is not in SHARED state
   - `members`: MEC still has members assigned

2. **Success behavior** - Returns `Ok(())` when all preconditions pass, resulting in the MEC transitioning to `MEC_STATE_PRIVATE_UNASSIGNED` state

3. **Footprint** - Only accesses `MecState(mecid)` memory location