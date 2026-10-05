pub open spec fn cpu_suspend_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    // Failure: INVALID_PARAMETERS
    // - power_state parameter is invalid (reserved bits non-zero, invalid StateType, etc.)
    // - OS-initiated mode: request for higher-than-core-level topology node
    // - OS-initiated mode: at least one child in incompatible local low-power state
    // - System state inconsistent with request (nodes in low-power state)
    // Failure: INVALID_ADDRESS
    // - entry_point_address is invalid (in unavailable range)
    // Failure: DENIED
    // - OS-initiated mode: request for higher-than-core-level topology node AND all incompatible cores are running
    (!ResultEqual(result, RSI_SUCCESS) ==> (
        (ResultEqual(result, RSI_ERROR_INPUT)) ||
        (ResultEqual(result, RSI_ERROR_STATE)) ||
        (ResultEqual(result, RSI_INCOMPLETE)) ||
        (ResultEqual(result, RSI_ERROR_UNKNOWN))
    ))
    && (ResultEqual(result, RSI_SUCCESS) ==> true)
}