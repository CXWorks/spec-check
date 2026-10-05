pub open spec fn protocol_attributes__3_7_2_3_spec(result: RsiCommandReturnCode, old_s: S, new_s: S, attributes: UInt32, sensor_reg_address_low: UInt32, sensor_reg_address_high: UInt32, sensor_reg_len: UInt32) -> bool {
    (result == RSI_SUCCESS)
    && (Bits64(attributes, 31, 24) == 0)
    && (Bits64(attributes, 23, 16) == PlatformMaxOutstandingAsyncCommands())
    && (Bits64(attributes, 15, 0) == NumSensorsPresent())
    && ((sensor_reg_len == 0) == !PlatformImplementsSensorSharedMemory())
    && (sensor_reg_len != 0 ==> sensor_reg_len == SensorSharedMemoryLength())
    && (sensor_reg_len != 0 ==> (sensor_reg_address_high as int * 4294967296 + sensor_reg_address_low as int) == SensorSharedMemoryBase())
    && (sensor_reg_len != 0 ==> (sensor_reg_address_low as int) % 8 == 0)
    && (sensor_reg_len != 0 ==> IsInCallingAgentMemoryMap(sensor_reg_address_high as int * 4294967296 + sensor_reg_address_low as int))
}