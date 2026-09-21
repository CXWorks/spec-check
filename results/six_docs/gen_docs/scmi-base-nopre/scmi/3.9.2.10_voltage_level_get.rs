pub open spec fn voltage_level_get_spec(result: int32, voltage_level: int32, old_s: S, new_s: S) -> bool {
    (result == 0x80000000 ==> (voltage_level == 0))
    && (result == 0x80000001 ==> (voltage_level == 0))
    && (result == 0x80000002 ==> (voltage_level == 0))
    && (result == 0x80000003 ==> (voltage_level == 0))
    && (result == 0 ==> (voltage_level == old_s.voltage_level_get(old_s.domain_id)))
    && (result != 0 ==> (voltage_level == 0))
}