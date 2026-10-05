pub open spec fn protocol_attributes__3_7_2_3_spec(result: int32, attributes: uint32, sensor_reg_address_low: uint32, sensor_reg_address_high: uint32, sensor_reg_len: uint32, old_s: S, new_s: S) -> bool {
    (result == 0)
    && (attributes & 0xFF000000 == 0)
    && (sensor_reg_address_low as int) % 8 == 0
    && (sensor_reg_address_high as int) % 8 == 0
    && (sensor_reg_len as int) % 8 == 0
    && (sensor_reg_len == 0 ==> (sensor_reg_address_low == 0 && sensor_reg_address_high == 0))
    && (old_s == new_s)
}