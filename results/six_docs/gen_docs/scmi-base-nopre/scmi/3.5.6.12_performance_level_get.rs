pub open spec fn 3.5.6.12_performance_level_get_spec(result: int32, performance_level: uint32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> performance_level == old_s.performance_level_get(old_s, result))
    && (result != 0 ==> result == NOT_FOUND)
}