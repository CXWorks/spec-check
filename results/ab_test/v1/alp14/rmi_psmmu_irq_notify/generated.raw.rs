```verus
pub open spec fn RMI_PSMMU_IRQ_NOTIFY_spec(
    s: S,
    psmmu: Address,
    irq: RmiSmmuIrq,
) -> (result: RmiCommandReturnCode, action: RmiSmmuAction, rd: Address, vsmmu: Address, msi_addr: Address, msi_data: u64) {
    if !PsmmuAddrIsValid(s, psmmu) {
        (RMI_ERROR_INPUT, arbitrary(), arbitrary(), arbitrary(), arbitrary(), arbitrary())
    } else {
        arbitrary()
    }
}
```

**Key points:**

1. **Input parameters**: `s` (state), `psmmu` (PSMMU address), `irq` (SMMU IRQ type)
2. **Output tuple**: Returns `(result, action, rd, vsmmu, msi_addr, msi_data)` matching the output registers X0-X5
3. **Failure condition**: Checks `PsmmuAddrIsValid(s, psmmu)` precondition; returns `RMI_ERROR_INPUT` if invalid
4. **Success case**: Returns `arbitrary()` for successful execution (no specific success conditions defined in spec)
5. **Conditional outputs**: The `rd`, `vsmmu`, `msi_addr`, and `msi_data` are only valid when `action == RMI_SMMU_ACTION_VIRQ`, but the spec function returns them regardless (validity constraints would be enforced by proof obligations)