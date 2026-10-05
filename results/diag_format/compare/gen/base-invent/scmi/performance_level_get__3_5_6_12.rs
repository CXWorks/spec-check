pub open spec fn performance_level_get__3_5_6_12_spec(result: int32, performance_level: uint32, old_s: S, new_s: S) -> bool {
    (result == 0x80000000 ==> (performance_level == 0))
    && (result != 0x80000000 ==> (result == 0))
    && (result == 0 ==> (old_s.domain_exists(old_s, performance_level as int)))
}