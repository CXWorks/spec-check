pub open spec fn 3.7.2.14_sensor_name_get_spec(result: int32, flags: uint32, name: [uint8; 64], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (flags == 0 && name[63] == 0))
    && (result == SCMI_NOT_FOUND ==> (flags == 0 && name[63] == 0))
    && (result != SCMI_SUCCESS ==> (flags == 0 && name[63] == 0))
    && (result == SCMI_SUCCESS ==> true)
}