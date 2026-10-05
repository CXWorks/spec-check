pub open spec fn sensor_config_get__3_7_2_10_spec(sensor_id: UInt32, status: Int32, sensor_config: UInt32, old_s: S, new_s: S) -> bool {
    (!SensorExists(old_s, sensor_id) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (
        SensorExists(old_s, sensor_id)
        && (((sensor_config >> 16u32) & 0xFFFFu32) == SensorUpdateIntervalSec(old_s, sensor_id))
        && (((sensor_config >> 11u32) & 0x1Fu32) == SensorUpdateIntervalExponent(old_s, sensor_id))
        && (!SensorSupportsUpdateInterval(old_s, sensor_id) ==> ((sensor_config >> 11u32) & 0x1F_FFFFu32) == 0u32)
        && ((((sensor_config >> 1u32) & 1u32) == 1u32) <==> SensorTimestampEnabled(old_s, sensor_id))
        && (((sensor_config & 1u32) == 1u32) <==> SensorEnabled(old_s, sensor_id))
    ))
    && new_s == old_s
}
