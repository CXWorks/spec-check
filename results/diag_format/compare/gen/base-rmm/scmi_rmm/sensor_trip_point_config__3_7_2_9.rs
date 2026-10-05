pub open spec fn sensor_trip_point_config__3_7_2_9_spec(status: Int32, old_s: S, new_s: S) -> bool {
    (!SensorExists(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
    && (trip_point_id > NumTripPoints(old_s, sensor_id) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (!AreLegalTripPointParams(old_s, sensor_id, trip_point_ev_ctrl, trip_point_val_low, trip_point_val_high) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (!SensorSupportsTripPointEvents(old_s, sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
    && (ResultEqual(status, SUCCESS) ==> TripPoint(new_s, sensor_id, trip_point_id).value == (trip_point_val_high << 32) | trip_point_val_low)
    && (ResultEqual(status, SUCCESS) ==> TripPoint(new_s, sensor_id, trip_point_id).ev_ctrl == trip_point_ev_ctrl[1:0])
}