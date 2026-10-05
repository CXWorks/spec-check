pub open spec fn telemetry_de_description_spec(result: Int32, num_remaining: UInt16, num_returned: UInt16, desc: [DE_DESC; 0], desc_index: UInt32, old_s: S, new_s: S) -> bool {
    (result != 0 ==> false)
    && (num_returned == 0 ==> true)
    && (num_returned > 0 ==> (num_remaining == NumRemainingDeDescriptors(desc_index, num_returned) && desc[0] == DeDescriptorAt(desc_index) && (forall i < num_returned: IsDeDescFormat(desc[i]))))
}