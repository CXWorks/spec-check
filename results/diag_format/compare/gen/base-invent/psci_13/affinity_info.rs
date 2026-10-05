pub open spec fn affinity_info_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.lowest_affinity_level as int < 0 || old_s.lowest_affinity_level as int > old_s.num_affinity_levels as int || old_s.target_affinity as int < 0 || old_s.target_affinity as int >= (1u64 << old_s.num_affinity_levels as int)))
    && (result == RSI_ERROR_STATE ==> (old_s.lowest_affinity_level as int >= old_s.num_affinity_levels as int || old_s.target_affinity as int >= (1u64 << old_s.num_affinity_levels as int)))
    && (result == RSI_INCOMPLETE ==> true)
    && (result == RSI_ERROR_UNKNOWN ==> true)
    && (result == RSI_SUCCESS ==> true)
}