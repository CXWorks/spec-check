pub open spec fn sensor_axis_name_get_spec(result: int32, flags: uint32, desc: [AxisNameDesc], old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> (old_s.sensor_exists(old_s.sensor_id) == false || old_s.axis_exists(old_s.sensor_id, old_s.axis_id) == false))
    && (result == NOT_SUPPORTED ==> old_s.sensor_exists(old_s.sensor_id) && old_s.axis_exists(old_s.sensor_id, old_s.axis_id) && old_s.sensor_axis_supports(old_s.sensor_id, old_s.axis_id) == false)
    && (result == SUCCESS ==> (old_s.sensor_exists(old_s.sensor_id) && old_s.axis_exists(old_s.sensor_id, old_s.axis_id) && old_s.sensor_axis_supports(old_s.sensor_id, old_s.axis_id)))
    && (result == SUCCESS ==> (flags & 0x3F) == (desc.len() as uint32))
    && (result == SUCCESS ==> (flags & 0x3F00) == 0)
    && (result == SUCCESS ==> (flags & 0xFC000000) == 0)
    && (result == SUCCESS ==> (old_s.sensor_id == new_s.sensor_id))
    && (result == SUCCESS ==> (old_s.axis_id == new_s.axis_id))
}