pub open spec fn sensor_trip_point_config__3_7_2_9_spec(sensor_id: UInt32, trip_point_id: UInt8, trip_point_ev_ctrl: UInt2, trip_point_val_low: UInt32, trip_point_val_high: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!SensorExists(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (trip_point_id > NumTripPoints(old_s, sensor_id) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!AreLegalTripPointParams(old_s, sensor_id, trip_point_ev_ctrl, trip_point_val_low, trip_point_val_high) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!SensorSupportsTripPointEvents(old_s, sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> TripPoint(new_s, sensor_id, trip_point_id as int).value == (trip_point_val_high << 32) | trip_point_val_low)
  && (ResultEqual(status, SUCCESS) ==> TripPoint(new_s, sensor_id, trip_point_id as int).ev_ctrl == trip_point_ev_ctrl[1:0])
  && ((SensorExists(old_s, sensor_id) &&
       !(trip_point_id > NumTripPoints(old_s, sensor_id)) &&
       AreLegalTripPointParams(old_s, sensor_id, trip_point_ev_ctrl, trip_point_val_low, trip_point_val_high) &&
       SensorSupportsTripPointEvents(old_s, sensor_id))
    ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, NOT_FOUND)
    ==> TripPoint(new_s, sensor_id, trip_point_id as int).value == TripPoint(old_s, sensor_id, trip_point_id as int).value)
  && (ResultEqual(status, NOT_FOUND)
    ==> TripPoint(new_s, sensor_id, trip_point_id as int).ev_ctrl == TripPoint(old_s, sensor_id, trip_point_id as int).ev_ctrl)
  && (ResultEqual(status, INVALID_PARAMETERS)
    ==> TripPoint(new_s, sensor_id, trip_point_id as int).value == TripPoint(old_s, sensor_id, trip_point_id as int).value)
  && (ResultEqual(status, INVALID_PARAMETERS)
    ==> TripPoint(new_s, sensor_id, trip_point_id as int).ev_ctrl == TripPoint(old_s, sensor_id, trip_point_id as int).ev_ctrl)
  && (ResultEqual(status, NOT_SUPPORTED)
    ==> TripPoint(new_s, sensor_id, trip_point_id as int).value == TripPoint(old_s, sensor_id, trip_point_id as int).value)
  && (ResultEqual(status, NOT_SUPPORTED)
    ==> TripPoint(new_s, sensor_id, trip_point_id as int).ev_ctrl == TripPoint(old_s, sensor_id, trip_point_id as int).ev_ctrl)
  && (status != SUCCESS
    ==> TripPoint(new_s, sensor_id, trip_point_id as int).value == TripPoint(old_s, sensor_id, trip_point_id as int).value)
  && (status != SUCCESS
    ==> TripPoint(new_s, sensor_id, trip_point_id as int).ev_ctrl == TripPoint(old_s, sensor_id, trip_point_id as int).ev_ctrl)
}