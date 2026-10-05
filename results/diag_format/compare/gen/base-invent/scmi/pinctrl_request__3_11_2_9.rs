pub open spec fn pinctrl_request__3_11_2_9_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !IsPinOrGroupValid(old_s, identifier(old_s)))
    && (result == INVALID_PARAMETERS ==> (flags(old_s) & 0x3 != 0 || (flags(old_s) & 0xFFFFFFFC) != 0))
    && (result == DENIED ==> !AgentCanRequestPinOrGroup(old_s, identifier(old_s)))
    && (result == IN_USE ==> IsPinOrGroupInUse(old_s, identifier(old_s)))
    && (result == SUCCESS ==> (IsPinOrGroupValid(old_s, identifier(old_s)) && (flags(old_s) & 0x3 == 0 || (flags(old_s) & 0x3 == 1)) && AgentCanRequestPinOrGroup(old_s, identifier(old_s)) && !IsPinOrGroupInUse(old_s, identifier(old_s))))
    && (result == SUCCESS ==> PinOrGroupControlledBy(old_s, identifier(old_s), new_s))
    && (result == SUCCESS ==> PinOrGroupControlledBy(old_s, identifier(old_s), new_s))
}