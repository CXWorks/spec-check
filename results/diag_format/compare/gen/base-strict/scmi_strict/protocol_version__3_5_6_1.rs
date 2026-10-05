pub open spec fn protocol_version__3_5_6_1_spec(status: Int32, version: UInt32, old_s: S, new_s: S) -> bool {
    StatusIsSuccess(status)
    && version == 0x40001
    && old_s == new_s
}