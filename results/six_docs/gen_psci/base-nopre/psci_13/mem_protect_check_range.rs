pub open spec fn mem_protect_check_range_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.pdev == null || old_s.pdev.len == 0))
    && (result == RSI_ERROR_STATE ==> (old_s.pdev == null || old_s.pdev.len == 0))
    && (result == RSI_INCOMPLETE ==> (old_s.pdev == null || old_s.pdev.len == 0))
    && (result == RSI_ERROR_UNKNOWN ==> (old_s.pdev == null || old_s.pdev.len == 0))
    && (result == RSI_SUCCESS ==> (old_s.pdev != null && old_s.pdev.len > 0))
    && (result == RSI_SUCCESS ==> (old_s.pdev.mem_protect_check_range(old_s.pdev.base, old_s.pdev.len) == true))
    && (result == RSI_SUCCESS ==> (new_s.pdev == old_s.pdev))
}