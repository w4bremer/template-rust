// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use testbed1::api::data_structs::*;
use testbed1::api::struct_array_interface::StructArrayInterfaceTrait;
use testbed1::implementation::struct_array_interface::StructArrayInterface;

/// tests for StructArrayInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func_bool() {
        let mut test_object: StructArrayInterface = Default::default();
        test_object.func_bool(Default::default());
    }

    #[test]
    fn test_func_bool_async() {
        let mut test_object: StructArrayInterface = Default::default();
        let _ = test_object.func_bool_async(Default::default());
    }

    #[test]
    fn test_func_int() {
        let mut test_object: StructArrayInterface = Default::default();
        test_object.func_int(Default::default());
    }

    #[test]
    fn test_func_int_async() {
        let mut test_object: StructArrayInterface = Default::default();
        let _ = test_object.func_int_async(Default::default());
    }

    #[test]
    fn test_func_float() {
        let mut test_object: StructArrayInterface = Default::default();
        test_object.func_float(Default::default());
    }

    #[test]
    fn test_func_float_async() {
        let mut test_object: StructArrayInterface = Default::default();
        let _ = test_object.func_float_async(Default::default());
    }

    #[test]
    fn test_func_string() {
        let mut test_object: StructArrayInterface = Default::default();
        test_object.func_string(Default::default());
    }

    #[test]
    fn test_func_string_async() {
        let mut test_object: StructArrayInterface = Default::default();
        let _ = test_object.func_string_async(Default::default());
    }

    #[test]
    fn test_prop_bool() {
        let mut test_object: StructArrayInterface = Default::default();
        let default_value: Vec<StructBool> = Default::default();
        test_object.set_prop_bool(&default_value);
        assert_eq!(test_object.prop_bool().clone(), default_value);
    }

    #[test]
    fn test_prop_int() {
        let mut test_object: StructArrayInterface = Default::default();
        let default_value: Vec<StructInt> = Default::default();
        test_object.set_prop_int(&default_value);
        assert_eq!(test_object.prop_int().clone(), default_value);
    }

    #[test]
    fn test_prop_float() {
        let mut test_object: StructArrayInterface = Default::default();
        let default_value: Vec<StructFloat> = Default::default();
        test_object.set_prop_float(&default_value);
        assert_eq!(test_object.prop_float().clone(), default_value);
    }

    #[test]
    fn test_prop_string() {
        let mut test_object: StructArrayInterface = Default::default();
        let default_value: Vec<StructString> = Default::default();
        test_object.set_prop_string(&default_value);
        assert_eq!(test_object.prop_string().clone(), default_value);
    }
}
