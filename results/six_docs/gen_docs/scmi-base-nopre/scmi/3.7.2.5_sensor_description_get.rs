pub open spec fn sensor_description_get_spec(result: int32, num_sensor_flags: uint32, desc: [SensorDescriptor; 0], old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS)
    && (num_sensor_flags & 0xFFFF == 0)
    && (num_sensor_flags & 0xF0000 == 0)
    && (num_sensor_flags & 0xFFFF != 0 || desc.len() == 0)
    && (forall i: int | 0 <= i && i < desc.len() => new_s.sensors[i] == old_s.sensors[i])
}