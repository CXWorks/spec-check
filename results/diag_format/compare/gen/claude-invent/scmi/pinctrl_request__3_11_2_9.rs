pub open spec fn pinctrl_request__3_11_2_9_spec(status: i32, identifier: u32, flags: u32, old_s: S, new_s: S) -> bool {
    (((flags & 0xFFFF_FFFCu32) != 0u32 || (flags & 0x3u32) > 1u32) ==> (status == INVALID_PARAMETERS && new_s == old_s))
    && (((flags & 0xFFFF_FFFCu32) == 0u32 && (flags & 0x3u32) <= 1u32
        && !PinctrlIdentifierValid(old_s, identifier, flags & 0x3u32))
        ==> (status == NOT_FOUND && new_s == old_s))
    && (((flags & 0xFFFF_FFFCu32) == 0u32 && (flags & 0x3u32) <= 1u32
        && PinctrlIdentifierValid(old_s, identifier, flags & 0x3u32)
        && !PinctrlAgentAllowed(old_s, CallerAgent(old_s), identifier, flags & 0x3u32))
        ==> (status == DENIED && new_s == old_s))
    && (((flags & 0xFFFF_FFFCu32) == 0u32 && (flags & 0x3u32) <= 1u32
        && PinctrlIdentifierValid(old_s, identifier, flags & 0x3u32)
        && PinctrlAgentAllowed(old_s, CallerAgent(old_s), identifier, flags & 0x3u32)
        && PinctrlIsExclusivelyControlledByOther(old_s, CallerAgent(old_s), identifier, flags & 0x3u32))
        ==> (status == IN_USE && new_s == old_s))
    && (((flags & 0xFFFF_FFFCu32) == 0u32 && (flags & 0x3u32) <= 1u32
        && PinctrlIdentifierValid(old_s, identifier, flags & 0x3u32)
        && PinctrlAgentAllowed(old_s, CallerAgent(old_s), identifier, flags & 0x3u32)
        && !PinctrlIsExclusivelyControlledByOther(old_s, CallerAgent(old_s), identifier, flags & 0x3u32))
        ==> (status == SUCCESS
            && PinctrlIsExclusivelyControlledBy(new_s, CallerAgent(old_s), identifier, flags & 0x3u32)))
}
