pub open spec fn base_discover_list_protocols__3_2_2_8_spec(result: int32, num_protocols: uint32, protocols: uint32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (num_protocols >= 0 && protocols == 0))
    && (result != 0 ==> (num_protocols == 0 && protocols == 0))
}