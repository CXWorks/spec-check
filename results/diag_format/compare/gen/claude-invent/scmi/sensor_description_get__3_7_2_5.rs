pub open spec fn sensor_description_get__3_7_2_5_spec(desc_index: u32, status: i32, num_sensor_flags: u32, old_s: S, new_s: S) -> bool {
    (status == 0i32 ==> (
        (num_sensor_flags & 0xF000u32) == 0u32
        && (desc_index as int) + ((num_sensor_flags & 0xFFFu32) as int) + ((num_sensor_flags >> 16u32) as int) == SensorDescriptorCount(old_s)
        && ReturnedSensorDescriptorsMatch(old_s, new_s, desc_index as int, (num_sensor_flags & 0xFFFu32) as int)
        && new_s == old_s
    ))
    && (status != 0i32 ==> new_s == old_s)
}
