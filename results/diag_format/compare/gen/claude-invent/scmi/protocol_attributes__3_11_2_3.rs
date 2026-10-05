pub open spec fn protocol_attributes__3_11_2_3_spec(status: i32, attributes_low: u32, attributes_high: u32, old_s: S, new_s: S) -> bool {
    (status == 0 ==> (
        ((attributes_low >> 16u32) as int) == PinctrlNumPinGroups(old_s)
        && ((attributes_low & 0xFFFFu32) as int) == PinctrlNumPins(old_s)
        && (attributes_high >> 16u32) == 0u32
        && ((attributes_high & 0xFFFFu32) as int) == PinctrlNumFunctions(old_s)
    ))
    && new_s == old_s
}
