pub open spec fn protocol_attributes_spec(result: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (IsSuccessStatus(result) ==> attributes == 0)
    && (attributes != 0 ==> !IsSuccessStatus(result))
}