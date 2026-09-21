pub open spec fn 3.12.4.12_telemetry_reset_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (new_s.telemetry_reset == true))
    && (result != 0 ==> (new_s.telemetry_reset == false))
}