pub open spec fn clock_possible_parents_get__3_6_2_14_spec(clock_id: u32, skip_parents: u32, status: i32, flags: u32, possible_parents: Seq<u32>, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> status == NOT_FOUND)
    && ((ClockExists(old_s, clock_id) && !IsValidSkipParents(old_s, clock_id, skip_parents)) ==> status != SUCCESS)
    && ((ClockExists(old_s, clock_id) && !ClockParentsAdvertisingSupported(old_s, clock_id)) ==> status != SUCCESS)
    && ((ClockExists(old_s, clock_id) && !AgentAllowedToGetClockParents(old_s, clock_id)) ==> status != SUCCESS)
    && (status == NOT_FOUND ==> !ClockExists(old_s, clock_id))
    && (status == OUT_OF_RANGE ==> !IsValidSkipParents(old_s, clock_id, skip_parents))
    && (status == NOT_SUPPORTED ==> !ClockParentsAdvertisingSupported(old_s, clock_id))
    && (status == DENIED ==> !AgentAllowedToGetClockParents(old_s, clock_id))
    && ((ClockExists(old_s, clock_id)
        && IsValidSkipParents(old_s, clock_id, skip_parents)
        && ClockParentsAdvertisingSupported(old_s, clock_id)
        && AgentAllowedToGetClockParents(old_s, clock_id)) ==> status == SUCCESS)
    && (status == SUCCESS ==> (
        ClockExists(old_s, clock_id)
        && IsValidSkipParents(old_s, clock_id, skip_parents)
        && ClockParentsAdvertisingSupported(old_s, clock_id)
        && AgentAllowedToGetClockParents(old_s, clock_id)
        && ((flags >> 8u32) & 0xFFFFu32) == 0
        && possible_parents.len() == (flags & 0xFFu32) as int
        && (skip_parents as int) + ((flags & 0xFFu32) as int) + (((flags >> 24u32) & 0xFFu32) as int)
            == ClockNumPossibleParents(old_s, clock_id) as int
        && (forall|i: int| 0 <= i < possible_parents.len() ==>
            possible_parents[i] == ClockPossibleParentAt(old_s, clock_id, (skip_parents as int) + i))
        && (forall|i: int, j: int| 0 <= i < j < possible_parents.len() ==>
            possible_parents[i] < possible_parents[j])
    ))
    && new_s == old_s
}
