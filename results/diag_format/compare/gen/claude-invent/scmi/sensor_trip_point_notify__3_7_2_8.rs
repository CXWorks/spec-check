pub open spec fn sensor_trip_point_notify__3_7_2_8_spec(sensor_id: u32, sensor_event_control: u32, status: i32, old_s: S, new_s: S) -> bool {
    (!SensorExists(old_s, sensor_id) ==> (status == NOT_FOUND && new_s == old_s))
    && ((SensorExists(old_s, sensor_id) && !SensorEventControlIsValid(sensor_event_control)) ==> (status == INVALID_PARAMETERS && new_s == old_s))
    && ((SensorExists(old_s, sensor_id) && SensorEventControlIsValid(sensor_event_control) && !SensorTripPointNotifySupported(old_s, sensor_id)) ==> (status == NOT_SUPPORTED && new_s == old_s))
    && ((SensorExists(old_s, sensor_id) && SensorEventControlIsValid(sensor_event_control) && SensorTripPointNotifySupported(old_s, sensor_id)) ==> (status == SUCCESS && SensorTripPointNotifyEnabled(new_s, sensor_id) == ((sensor_event_control & 1u32) == 1u32) && SensorTripPointConfigsUnchanged(old_s, new_s) && SensorTripPointNotifyUnchangedExcept(old_s, new_s, sensor_id)))
}
