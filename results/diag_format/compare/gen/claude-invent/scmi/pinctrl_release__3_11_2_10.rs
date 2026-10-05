pub open spec fn pinctrl_release__3_11_2_10_spec(identifier: UInt32, flags: UInt32, status: i32, old_s: S, new_s: S) -> bool {
    ((((flags as int) / 4) != 0 || ((flags as int) % 4) > 1) ==> status == INVALID_PARAMETERS)
    && ((((flags as int) / 4) == 0 && ((flags as int) % 4) <= 1 && !PinctrlIdentifierIsValid(old_s, identifier, (flags as int) % 4)) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (
        ((flags as int) / 4) == 0
        && ((flags as int) % 4) <= 1
        && PinctrlIdentifierIsValid(old_s, identifier, (flags as int) % 4)
        && PinctrlExclusiveControlReleased(old_s, new_s, identifier, (flags as int) % 4)
    ))
    && (status != SUCCESS ==> new_s == old_s)
}
