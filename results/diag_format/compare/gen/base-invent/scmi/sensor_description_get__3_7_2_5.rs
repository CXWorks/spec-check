pub open spec fn sensor_description_get__3_7_2_5_spec(result: int32, num_sensor_flags: uint32, desc: [SensorDescriptor; 0], old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS)
    && (num_sensor_flags == 0)
    && (desc.len() == 0)
    && (old_s == new_s)
}