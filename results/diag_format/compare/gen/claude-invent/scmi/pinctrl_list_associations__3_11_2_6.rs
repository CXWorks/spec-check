pub open spec fn pinctrl_list_associations__3_11_2_6_spec(identifier: UInt32, flags: UInt32, index: UInt32, status: i32, ret_flags: UInt32, array: Seq<u16>, old_s: S, new_s: S) -> bool {
    let selector = flags & 0x3u32;
    let selector_valid = selector == 1u32 || selector == 2u32;
    let target_exists = (selector == 1u32 && PinctrlGroupExists(old_s, identifier))
        || (selector == 2u32 && PinctrlFunctionExists(old_s, identifier));
    let supported = PinctrlListAssociationsSupported(old_s, identifier, flags, index);
    let allowed = PinctrlAgentAllowedListAssociations(old_s, identifier, selector);
    let assoc = PinctrlAssociations(old_s, identifier, selector);
    let n = (ret_flags & 0xFFFu32) as int;
    let remaining = (ret_flags >> 16u32) as int;
    (new_s == old_s)
    && ((selector_valid && !target_exists) ==> status == NOT_FOUND)
    && ((selector_valid && target_exists && !supported) ==> status == NOT_SUPPORTED)
    && ((selector_valid && target_exists && supported && !allowed) ==> status == DENIED)
    && ((status == SUCCESS) ==> (selector_valid && target_exists && supported && allowed))
    && ((selector_valid && target_exists && supported && allowed && (index as int) <= assoc.len()) ==> (
        status == SUCCESS
        && (ret_flags & 0xF000u32) == 0u32
        && array.len() == n
        && (index as int) + n <= assoc.len()
        && remaining == assoc.len() - (index as int) - n
        && (forall|i: int| 0 <= i < n ==> array[i] == assoc[(index as int) + i])
        && (forall|i: int, j: int| 0 <= i < j < n ==> array[i] < array[j])
    ))
}
