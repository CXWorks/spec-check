pub open spec fn 3.7.2.9_sensor_trip_point_config_spec(sensor_id: UInt32, trip_point_ev_ctrl: UInt32, trip_point_val_low: UInt32, trip_point_val_high: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
}