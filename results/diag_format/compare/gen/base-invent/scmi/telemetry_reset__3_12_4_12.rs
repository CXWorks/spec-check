pub open spec fn telemetry_reset__3_12_4_12_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0x1B_0000_0000) ==> (old_s.telemetry_flags == 0)
    && (result == 0x1B_0000_0001) ==> (old_s.telemetry_flags != 0)
    && (result == 0x1B_0000_0002) ==> (old_s.telemetry_flags != 0)
    && (result == 0x1B_0000_0000) ==> (new_s.telemetry_flags == 0)
    && (result == 0x1B_0000_0000) ==> (new_s.telemetry_data == 0)
    && (result == 0x1B_0000_0000) ==> (new_s.telemetry_config == 0)
    && (result != 0x1B_0000_0000) ==> (new_s.telemetry_flags == old_s.telemetry_flags)
    && (result != 0x1B_0000_0000) ==> (new_s.telemetry_data == old_s.telemetry_data)
    && (result != 0x1B_0000_0000) ==> (new_s.telemetry_config == old_s.telemetry_config)
}