pub open spec fn sensor_continuous_update_notify_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> sensor_id_is_invalid(old_s, sensor_id))
    && (result == NOT_SUPPORTED ==> !sensor_supports_continuous_update(old_s, sensor_id))
    && (result == INVALID_PARAMETERS ==> notify_enable_is_invalid(old_s, notify_enable))
    && (result == SUCCESS ==> sensor_id_is_valid(old_s, sensor_id) && sensor_supports_continuous_update(old_s, sensor_id) && notify_enable_is_valid(old_s, notify_enable))
}

pub open spec fn sensor_id_is_invalid(old_s: S, sensor_id: uint32) -> bool {
    sensor_id == 0
}

pub open spec fn sensor_supports_continuous_update(old_s: S, sensor_id: uint32) -> bool {
    true
}

pub open spec fn notify_enable_is_invalid(old_s: S, notify_enable: uint32) -> bool {
    notify_enable != 0 && notify_enable != 1
}

pub open spec fn notify_enable_is_valid(old_s: S, notify_enable: uint32) -> bool {
    notify_enable == 0 || notify_enable == 1
}