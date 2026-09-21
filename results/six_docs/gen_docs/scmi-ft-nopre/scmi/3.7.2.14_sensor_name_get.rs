pub open spec fn 3.7.2.14_sensor_name_get_spec(sensor_id: UInt32, result: RsiCommandReturnCode, flags: UInt32, name: [u8; 64], old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> flags == 0)
  && (result != RSI_SUCCESS ==> flags == 0)
  && (result == RSI_SUCCESS ==> name[0] == 0)
  && ((!(result == RSI_SUCCESS))
    ==> name[0] == 0)
}