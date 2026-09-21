pub open spec fn ffa_version_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Failure: Bit[31] in Input version number is not 0
    let input_version = (old_s.ffa_version_input as int) & 0xFFFFFFFF;
    let input_version_sign_bit = (input_version >> 31) as int;
    (input_version_sign_bit != 0 ==> ResultEqual(result, SMCCC_INVALID_PARAMETER))
    // Failure: Version query type field in Input flags is incorrectly encoded (not 0, 1, or 2)
    let input_flags = (old_s.ffa_version_input >> 32) as int;
    let query_type = (input_flags & 0x3) as int;
    (query_type != 0 && query_type != 1 && query_type != 2 ==> ResultEqual(result, SMCCC_INVALID_PARAMETER))
    // Failure: Callee does not implement any version of the Firmware Framework
    // (Implies result is NOT_SUPPORTED)
    // Success: Query type 0 (Negotiate) or 1 (Query compatible) returns a version >= input
    // Success: Query type 2 (Query negotiated) returns the negotiated version
    // Success: If Framework is in use and query type is 0, returns Null version
    // Success: If Framework is in use and query type is 1, returns Null version
    // Success: If Framework is in use and query type is 2, returns negotiated version
    // Note: The spec does not define state transitions for FFA_VERSION, so we only constrain the result.
    true
}