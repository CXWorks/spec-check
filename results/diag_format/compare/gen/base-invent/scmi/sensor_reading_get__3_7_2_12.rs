pub open spec fn sensor_reading_get__3_7_2_12_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> sensor_id_not_found(old_s, sensor_id))
    && (result == INVALID_PARAMETERS ==> flags_invalid(old_s, flags))
    && (result == PROTOCOL_ERROR ==> sensor_disabled(old_s, sensor_id))
    && (result == SUCCESS ==> (sensor_enabled(old_s, sensor_id) && (flags & 1) == 0))
    && (result == SUCCESS && (flags & 1) == 1 ==> sensor_enqueued_for_async(old_s, sensor_id))
    && (result == SUCCESS ==> sensor_reading_valid(old_s, sensor_id))
}

pub open spec fn sensor_id_not_found(old_s: S, sensor_id: uint32) -> bool {
    true
}

pub open spec fn flags_invalid(old_s: S, flags: uint32) -> bool {
    (flags & 0xFFFFFFFE) != 0
}

pub open spec fn sensor_disabled(old_s: S, sensor_id: uint32) -> bool {
    sensor_state(old_s, sensor_id) == SENSOR_DISABLED
}

pub open spec fn sensor_enabled(old_s: S, sensor_id: uint32) -> bool {
    sensor_state(old_s, sensor_id) == SENSOR_ENABLED
}

pub open spec fn sensor_enqueued_for_async(old_s: S, sensor_id: uint32) -> bool {
    sensor_state(old_s, sensor_id) == SENSOR_ENABLED
}

pub open spec fn sensor_reading_valid(old_s: S, sensor_id: uint32) -> bool {
    true
}