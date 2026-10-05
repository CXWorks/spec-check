pub open spec fn sensor_trip_point_config__3_7_2_9_spec(sensor_id: UInt32, trip_point_ev_ctrl: UInt32, trip_point_val_low: UInt32, trip_point_val_high: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!SensorExists(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (Bits(trip_point_ev_ctrl, 11, 4) > NumTripPoints(old_s, sensor_id) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!TripPointConfigParamsValid(old_s, sensor_id, trip_point_ev_ctrl, trip_point_val_low, trip_point_val_high) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!SensorSupportsTripPointEvents(old_s, sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> TripPoint(new_s, sensor_id, Bits(trip_point_ev_ctrl, 11, 4)).ev_ctrl == Bits(trip_point_ev_ctrl, 1, 0))
  && (ResultEqual(status, SUCCESS) ==> TripPoint(new_s, sensor_id, Bits(trip_point_ev_ctrl, 11, 4)).value == trip_point_val_high * 4294967296 + trip_point_val_low)
  && (ResultEqual(status, SUCCESS) ==> TripPointNotifyEnabled(new_s, sensor_id) ==> GeneratesTripPointEventOnCrossing(new_s, sensor_id, Bits(trip_point_ev_ctrl, 11, 4)))
  && ((SensorExists(old_s, sensor_id) &&
       !(Bits(trip_point_ev_ctrl, 11, 4) > NumTripPoints(old_s, sensor_id)) &&
       TripPointConfigParamsValid(old_s, sensor_id, trip_point_ev_ctrl, trip_point_val_low, trip_point_val_high) &&
       SensorSupportsTripPointEvents(old_s, sensor_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> TripPoint(new_s, sensor_id, Bits(trip_point_ev_ctrl, 11, 4)).ev_ctrl == TripPoint(old_s, sensor_id, Bits(trip_point_ev_ctrl, 11, 4)).ev_ctrl)
  && (result != SUCCESS
    ==> TripPoint(new_s, sensor_id, Bits(trip_point_ev_ctrl, 11, 4)).value == TripPoint(old_s, sensor_id, Bits(trip_point_ev_ctrl, 11, 4)).value)
}