pub open spec fn protocol_attributes__3_7_2_3_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS)
    && (old_s.attributes[31:24] == 0)
    && (old_s.attributes[23:16] == PlatformMaxOutstandingAsyncCommands())
    && (old_s.attributes[15:0] == NumSensorsPresent())
    && (!PlatformImplementsSensorSharedMemory() ==> old_s.sensor_reg_len == 0)
    && (PlatformImplementsSensorSharedMemory() ==> old_s.sensor_reg_len == SensorSharedMemoryLength())
    && (old_s.sensor_reg_len != 0 ==> (((old_s.sensor_reg_address_high << 32) | old_s.sensor_reg_address_low) == SensorSharedMemoryBase()) && IsInCallerMemoryMap(SensorSharedMemoryBase()))
    && (old_s.sensor_reg_len == 0 ==> (!IsValid(old_s.sensor_reg_address_low) && !IsValid(old_s.sensor_reg_address_high)))
}