pub open spec fn pinctrl_release__3_11_2_10_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !PinOrGroupExists(old_s, identifier))
    && (result == INVALID_PARAMETERS ==> (flags != 0 || selector != 0 && selector != 1))
    && (result == SUCCESS ==> PinOrGroupExists(old_s, identifier) && !PinOrGroupExists(new_s, identifier))
}