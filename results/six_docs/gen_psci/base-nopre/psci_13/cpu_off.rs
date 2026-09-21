pub open spec fn cpu_off_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.pdev == PDEV_SECURE))
    && (result == RSI_SUCCESS ==> (old_s.pdev == PDEV_NORMAL))
    && (result == RSI_SUCCESS ==> (new_s.core_powered_down == true))
    && (result == RSI_SUCCESS ==> (old_s.core_powered_down == false))
}