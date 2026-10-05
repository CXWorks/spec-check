pub open spec fn base_discover_list_protocols__3_2_2_8_spec(skip: UInt32, status: i32, num_protocols: UInt32, protocols: Seq<UInt32>, old_s: S, new_s: S) -> bool {
    (!IsSkipValid(old_s, skip) ==> status == INVALID_PARAMETERS)
    && (IsSkipValid(old_s, skip) ==> (
        status == SUCCESS
        && (num_protocols as int) <= AllowedProtocolCount(old_s) - (skip as int)
        && protocols.len() == 1 + ((num_protocols as int) - 1) / 4
        && PackedProtocolListEqual(old_s, skip, num_protocols, protocols)
        && ProtocolListIsAscending(num_protocols, protocols)
        && ProtocolListExcludesBase(num_protocols, protocols)
    ))
    && new_s == old_s
}
