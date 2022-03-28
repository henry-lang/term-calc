use std::{collections::HashMap, f64};

pub type Function = fn(f64) -> f64;
pub type Constant = f64;

pub struct Identifiers {
    funcs: HashMap<&'static str, Function>,
    constants: HashMap<&'static str, Constant>,
}

impl Identifiers {
    pub fn get() -> Self {
        let mut it = Identifiers {
            funcs: HashMap::new(),
            constants: HashMap::new(),
        };

        it.funcs.insert("sin", f64::sin);
        it.funcs.insert("cos", f64::cos);
        it.funcs.insert("tan", f64::tan);
        it.funcs.insert("arcsin", f64::asin);
        it.funcs.insert("arccos", f64::acos);
        it.funcs.insert("arctan", f64::atan);
        it.funcs.insert("sqrt", f64::sqrt);
        it.funcs.insert("round", f64::round);

        it.constants.insert("pi", f64::consts::PI);
        it.constants.insert("e", f64::consts::E);
        it.constants.insert("tau", f64::consts::TAU);

        it
    }

    pub fn get_func(&self, name: &str) -> Option<&Function> {
        self.funcs.get(name)
    }

    pub fn get_constant(&self, name: &str) -> Option<&Constant> {
        self.constants.get(name)
    }
}
