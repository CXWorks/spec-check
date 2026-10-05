pub open spec fn protocol_attributes__3_7_2_3_spec(attributes: UInt32, sensor_reg_address_low: UInt32, sensor_reg_address_high: UInt32, sensor_reg_len: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (Bits(attributes, 31, 24) == 0)
  && (Bits(attributes, 23, 16) == PlatformMaxOutstandingAsyncCommands())
  && (Bits(attributes, 15, 0) == NumSensorsPresent())
  && ((sensor_reg_len == 0) == !PlatformImplementsSensorSharedMemory())
  && (sensor_reg_len != 0 ==> sensor_reg_len == SensorSharedMemoryLength())
  && (sensor_reg_len != 0 ==> (sensor_reg_address_high * 4294967296 + sensor_reg_address_low) == SensorSharedMemoryBase())
  && (sensor_reg_len != 0 ==> sensor_reg_address_low % 8 == 0)
  && (sensor_reg_len != 0 ==> IsInCallingAgentMemoryMap(sensor_reg_address_high * 4294967296 + sensor_reg_address_low))
  && ((!(Bits(attributes, 31, 24) == 0) ||
       !(Bits(attributes, 23, 16) == PlatformMaxOutstandingAsyncCommands()) ||
       !(Bits(attributes, 15, 0) == NumSensorsPresent()) ||
       !((sensor_reg_len == 0) == !PlatformImplementsSensorSharedMemory()) ||
       !(sensor_reg_len != 0 ==> sensor_reg_len == SensorSharedMemoryLength()) ||
       !(sensor_reg_len != 0 ==> (sensor_reg_address_high * 4294967296 + sensor_reg_address_low) == SensorSharedMemoryBase()) ||
       !(sensor_reg_len != 0 ==> sensor_reg_address_low % 8 == 0) ||
       !(sensor_reg_len != 0 ==> IsInCallingAgentMemoryMap(sensor_reg_address_high * 4294967296 + sensor_reg_address_low)))
    ==> result == RSI_SUCCESS)
}