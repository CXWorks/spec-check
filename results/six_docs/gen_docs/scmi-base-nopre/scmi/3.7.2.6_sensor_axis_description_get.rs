pub open spec fn sensor_axis_description_get_spec(result: int32, num_axis_flags: uint32, desc: [SensorAxisDescriptor], old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> (old_s.sensor_exists(old_s.sensor_id) == false))
    && (result == NOT_SUPPORTED ==> (old_s.sensor_supports_axis(old_s.sensor_id) == false))
    && (result == SUCCESS ==> (old_s.sensor_exists(old_s.sensor_id) == true))
    && (result == SUCCESS ==> (old_s.sensor_supports_axis(old_s.sensor_id) == true))
    && (result == SUCCESS ==> (num_axis_flags & 0x3F) == (desc.len() as uint32))
    && (result == SUCCESS ==> (num_axis_flags & 0x3F00) == 0)
    && (result == SUCCESS ==> (old_s.sensor_axis_descriptors(old_s.sensor_id, old_s.sensor_axis_desc_index, desc.len()) == desc))
    && (result == SUCCESS ==> (new_s.sensor_axis_descriptors(old_s.sensor_id, old_s.sensor_axis_desc_index, desc.len()) == desc))
    && (result == SUCCESS ==> (new_s.sensor_axis_desc_index(old_s.sensor_id) == old_s.sensor_axis_desc_index))
    && (result == SUCCESS ==> (new_s.sensor_axis_desc_index(old_s.sensor_id) == old_s.sensor_axis_desc_index))
}