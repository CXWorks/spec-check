pub open spec fn protocol_attributes__3_11_2_3_spec(result: Int32, num_pin_groups: UInt16, num_pins: UInt16, reserved: UInt16, num_functions: UInt16, old_s: S, new_s: S) -> bool {
    (result == 0)
    && (num_pin_groups == NumPinGroups())
    && (num_pins == NumPins())
    && (reserved == 0)
    && (num_functions == NumFunctions())
    && (old_s == new_s)
}