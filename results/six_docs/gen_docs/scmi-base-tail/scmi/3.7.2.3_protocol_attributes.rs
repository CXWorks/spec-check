pub open spec fn protocol_attributes_spec(result: int, attributes: u32, sensor_reg_address_low: u32, sensor_reg_address_high: u32, sensor_reg_len: u32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (attributes & 0xFF000000 == 0)
    && (sensor_reg_len == 0 ==> (sensor_reg_address_low == 0 && sensor_reg_address_high == 0))
    && (sensor_reg_len == 0 ==> (sensor_reg_address_low == 0 && sensor_reg_address_high == 0))
    && (sensor_reg_len == 0 ==> (sensor_reg_address_low == 0 && sensor_reg_address_high == 0))
}