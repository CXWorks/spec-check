```rust
pub open spec fn RMI_PSMMU_MSI_CONFIG_spec(
    s: S,
    psmmu: Address,
    gerr_addr: Address,
    gerr_data: u64,
    eventq_addr: Address,
    eventq_data: u64,
    priq_addr: Address,
    priq_data: u64,
) -> Result<(), RmiStatusCode> {
    if !PsmmuAddrIsValid(s, psmmu) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    } else if !PsmmuSupportsMsi(s, psmmu) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    } else if !MsiAddrIsValid(s, gerr_addr) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    } else if !MsiAddrIsValid(s, eventq_addr) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    } else if !MsiAddrIsValid(s, priq_addr) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    } else {
        Ok(())
    }
}
```

This specification function:
- Takes the command inputs as parameters (state `s`, PSMMU address, and three MSI address/data pairs)
- Returns a `Result<(), RmiStatusCode>` matching the command's output
- Validates all preconditions in the specified order:
  1. PSMMU address validity
  2. PSMMU MSI support
  3. GERROR MSI address validity
  4. EVENTQ MSI address validity
  5. PRIQ MSI address validity
- Returns `RMI_ERROR_INPUT` for any validation failure
- Returns `Ok(())` on success, implicitly encoding that the MSI configuration gets programmed to the respective SMMU registers