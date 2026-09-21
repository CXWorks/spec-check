pub open spec fn migrate_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.pdev.mpidr != new_s.pdev.mpidr))
    && (result == RSI_ERROR_STATE ==> (old_s.pdev.trusted_os_present != new_s.pdev.trusted_os_present))
    && (result == RSI_INCOMPLETE ==> (old_s.pdev.trusted_os_present != new_s.pdev.trusted_os_present))
    && (result == RSI_ERROR_UNKNOWN ==> (old_s.pdev.trusted_os_present != new_s.pdev.trusted_os_present))
    && (result == RSI_SUCCESS ==> (old_s.pdev.trusted_os_present == new_s.pdev.trusted_os_present))
}