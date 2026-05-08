```rust
pub open spec fn RMI_REC_AUX_COUNT_spec(s: S, rd: Address) -> (result: RmiCommandReturnCode, aux_count: u64) {
    if !AddrIsGranuleAligned(rd) {
        (RMI_ERROR_INPUT, 0)
    } else if !PaIsDelegable(rd) {
        (RMI_ERROR_INPUT, 0)
    } else if GranuleAt(s, rd).state != RmmGranuleState::RD {
        (RMI_ERROR_INPUT, 0)
    } else {
        let rd_granule = GranuleAt(s, rd);
        let rd_obj = rd_granule.data.get_RD();
        (RMI_SUCCESS, RecAuxCount(s, rd_obj) as u64)
    }
}
```

**Key points:**

1. **Input parameters**: `s` (system state) and `rd` (realm descriptor address)
2. **Output**: Tuple of `(RmiCommandReturnCode, u64)` for result and aux_count
3. **Failure conditions** (checked in order):
   - `rd_align`: RD not granule-aligned → `RMI_ERROR_INPUT`
   - `rd_bound`: RD not delegable → `RMI_ERROR_INPUT`
   - `rd_state`: Granule at RD not in RD state → `RMI_ERROR_INPUT`
4. **Success condition**: Returns `RMI_SUCCESS` with `aux_count` from `RecAuxCount(s, rd_obj)`
5. **Footprint**: No state modifications (query-only operation)