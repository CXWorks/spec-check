pub open spec fn clock_attributes__3_6_2_5_spec(result: int32, attributes: uint32, clock_name: [uint8; 16], clock_enable_delay: uint32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (attributes >= 0 && clock_name[15] == 0 && clock_enable_delay >= 0))
    && (result != 0 ==> true)
}