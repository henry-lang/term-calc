use std::{collections::HashMap, f64};

pub type Function = fn(f64) -> f64;
pub type Constant = f64;

pub struct Identifiers {
    funcs: HashMap<String, Function>,
    constants: HashMap<String, Constant> 
}

impl Identifiers {
    pub fn get() -> Self {
        let mut it = Identifiers{funcs: HashMap::new(), constants: HashMap::new()};

        it.funcs.insert("sin".into(), f64::sin);
        it.funcs.insert("cos".into(), f64::cos);
        it.funcs.insert("tan".into(), f64::tan);
        it.funcs.insert("arcsin".into(), f64::asin);
        it.funcs.insert("arccos".into(), f64::acos);
        it.funcs.insert("arctan".into(), f64::atan);
        it.funcs.insert("sqrt".into(), f64::sqrt);

        it.constants.insert("pi".into(), f64::consts::PI);
        it.constants.insert("e".into(), f64::consts::E);
        it.constants.insert("tau".into(), f64::consts::TAU);

        it
    }

    pub fn get_func(&self, name: &String) -> Option<&Function> {
        self.funcs.get(name)
    }

    pub fn get_constant(&self, name: &String) -> Option<&Constant> {
        self.constants.get(name)
    }
}
