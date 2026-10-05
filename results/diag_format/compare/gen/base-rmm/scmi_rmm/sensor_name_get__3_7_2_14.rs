pub open spec fn sensor_name_get__3_7_2_14_spec(result: Int32, flags: UInt32, name: UInt8[64], old_s: S, new_s: S) -> bool {
    (!SensorExists(sensor_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (SensorExists(sensor_id(old_s)) ==> (ResultEqual(result, SUCCESS) && flags == 0 && name == SensorExtendedName(sensor_id(old_s)) && IsNullTerminatedUtf8(name, 64)))
    && (old_s == new_s)
}