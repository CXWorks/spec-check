pub open spec fn sensor_description_get__3_7_2_5_spec(desc_index: UInt32, result: RsiCommandReturnCode, num_remaining: UInt16, reserved: UInt16, num_returned: UInt12, desc: [SENSOR_DESC; 1], old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> desc[0] == SensorDescriptorArray(new_s, desc_index))
  && (result == RSI_SUCCESS ==> num_sensor_flags(new_s, 11, 0) == 1)
  && (result == RSI_SUCCESS ==> num_sensor_flags(new_s, 31, 16) == NumRemainingSensorDescriptors(new_s, desc_index, 1))
  && (result == RSI_SUCCESS ==> num_sensor_flags(new_s, 15, 12) == 0)
  && (result == RSI_SUCCESS ==> desc[i].sensor_attributes_high[8] == 1 ==> desc[i].sensor_attributes_high[21, 16] != 0)
  && (result == RSI_SUCCESS ==> desc[i].sensor_power == PreviousSensorDescription(new_s, desc[i].sensor_id).sensor_power)
  && ((!(result == RSI_SUCCESS))
    ==> SensorDescriptorArray(new_s, desc_index) == SensorDescriptorArray(old_s, desc_index))
  && ((!(result == RSI_SUCCESS))
    ==> num_sensor_flags(new_s, 11, 0) == num_sensor_flags(old_s, 11, 0))
  && ((!(result == RSI_SUCCESS))
    ==> num_sensor_flags(new_s, 31, 16) == num_sensor_flags(old_s, 31, 16))
  && ((!(result == RSI_SUCCESS))
    ==> num_sensor_flags(new_s, 15, 12) == num_sensor_flags(old_s, 15, 12))
  && (result != RSI_SUCCESS
    ==> desc[0] == desc[0])
  && (result != RSI_SUCCESS
    ==> num_sensor_flags(new_s, 11, 0) == num_sensor_flags(old_s, 11, 0))
  && (result != RSI_SUCCESS
    ==> num_sensor_flags(new_s, 31, 16) == num_sensor_flags(old_s, 31, 16))
  && (result != RSI_SUCCESS
    ==> num_sensor_flags(new_s, 15, 12) == num_sensor_flags(old_s, 15, 12))
}