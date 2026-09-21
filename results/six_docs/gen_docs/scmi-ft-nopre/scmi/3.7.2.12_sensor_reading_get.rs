pub open spec fn 3.7.2.12_sensor_reading_get_spec(sensor_id: UInt32, flags: UInt32, result: RsiCommandReturnCode, sensor_value_low: Int32, sensor_value_high: Int32, timestamp_low: UInt32, timestamp_high: UInt32, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> sensor_value_low != 0)
  && (result == RSI_SUCCESS ==> sensor_value_high != 0)
  && (result == RSI_SUCCESS ==> timestamp_low != 0)
  && (result == RSI_SUCCESS ==> timestamp_high != 0)
  && ((!(result == RSI_SUCCESS))
    ==> sensor_value_low == 0)
  && ((!(result == RSI_SUCCESS))
    ==> sensor_value_high == 0)
  && ((!(result == RSI_SUCCESS))
    ==> timestamp_low == 0)
  && ((!(result == RSI_SUCCESS))
    ==> timestamp_high == 0)
}