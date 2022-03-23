use std::collections::HashMap;

pub type Function = fn(f64) -> f64;
pub type FunctionRegistry = HashMap<String, Function>;

pub fn get_registry() -> FunctionRegistry {
    let mut registry = FunctionRegistry::new();
    
    registry.insert("sin".into(), f64::sin);
    registry.insert("cos".into(), f64::cos);
    registry.insert("tan".into(), f64::tan);

    registry
}
