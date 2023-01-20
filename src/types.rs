use serde_json::Value;
use serde::{Deserialize, Serialize};
#[derive(Serialize,Deserialize ,Clone,Debug)]
pub struct MongoLog{
    pub attr:Value,
    pub c:String,
    pub ctx:String, 
    pub id:String,
    pub msg:String,
    pub s:String, //Severity (F)atal, (E)rror, (W)arning, (I)nformational
    pub t:String
}
