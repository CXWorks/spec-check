pub open spec fn cpu_suspend_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state & 0x3FF) != 0
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.flags & 0x2) == 0 && (old_s.power_state & 0x3F) != 0 ||
        (old_s.flags & 0x2) != 0 && (old_s.power_state &