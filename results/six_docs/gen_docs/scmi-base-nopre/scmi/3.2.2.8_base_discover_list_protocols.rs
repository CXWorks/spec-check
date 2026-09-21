pub open spec fn 3.2.2.8_base_discover_list_protocols_spec(result: int32, num_protocols: uint32, protocols: [uint32], old_s: S, new_s: S) -> bool {
    (result == 0 ==> num_protocols == 0)
    && (result == 0 ==> protocols == [0; 1 + (num_protocols - 1) / 4])
    && (result != 0 ==> num_protocols == 0)
    && (result != 0 ==> protocols == [0; 1 + (num_protocols - 1) / 4])
}