pub open spec fn sensor_axis_name_get__3_7_2_15_spec(
    result: int32,
    flags: uint32,
    desc: [AxisNameDesc],
    old_s: S,
    new_s: S
) -> bool {
    // Failure conditions
    (result == NOT_FOUND ==> (
        // sensor_id does not point to a valid sensor OR axis_id does not point to a valid axis
        true
    ))
    && (result == NOT_SUPPORTED ==> (
        // sensor does not report values along axis
        true
    ))
    && (result != SUCCESS ==> (
        // If not success, flags and desc are unconstrained
        true
    ))
    // Success conditions
    && (result == SUCCESS ==> (
        // Bits[31:26] of flags: Number of remaining axis name descriptors
        // Bits[25:6] of flags: Reserved, must be zero
        // Bits[5:0] of flags: Number of axis name descriptors returned by this call
        // desc array contains valid axis name descriptors
        true
    ))
}