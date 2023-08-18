// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::nested_struct3_interface::NestedStruct3InterfaceTrait;
use testbed2::implementation::nested_struct3_interface::NestedStruct3Interface;

/// tests for NestedStruct3Interface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_func1() {
        let mut test_object: NestedStruct3Interface = Default::default();
        test_object.func1(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func1_async() {
        let mut test_object: NestedStruct3Interface = Default::default();
        test_object.func1_async(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func2() {
        let mut test_object: NestedStruct3Interface = Default::default();
        test_object.func2(
        &Default::default(),
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func2_async() {
        let mut test_object: NestedStruct3Interface = Default::default();
        test_object.func2_async(
        &Default::default(),
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func3() {
        let mut test_object: NestedStruct3Interface = Default::default();
        test_object.func3(
        &Default::default(),
        &Default::default(),
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func3_async() {
        let mut test_object: NestedStruct3Interface = Default::default();
        test_object.func3_async(
        &Default::default(),
        &Default::default(),
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop1() {
        let mut test_object: NestedStruct3Interface = Default::default();
        let default_value: NestedStruct1 = Default::default();
        test_object.set_prop1(&default_value);
        assert_eq!(test_object.prop1().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop2() {
        let mut test_object: NestedStruct3Interface = Default::default();
        let default_value: NestedStruct2 = Default::default();
        test_object.set_prop2(&default_value);
        assert_eq!(test_object.prop2().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop3() {
        let mut test_object: NestedStruct3Interface = Default::default();
        let default_value: NestedStruct3 = Default::default();
        test_object.set_prop3(&default_value);
        assert_eq!(test_object.prop3().clone(), default_value);
    }
}
