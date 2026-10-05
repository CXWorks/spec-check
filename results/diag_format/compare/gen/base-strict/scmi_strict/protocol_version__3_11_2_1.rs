pub open spec fn protocol_version__3_11_2_1_spec(status: Int32, version: UInt32, old_s: S, new_s: S) -> bool {
    IsSuccessStatus(status)
    && version == 0x10000
    && old_s == new_s
}