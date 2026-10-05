pub open spec fn sensor_name_get__3_7_2_14_spec(sensor_id: UInt32, status: i32, flags: UInt32, name: Seq<u8>, old_s: S, new_s: S) -> bool {
    (!SensorExists(old_s, sensor_id) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (
        SensorExists(old_s, sensor_id)
        && SensorExtendedNameSupported(old_s, sensor_id)
        && flags == 0
        && name.len() == 64
        && (exists|i: int| 0 <= i < 64 && name[i] == 0u8)
        && name == SensorExtendedName(old_s, sensor_id)
    ))
    && new_s == old_s
}
