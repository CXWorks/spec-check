pub open spec fn 3.6.2.16_clock_parent_get_spec(result: int32, parent_id: uint32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (parent_id == old_s.clock_parent_get_parent_id(old_s, result) as uint32))
    && (result != 0 ==> (result == 0 || result == 1 || result == 2 || result == 3))
}