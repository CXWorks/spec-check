pub open spec fn sensor_axis_description_get__3_7_2_6_spec(status: Int32, num_axis_flags: UInt32, desc: [SensorAxisDescriptor], old_s: S, new_s: S) -> bool {
    (!IsValidSensor(old_s, sensor_id(old_s)) ==> ResultEqual(status, NOT_FOUND))
    && (!SensorReportsAxisValues(old_s, sensor_id(old_s)) ==> ResultEqual(status, NOT_SUPPORTED))
    && (ResultEqual(status, SUCCESS) ==> (
        Bits(num_axis_flags, 5, 0) == DescCount(desc)
        && Bits(num_axis_flags, 25, 6) == 0
        && Bits(num_axis_flags, 31, 26) == (NumSensorAxes(old_s, sensor_id(old_s)) - axis_desc_index(old_s) - Bits(num_axis_flags, 5, 0))
        && (forall i: UInt32 | i < Bits(num_axis_flags, 5, 0) ==> DescAt(desc, i) == SensorAxisDescriptor(old_s, sensor_id(old_s), axis_desc_index(old_s) + i))
        && (forall i: UInt32 | i < Bits(num_axis_flags, 5, 0) ==> (Bits(DescAt(desc, i).axis_attributes_low, 9, 9) == 1 ==> AxisNameLongerThan16Bytes(old_s, sensor_id(old_s), axis_desc_index(old_s) + i))
        && (forall i: UInt32 | i < Bits(num_axis_flags, 5, 0) ==> IsNullTerminatedUtf8(DescAt(desc, i).name, 16))
        && (forall i: UInt32 | i < Bits(num_axis_flags, 5, 0) ==> (Bits(DescAt(desc, i).axis_attributes_low, 8, 8) == 0 ==> !ExtendedAttributeFieldsAllocated(DescAt(desc, i))))
        && (forall i: UInt32 | i < Bits(num_axis_flags, 5, 0) ==> ((Bits(DescAt(desc, i).axis_attributes_low, 8, 8) == 1 && !SensorReportsAxisResolution(old_s, sensor_id(old_s), axis_desc_index(old_s) + i) ==> DescAt(desc, i).axis_resolution == 0x0))
        && (forall i: UInt32 | i < Bits(num_axis_flags, 5, 0) ==> ((Bits(DescAt(desc, i).axis_attributes_low, 8, 8) == 1 && !SensorReportsAxisMinRange(old_s, sensor_id(old_s), axis_desc_index(old_s) + i) ==> DescAt(desc, i).axis_min_range_low == 0x0))
        && (forall i: UInt32 | i < Bits(num_axis_flags, 5, 0) ==> ((Bits(DescAt(desc, i).axis_attributes_low, 8, 8) == 1 && !SensorReportsAxisMinRange(old_s, sensor_id(old_s), axis_desc_index(old_s) + i) ==> DescAt(desc, i).axis_min_range_high == 0x80000000))
        && (forall i: UInt32 | i < Bits(num_axis_flags, 5, 0) ==> ((Bits(DescAt(desc, i).axis_attributes_low, 8, 8) == 1 && !SensorReportsAxisMaxRange(old_s, sensor_id(old_s), axis_desc_index(old_s) + i) ==> DescAt(desc, i).axis_max_range_low == 0xFFFFFFFF))
        && (forall i: UInt32 | i < Bits(num_axis_flags, 5, 0) ==> ((Bits(DescAt(desc, i).axis_attributes_low, 8, 8) == 1 && !SensorReportsAxisMaxRange(old_s, sensor_id(old_s), axis_desc_index(old_s) + i) ==> DescAt(desc, i).axis_max_range_high == 0x7FFFFFFF)))
    ))
    && (old_s == new_s)
}