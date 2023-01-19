use super::types::MongoLog;
use serde_json::Value;

pub fn clean_string(s:String)->String{
    return s.replace('"', "");
}
pub fn verbose_level_from_abbrv(abbrv:String)-> String{
    if abbrv == "F" {return "Fatal".to_string()}
    else if abbrv =="E" {return "Error".to_string()}
    else if abbrv =="W" {return "Warning".to_string()}
    else if abbrv == "I" {return "Informational".to_string()}
    return abbrv.to_string()
}
pub fn make_object_from_untyped_object(v:&Value)->MongoLog{
    let verbosity = clean_string(v["s"].to_string());
    return MongoLog{
        attr:serde_json::Value::String(v["attr"].to_string()),
        c:clean_string(v["c"].to_string()),
        ctx:clean_string(v["ctx"].to_string()),
        id:clean_string(v["id"].to_string()),
        msg:clean_string(v["msg"].to_string()),
        s:verbose_level_from_abbrv(verbosity).to_string(),
        t:clean_string(v["t"]["$date"].to_string())
    }
}