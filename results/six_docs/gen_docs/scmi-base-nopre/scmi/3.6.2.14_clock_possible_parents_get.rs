pub open spec fn clock_possible_parents_get_spec(result: int32, flags: uint32, possible_parents: array<uint32>, old_s: S, new_s: S) -> bool {
    (result == 0)
    && (flags as int >= 0)
    && (flags as int <= 255)
    && (possible_parents.len() as int == (flags as int) & 0xFF)
    && (old_s == new_s)
}