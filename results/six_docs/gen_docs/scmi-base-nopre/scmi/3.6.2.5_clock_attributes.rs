pub open spec fn clock_attributes_spec(result: i32, attributes: u32, clock_name: [u8; 16], clock_enable_delay: u32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (attributes >= 0 && clock_name[0] == 0 && clock_enable_delay >= 0))
    && (result != 0 ==> true)
}