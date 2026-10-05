pub open spec fn sensor_description_get__3_7_2_5_spec(status: Int32, num_sensor_flags: UInt32, desc: [SENSOR_DESC], desc_index: UInt32, old_s: S, new_s: S) -> bool {
    (ResultEqual(status, SUCCESS) ==> (
        Bits(num_sensor_flags, 15, 12) == 0
        && Bits(num_sensor_flags, 11, 0) == NumRemainingSensorDescriptors(desc_index, Bits(num_sensor_flags, 11, 0))
        && (forall|i: UInt32| i < Bits(num_sensor_flags, 11, 0) ==> (
            desc[i] == SensorDescriptorAt(desc_index + i)
            && SensorDescriptorIsSameOnRepeatedCalls(desc[i].sensor_id)
            && IsNullTerminatedUtf8(desc[i].sensor_name, 16)
            && (Bits(desc[i].sensor_attributes_high, 8, 8) == 1 ==> Bits(desc[i].sensor_attributes_high, 21, 16) != 0)
            && (Bits(desc[i].sensor_attributes_low, 8, 8) == 0 ==> !ExtendedAttributesAllocated(desc[i]))
            && ((Bits(desc[i].sensor_attributes_low, 8, 8) == 1 && !SensorReportsPower(desc[i].sensor_id) ==> desc[i].sensor_power == 0)
            && ((Bits(desc[i].sensor_attributes_low, 8, 8) == 1 && Bits(desc[i].sensor_attributes_high, 8, 8) == 0 && !SensorReportsResolution(desc[i].sensor_id) ==> desc[i].sensor_resolution == 0x0)
            && ((Bits(desc[i].sensor_attributes_low, 8, 8) == 1 && Bits(desc[i].sensor_attributes_high, 8, 8) == 0 && !SensorReportsMinRange(desc[i].sensor_id) ==> desc[i].sensor_min_range_low == 0x0)
            && ((Bits(desc[i].sensor_attributes_low, 8, 8) == 1 && Bits(desc[i].sensor_attributes_high, 8, 8) == 0 && !SensorReportsMinRange(desc[i].sensor_id) ==> desc[i].sensor_min_range_high == 0x80000000)
            && ((Bits(desc[i].sensor_attributes_low, 8, 8) == 1 && Bits(desc[i].sensor_attributes_high, 8, 8) == 0 && !SensorReportsMaxRange(desc[i].sensor_id) ==> desc[i].sensor_max_range_low == 0xFFFFFFFF)
            && ((Bits(desc[i].sensor_attributes_low, 8, 8) == 1 && Bits(desc[i].sensor_attributes_high, 8, 8) == 0 && !SensorReportsMaxRange(desc[i].sensor_id) ==> desc[i].sensor_max_range_high == 0x7FFFFFFF))))))))))
    )
    && (ResultEqual(status, SUCCESS) ==> (
        new_s == old_s
    ))
}