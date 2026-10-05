pub open spec fn sensor_trip_point_config__3_7_2_9_spec(status: Int32, sensor_id: UInt32, trip_point_ev_ctrl: UInt32, trip_point_val_low: UInt32, trip_point_val_high: UInt32, old_s: S, new_s: S) -> bool {
    (!SensorExists(sensor_id) ==> ResultEqual(status, NOT_FOUND))
    && (Bits(trip_point_ev_ctrl, 11, 4) as int > NumTripPoints(sensor_id) as int ==> ResultEqual(status, INVALID_PARAMETERS))
    && (!TripPointConfigParamsValid(sensor_id, trip_point_ev_ctrl, trip_point_val_low, trip_point_val_high) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (!SensorSupportsTripPointEvents(sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
    && (ResultEqual(status, SUCCESS) ==> (TripPoint(new_s, Bits(trip_point_ev_ctrl, 11, 4)).ev_ctrl == Bits(trip_point_ev_ctrl, 1, 0) && TripPoint(new_s, Bits(trip_point_ev_ctrl, 11, 4)).value == (trip_point_val_high as u64 * 4294967296u64 + trip_point_val_low as u64)))
    && (ResultEqual(status, SUCCESS) ==> (TripPointNotifyEnabled(sensor_id) ==> GeneratesTripPointEventOnCrossing(sensor_id, Bits(trip_point_ev_ctrl, 11, 4))))
}