use serde_json::Value;

#[derive(Clone,Debug)]
pub struct MongoLog{
    pub attr:Value,
    pub c:String,
    pub ctx:String,
    pub id:String,
    pub msg:String,
    pub s:String, //Severity (F)atal, (E)rror, (W)arning, (I)nformational
    pub t:String
}
#[derive(Debug, Clone, Copy)]
pub enum EventComponent{
    ACCESS,
    COMMAND,
    CONTROL,
    ELECTION,
    FTDC,
    GEO,
    INDEX,
    INITSYNC,
    JOURNAL,
    NETWORK,
    QUERY,
    RECOVERY,
    REPL,
    REPL_HB,
    ROLLBACK,
    SHARDING,
    STORAGE,
    TXN,
    WRITE,
    WT,
    WTBACKUP,
    WTCHKPT,
    WTCMPCT,
    WTEVICT,
    WTHS,
    WTRECOV,
    WTRTS,
    WTSLVG,
    WTTIER,
    WTTS,
    WTTXN,
    WTVRFY,
    WTWRTLOG
}