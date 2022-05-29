use pickpick::Servertime;

pub async fn calculate(mut servertime:Servertime, host:String) -> Result<((i64,i64),String), String> {
    let res = servertime.calculate();
    match res{
        Ok(res) => Ok((res,host)),
        Err(st) => Err(st),
    }
}