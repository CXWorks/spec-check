pub open spec fn base_discover_sub_vendor__3_2_2_6_spec(status: i32, vendor_identifier: Seq<u8>) -> bool {
    (status == 0 ==> (
        vendor_identifier.len() == 16
        && (exists|i: int| 0 <= i < 16 && vendor_identifier[i] == 0u8
            && (forall|j: int| 0 <= j < i ==> vendor_identifier[j] < 128u8))
    ))
}
