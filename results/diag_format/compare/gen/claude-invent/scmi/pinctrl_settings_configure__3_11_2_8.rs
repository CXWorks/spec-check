pub open spec fn pinctrl_settings_configure__3_11_2_8_spec(identifier: UInt32, function_id: UInt32, attributes: UInt32, configs: Seq<(UInt32, UInt32)>, status: ScmiStatus, old_s: S, new_s: S) -> bool {
    let selector: UInt32 = attributes & 0x3u32;
    let num_configs: UInt32 = (attributes >> 2u32) & 0xFFu32;
    let function_id_valid: bool = ((attributes >> 10u32) & 0x1u32) == 1u32;
    let attributes_reserved_ok: bool = (attributes >> 11u32) == 0u32;
    let selector_ok: bool = selector == 0u32 || selector == 1u32;
    let count_ok: bool = (configs.len() as int) == (num_configs as int) && !(num_configs == 0u32 && !function_id_valid);
    let config_types_ok: bool = forall |i: int| 0 <= i < configs.len() ==> ((configs[i].0 >> 8u32) == 0u32 && IsValidPinctrlConfigType(configs[i].0 & 0xFFu32));
    let sorted_ok: bool = forall |i: int, j: int| 0 <= i < j < configs.len() ==> (configs[i].0 & 0xFFu32) < (configs[j].0 & 0xFFu32);
    let params_ok: bool = attributes_reserved_ok && selector_ok && count_ok && config_types_ok && sorted_ok;
    let found_ok: bool = IsValidPinctrlIdentifier(old_s, selector, identifier);
    let supported_ok: bool = PinctrlConfigsSupported(old_s, selector, identifier, configs) && (!function_id_valid || PinctrlFunctionSupported(old_s, selector, identifier, function_id));
    let permitted_ok: bool = PinctrlAgentPermitted(old_s, selector, identifier);
    let not_in_use_ok: bool = !PinctrlInUseByOtherAgent(old_s, selector, identifier);
    let transport_ok: bool = !PinctrlConfigsExceedTransport(old_s, num_configs);
    (!found_ok ==> status != SUCCESS)
    && (!params_ok ==> status != SUCCESS)
    && (!supported_ok ==> status != SUCCESS)
    && (!permitted_ok ==> status != SUCCESS)
    && (!not_in_use_ok ==> status != SUCCESS)
    && (!transport_ok ==> status != SUCCESS)
    && (status == NOT_FOUND ==> !found_ok)
    && (status == INVALID_PARAMETERS ==> !params_ok)
    && (status == NOT_SUPPORTED ==> !supported_ok)
    && (status == DENIED ==> !permitted_ok)
    && (status == IN_USE ==> !not_in_use_ok)
    && (status == PROTOCOL_ERROR ==> !transport_ok)
    && (status != SUCCESS ==> new_s == old_s)
    && ((found_ok && params_ok && supported_ok && permitted_ok && not_in_use_ok && transport_ok) ==> status == SUCCESS)
    && (status == SUCCESS ==> (
        found_ok && params_ok && supported_ok && permitted_ok && not_in_use_ok && transport_ok
        && (forall |i: int| 0 <= i < configs.len() ==>
                PinctrlConfigValue(new_s, selector, identifier, configs[i].0 & 0xFFu32) == configs[i].1)
        && (forall |t: UInt32| (forall |i: int| 0 <= i < configs.len() ==> (configs[i].0 & 0xFFu32) != t) ==>
                PinctrlConfigValue(new_s, selector, identifier, t) == PinctrlConfigValue(old_s, selector, identifier, t))
        && (function_id_valid ==> PinctrlFunctionSelected(new_s, selector, identifier) == function_id)
        && (!function_id_valid ==> PinctrlFunctionSelected(new_s, selector, identifier) == PinctrlFunctionSelected(old_s, selector, identifier))
    ))
}
