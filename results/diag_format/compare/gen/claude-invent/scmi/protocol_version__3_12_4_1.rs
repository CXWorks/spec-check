pub open spec fn protocol_version__3_12_4_1_spec(status: i32, version: UInt32, old_s: S, new_s: S) -> bool {
    (status == 0 ==> (version as int) == 0x10000)
    && (new_s == old_s)
}
