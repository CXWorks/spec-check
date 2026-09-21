pub open spec fn protocol_version_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0x30000)
}