pub open spec fn protocol_attributes__3_7_2_3_spec(attributes: UInt32, max_async: UInt32, num_sensors: UInt32, sensor_reg_address_low: UInt32, sensor_reg_address_high: UInt32, sensor_reg_len: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> attributes[24..32] == 0)
  && (result == RSI_SUCCESS ==> attributes[16..24] == PlatformMaxOutstandingAsyncCommands())
  && (result == RSI_SUCCESS ==> attributes[0..16] == NumSensorsPresent())
  && (result == RSI_SUCCESS && !PlatformImplementsSensorSharedMemory() ==> sensor_reg_len == 0)
  && (result == RSI_SUCCESS && PlatformImplementsSensorSharedMemory() ==> sensor_reg_len == SensorSharedMemoryLength())
  && (result == RSI_SUCCESS && sensor_reg_len != 0 ==> ((sensor_reg_address_high << 32) | sensor_reg_address_low) == SensorSharedMemoryBase() && IsInCallerMemoryMap(SensorSharedMemoryBase()))
  && (result == RSI_SUCCESS && sensor_reg_len == 0 ==> !IsValid(sensor_reg_address_low) && !IsValid(sensor_reg_address_high))
  && ((!(result == RSI_SUCCESS) || PlatformImplementsSensorSharedMemory()) ==> sensor_reg_len == SensorSharedMemoryLength())
}