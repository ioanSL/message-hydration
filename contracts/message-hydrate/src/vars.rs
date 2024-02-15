use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Variable {
    pub name: String,
    pub value: String
} 

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVariableError;

impl FromStr for Variable {
    type Err = ParseVariableError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (var, val) = s
            .split_once(":")
            .ok_or(ParseVariableError)?;

        Ok(Variable {
            name: var.to_string(),
            value: val.to_string(),
        })
    }
}

pub fn replace_variables(s: &mut String, vars: &Vec<Variable>){
    for element in vars {
        *s = s.replace(&element.name, &element.value);
    }
}

pub fn get_variables_from_string(vars: String) -> Vec<Variable> {
    let binding = vars.replace(&['\"', '[', ']', ' '][..], "");
    let raw_var_list = binding.split(',').collect::<Vec<&str>>();
    let mut var_list: Vec<Variable> = vec![];
    
    for e in raw_var_list {
        if e.len() > 0 {
            match Variable::from_str(e) {
                Ok(variable) => {
                    var_list.push(variable);
                }
                _ => todo!() // Handle error
            }
        }
    }

    var_list
}