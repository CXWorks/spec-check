pub open spec fn drtm_enable_secure_interrupts_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.drtm_parameters.secure_interrupts_disabled == false))
    && (result == RSI_ERROR_STATE ==> (old_s.drtm_parameters.secure_interrupts_disabled == false))
    && (result == RSI_SUCCESS ==> (old_s.drtm_parameters.secure_interrupts_disabled == true))
    && (result == RSI_SUCCESS ==> (new_s.drtm_parameters.secure_interrupts_disabled == true))
    && (result == RSI_SUCCESS ==> (new_s.secure_interrupts_enabled == true))
    && (result == RSI_SUCCESS ==> (old_s.secure_interrupts_enabled == false))
    && (result == RSI_SUCCESS ==> (new_s.drtm_parameters == old_s.drtm_parameters))
    && (result == RSI_SUCCESS ==> (new_s.secure_interrupts_enabled == true))
}