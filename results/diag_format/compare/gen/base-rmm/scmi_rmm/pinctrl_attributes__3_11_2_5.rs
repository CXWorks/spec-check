pub open spec fn pinctrl_attributes__3_11_2_5_spec(result: Int32, attributes: UInt32, name: UInt8[16], old_s: S, new_s: S) -> bool {
    (!PinctrlObjectExists(flags_from_selector_and_identifier(old_s, result, attributes), identifier_from_selector_and_identifier(old_s, result, attributes)) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (
        (PinctrlNameLength(flags_from_selector_and_identifier(old_s, result, attributes), identifier_from_selector_and_identifier(old_s, result, attributes)) > 16 ==> attributes[31] == 1)
        && (PinctrlNameLength(flags_from_selector_and_identifier(old_s, result, attributes), identifier_from_selector_and_identifier(old_s, result, attributes)) <= 16 ==> attributes[31] == 0)
        && (flags_from_selector_and_identifier(old_s, result, attributes) == 2 ==> attributes[17] == FunctionSupportsGpio(identifier_from_selector_and_identifier(old_s, result, attributes)))
        && (flags_from_selector_and_identifier(old_s, result, attributes) == 2 ==> (attributes[17] == 1 ==> count_functions_with_gpio_support(old_s) <= 1))
        && (flags_from_selector_and_identifier(old_s, result, attributes) == 2 ==> (IsPinOnlyFunction(identifier_from_selector_and_identifier(old_s, result, attributes)) ==> attributes[16] == 1))
        && (flags_from_selector_and_identifier(old_s, result, attributes) == 2 ==> (!IsPinOnlyFunction(identifier_from_selector_and_identifier(old_s, result, attributes)) ==> attributes[16] == 0))
        && (flags_from_selector_and_identifier(old_s, result, attributes) == 0 ==> attributes[15:0] == 1)
        && (flags_from_selector_and_identifier(old_s, result, attributes) == 1 ==> attributes[15:0] == GroupPinCount(identifier_from_selector_and_identifier(old_s, result, attributes)))
        && (flags_from_selector_and_identifier(old_s, result, attributes) == 2 && attributes[16] == 1 ==> attributes[15:0] == FunctionPinCount(identifier_from_selector_and_identifier(old_s, result, attributes)))
        && (flags_from_selector_and_identifier(old_s, result, attributes) == 2 && attributes[16] == 0 ==> attributes[15:0] == FunctionGroupCount(identifier_from_selector_and_identifier(old_s, result, attributes)))
        && (attributes[31] == 0 ==> name == PinctrlName(flags_from_selector_and_identifier(old_s, result, attributes), identifier_from_selector_and_identifier(old_s, result, attributes)))
        && (attributes[31] == 1 ==> name == lower_15_bytes_of(PinctrlName(flags_from_selector_and_identifier(old_s, result, attributes), identifier_from_selector_and_identifier(old_s, result, attributes))))
    ))
}

fn flags_from_selector_and_identifier(s: S, result: Int32, attributes: UInt32) -> UInt8 {
    (result == SUCCESS) ? (attributes[1:0] as UInt8) : 0
}

fn identifier_from_selector_and_identifier(s: S, result: Int32, attributes: UInt32) -> UInt32 {
    (result == SUCCESS) ? (attributes[15:0] as UInt32) : 0
}

fn count_functions_with_gpio_support(s: S) -> int {
    // Placeholder for counting functions with GPIO support; implementation depends on specific platform state
    0
}

fn lower_15_bytes_of(name: UInt8[16]) -> UInt8[15] {
    // Helper to extract lower 15 bytes
    name[0:15]
}