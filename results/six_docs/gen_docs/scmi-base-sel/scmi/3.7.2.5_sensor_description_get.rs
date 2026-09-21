pub open spec fn 3_7_2_5_sensor_description_get_spec(result: int32, num_sensor_flags: uint32, desc: [SENSOR_DESC], old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (num_sensor_flags == 0 || (num_sensor_flags & 0xFFF) > 0))
    && (result == SCMI_SUCCESS ==> (num_sensor_flags & 0xF000) == 0)
    && (result == SCMI_SUCCESS ==> (desc.len() as int == (num_sensor_flags & 0xFFF) as int))
    && (result != SCMI_SUCCESS ==> (num_sensor_flags == 0))
    && (result != SCMI_SUCCESS ==> (desc.len() == 0))
    && (result == SCMI_SUCCESS ==> (old_s == new_s))
}