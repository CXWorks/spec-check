pub open spec fn 3.4.2.5_system_power_state_set_spec(result: int32, old_s: S, new_s: S) -> bool {
    let flags: UInt32 = old_s.cmd_input_flags;
    let system_state: UInt32 = old_s.cmd_input_system_state;
    let is_graceful: bool = flags & 1 != 0;
    let is_valid_state: bool = system_state <= 0x7FFFFFFF && system_state != 0x80000000;
    let is_shutdown: bool = system_state == 0x0;
    let is_cold_reset: bool = system_state == 0x1;
    let is_warm_reset: bool = system_state == 0x2;
    let is_power_up: bool = system_state == 0x3;
    let is_suspend: bool = system_state == 0x4;
    let is_vendor_defined: bool = system_state >= 0x80000000;
    let is_reserved: bool = system_state > 0x7FFFFFFF && system_state < 0x80000000;
    let is_invalid_state: bool = is_reserved;
    let is_forceful: bool = !is_graceful;
    let is_power_up_request: bool = is_power_up;
    let is_graceful_for_power_up: bool = is_graceful && is_power_up_request;
    let is_graceful_for_other: bool = is_graceful && !is_power_up_request;
    let is_valid_for_graceful: bool = is_graceful_for_power_up || is_graceful_for_other;
    let is_valid_for_forceful: bool = !is_graceful;
    let is_valid_flags: bool = (flags & 0xFFFFFFFE) == 0;
    let is_valid_input: bool = is_valid_flags && !is_invalid_state;
    let is_suspend_denied: bool = is_suspend && (new_s.running_app_count > 0 || new_s.idle_app_count > 0);
    let is_suspend_allowed: bool = !is_suspend_denied;
    let is_power_up_allowed: bool = true;
    let is_reset_allowed: bool = true;
    let is_shutdown_allowed: bool = true;
    let is_vendor_allowed: bool = is_vendor_defined;
    let is_supported: bool = is_power_up_allowed || is_reset_allowed || is_shutdown_allowed || is_vendor_allowed;
    let is_denied: bool = is_suspend && !is_suspend_allowed;
    let is_not_supported: bool = !is_supported;
    let is_invalid_params: bool = !is_valid_input;
    let is_success: bool = result == 0;
    let is_error: bool = result != 0;
    let is_error_invalid_params: bool = result == -1;
    let is_error_not_supported: bool = result == -2;
    let is_error_denied: bool = result == -3;
    (!is_valid_flags ==> is_error_invalid_params)
    && (!is_valid_state ==> is_error_invalid_params)
    && (is_invalid_state ==> is_error_invalid_params)
    && (is_suspend && !is_suspend_allowed ==> is_error_denied)
    && (!is_supported ==> is_error_not_supported)
    && (is_success ==> (is_valid_input && is_supported))
    && (is_error_invalid_params ==> (!is_valid_input || is_invalid_state))
    && (is_error_not_supported ==> is_supported)
    && (is_error_denied ==> is_suspend && !is_suspend_allowed)
}