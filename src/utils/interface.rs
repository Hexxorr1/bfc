use crate::utils::BfcError;

#[derive(Debug, PartialEq)]
pub enum OutputType{
    Sbf,
    OperationList,
}

impl OutputType{
    pub fn from_string(s: String) -> Result<Self, BfcError>{
        use OutputType::*;
        match s.as_str(){
            "sbf" => Ok(Sbf),
            "ol" => Ok(OperationList),
            _ => Err(BfcError::InvalidFlagArgs(vec![s])),
        }
    }
}

pub fn log_if(log: bool, text: &str){
    if log{
        println!("{}", text);
    }
}
