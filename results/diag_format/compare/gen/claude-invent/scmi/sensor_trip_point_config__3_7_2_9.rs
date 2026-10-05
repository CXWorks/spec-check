pub open spec fn sensor_trip_point_config__3_7_2_9_spec(sensor_id: UInt32, trip_point_ev_ctrl: UInt32, trip_point_val_low: UInt32, trip_point_val_high: UInt32, status: i32, old_s: S, new_s: S) -> bool {
    (!SensorExists(old_s, sensor_id) ==> (status == NOT_FOUND && new_s == old_s))
    && ((SensorExists(old_s, sensor_id)
         && (((trip_point_ev_ctrl >> 4u32) & 0xFFu32) as int) > SensorNumTripPoints(old_s, sensor_id) as int)
        ==> ((status == INVALID_PARAMETERS || status == NOT_SUPPORTED) && new_s == old_s))
    && ((SensorExists(old_s, sensor_id)
         && !SensorSupportsTripPoints(old_s, sensor_id))
        ==> ((status == NOT_SUPPORTED || status == INVALID_PARAMETERS) && new_s == old_s))
    && ((status != SUCCESS) ==> new_s == old_s)
    && ((SensorExists(old_s, sensor_id)
         && SensorSupportsTripPoints(old_s, sensor_id)
         && (((trip_point_ev_ctrl >> 4u32) & 0xFFu32) as int) <= SensorNumTripPoints(old_s, sensor_id) as int)
        ==> (status == SUCCESS
             && SensorTripPointEventCtrl(new_s, sensor_id, ((trip_point_ev_ctrl >> 4u32) & 0xFFu32) as int) as int
                == (trip_point_ev_ctrl & 0x3u32) as int
             && SensorTripPointValue(new_s, sensor_id, ((trip_point_ev_ctrl >> 4u32) & 0xFFu32) as int) as int
                == (trip_point_val_high as int) * 0x1_0000_0000 + (trip_point_val_low as int)
             && SensorExists(new_s, sensor_id)
             && SensorNumTripPoints(new_s, sensor_id) == SensorNumTripPoints(old_s, sensor_id)
             && SensorSupportsTripPoints(new_s, sensor_id) == SensorSupportsTripPoints(old_s, sensor_id)
             && TripPointNotifyEnabled(new_s, sensor_id) == TripPointNotifyEnabled(old_s, sensor_id)))
}
