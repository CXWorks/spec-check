pub open spec fn protocol_version__3_7_2_1_spec(result: Result<(), RmiStatusCode>, version: UInt32, old_s: S, new_s: S) -> bool {
    (ResultEqual(result, SUCCESS) && version == 0x30001)
    && (new_s == old_s)
}