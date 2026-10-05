pub open spec fn protocol_attributes__3_11_2_3_spec(num_pin_groups: UInt16, num_pins: UInt16, reserved: UInt16, num_functions: UInt16, old_s: S, new_s: S) -> bool {
  (num_pin_groups == NumPinGroups())
  && (num_pins == NumPins())
  && (num_functions == NumFunctions())
  && (reserved == 0)
}