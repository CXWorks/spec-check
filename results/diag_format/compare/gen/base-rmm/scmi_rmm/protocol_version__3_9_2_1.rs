pub open spec fn protocol_version__3_9_2_1_spec(result: int32, version: uint32, old_s: S, new_s: S) -> bool {
    (StatusIsSuccess(result) ==> version == 0x20001)
    && (old_s == new_s)
}