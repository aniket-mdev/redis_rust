use redis::Commands;

trait RedisTrait {
    fn connect(client_options:RedicClientOptions) -> Result<Redis, Box<dyn std::error::Error>>;
    fn set_key(&self, key_name:&str, value:&str, time_out:usize) -> Result<(), Box<dyn std::error::Error>>;
    fn get_key(&self, key_name:String) -> Result<String, Box<dyn std::error::Error>>;
    fn close_connection(&mut self) -> Result<&str, Box<dyn std::error::Error>>;
}

pub struct RedicClientOptions {
    pub connection_address:String,
    pub port:i64
}

pub struct Redis {
    pub client:Option<redis::Client>
}

impl RedisTrait for Redis {
    fn connect(client_options:RedicClientOptions) -> Result<Redis, Box<dyn std::error::Error>> {
        let redis_url:String = format!("redis://:@{}:{}",client_options.connection_address, client_options.port);
        match redis::Client::open(redis_url.as_str()) {
            Ok(client) => Ok(Redis{client:Some(client)}),
            Err(e) => Err(Box::new(e)),
        }
    }
    
    fn set_key(&self, key_name:&str, value:&str, time_out:usize) -> Result<(), Box<dyn std::error::Error>> {
        if self.client.is_none() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("redis client is not availiable "),
            )))
        }

        let _:() = self.client.as_ref().unwrap().set_ex(key_name, value, time_out)?;
        return Ok(());
    }
    
    fn get_key(&self, key_name:String) -> Result<String, Box<dyn std::error::Error>> {
       if self.client.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("redis client is not availiable "),
        )))
       }

       match self.client.as_ref().unwrap().get(key_name) {
            Ok(value) => Ok(value),
            Err(e) => Err(Box::new(e)),
        }
    }
    
    fn close_connection(&mut self) -> Result<&str, Box<dyn std::error::Error>> {
        if self.client.is_none() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("redis client is not availiable "),
            )))
        }

        let _ = self.client.as_ref().take();
        self.client = None;
        return Ok("connection has been closed")
        
    }
    
}

fn main() {
   let redis_config = RedicClientOptions { connection_address:"localhost".to_string(), port:6379} ;
   match Redis::connect(redis_config)   {
    Ok(mut redis) => {

        match redis.set_key("dbApp", "MongoDB", 1) {
            Ok(_) => println!("value has been set"),
            Err(e) => println!("{:?}",e),
        }
        
        match redis.get_key("dbApp".to_string()) {
            Ok(value) => println!("Data key : {:?}", value),
            Err(e) => println!("{:?}", e),
        }

        match redis.close_connection() {
            Ok(result) => println!("{:?}", result),
            Err(e) => println!("{:?}",e.to_string()),
        }

        match redis.get_key("dbApp".to_string()) {
            Ok(value) => println!("Data key : {:?}", value),
            Err(e) => println!("{:?}", e),
        }
    },
    Err(err) => {
        println!("Error : {:?}", err.to_string())
    },
   }

}
