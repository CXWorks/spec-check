pub open spec fn protocol_attributes__3_7_2_3_spec(status: i32, attributes: u32, sensor_reg_address_low: u32, sensor_reg_address_high: u32, sensor_reg_len: u32, old_s: S, new_s: S) -> bool {
    (status == 0i32 ==> (
        ((attributes >> 24u32) == 0u32)
        && (((attributes >> 16u32) & 0xFFu32) as int == MaxOutstandingAsyncCommandsSupported(old_s))
        && ((attributes & 0xFFFFu32) as int == NumSensorsPresent(old_s))
        && (sensor_reg_len == 0u32 <==> !SensorSharedMemoryImplemented(old_s))
        && (sensor_reg_len != 0u32 ==> (
            ((sensor_reg_address_low as int) % 8 == 0)
            && (((sensor_reg_address_high as int) * 0x1_0000_0000 + (sensor_reg_address_low as int)) == SensorSharedMemoryBase(old_s))
            && ((sensor_reg_len as int) == SensorSharedMemoryLength(old_s))
            && AddrInAgentMemoryMap(old_s, (sensor_reg_address_high as int) * 0x1_0000_0000 + (sensor_reg_address_low as int))
        ))
    ))
    && (new_s == old_s)
}
