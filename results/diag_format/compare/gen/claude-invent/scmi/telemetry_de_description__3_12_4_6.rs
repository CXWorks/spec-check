pub open spec fn telemetry_de_description__3_12_4_6_spec(desc_index: u32, status: i32, num_desc: u32, desc: Seq<DeDesc>, old_s: S, new_s: S) -> bool {
    (new_s == old_s)
    && (status == 0 ==> (
        ((num_desc & 0xFFFFu32) as int == desc.len())
        && ((desc_index as int) + desc.len() <= DeDescriptorArray(old_s).len())
        && (forall|i: int| 0 <= i < desc.len() ==> desc[i] == DeDescriptorArray(old_s)[(desc_index as int) + i])
        && (((num_desc >> 16u32) & 0xFFFFu32) as int == DeDescriptorArray(old_s).len() - (desc_index as int) - desc.len())
        && (forall|i: int| 0 <= i < desc.len() ==> ((desc[i].de_attributes_1 >> 2u32) & 0x7u32) == 0)
        && (forall|i: int| 0 <= i < desc.len() ==> (desc[i].de_attributes_1 & 0x3u32) != 3u32)
        && (forall|i: int| 0 <= i < desc.len() ==> desc[i].de_attributes_3 == 0)
        && (forall|i: int, j: int| 0 <= i < desc.len() && 0 <= j < desc.len() && i != j ==> desc[i].de_id != desc[j].de_id)
    ))
}
