use std::collections::HashMap;

pub type Function = fn(f64) -> f64;
pub type FunctionRegistry = HashMap<String, Function>;

pub fn get_registry() -> FunctionRegistry {
    let mut registry = FunctionRegistry::new();

    registry.insert("sin".into(), f64::sin);
    registry.insert("cos".into(), f64::cos);
    registry.insert("tan".into(), f64::tan);
    registry.insert("arcsin".into(), f64::asin);
    registry.insert("arccos".into(), f64::acos);
    registry.insert("arctan".into(), f64::atan);
    registry.insert("sqrt".into(), f64::sqrt);
    registry.insert("ln".into(), f64::ln);

    registry
}
