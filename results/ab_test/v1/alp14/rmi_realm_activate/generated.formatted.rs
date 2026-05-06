# RMI_REALM_ACTIVATE Specification

Based on the RMM command specification, here is the Verus specification function:

```verus
pub open spec fn RMI_REALM_ACTIVATE_spec(s: S, rd: Address) -> Result<(), RmiStatusCode> {
    // Failure condition: rd_align
    if !AddrIsGranuleAligned(rd) {
        Err(RMI_ERROR_INPUT)
    }
    // Failure condition: rd_bound
    else if !PaIsDelegable(rd) {
        Err(RMI_ERROR_INPUT)
    }
    // Failure condition: rd_state
    else if GranuleAt(s, rd).state != RD {
        Err(RMI_ERROR_INPUT)
    }
    // Failure condition: realm_state
    else if RealmAt(s, rd).state != REALM_NEW {
        Err(RMI_ERROR_REALM)
    }
    // Success condition: realm_state
    else {
        Ok(())
    }
}
```

**Notes:**
- The function checks failure conditions in the order specified by the ordering constraints: `[rd_bound, rd_state] < [realm_state]` followed by `rd_align`
- Input parameter `rd` is the physical address of the Realm Descriptor
- The function returns `Result<(), RmiStatusCode>` following the standard pattern
- Upon success, the realm transitions from `REALM_NEW` to `REALM_ACTIVE` state (represented implicitly as `Ok(())`)
- The footprint shows that only `realm.state` is modified