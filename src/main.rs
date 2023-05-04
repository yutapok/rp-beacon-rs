use bluer::adv::Advertisement;

use std::{error, fmt};
use std::time::Duration;
use uuid::Uuid;
//use std::collections::BTreeMap;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time::sleep,
};

//const MANUFACTURE_ID: u16 = 0xffff;

#[derive(Debug, Clone)]
struct InvalidError {
  message: String,
}

fn invalid_error(err: &dyn error::Error) -> InvalidError {
  InvalidError{message: err.to_string()}
}

impl fmt::Display for InvalidError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "InvalidError, {}", self.message)
  }
}

impl error::Error for InvalidError {
 fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
 }
}

#[derive(Debug)]
struct Beacon {
  display_name: Option<String>,
  raw_uuid: Option<Uuid>,
  //manufacturer_data_value: Option<Vec<u8>>
}

impl Beacon {
  pub fn new() -> Self {
    Beacon {
      display_name: None,
      raw_uuid: None,
      //manufacturer_data_value: None,
    }
  }
  pub fn display(&mut self, local_name: &str) -> Result<(), InvalidError> {
    self.display_name = Some(local_name.to_string());
    Ok(())
  }

  pub fn uuid(&mut self, uuid_str: &str) -> Result<(), InvalidError> {
    match uuid_str.parse::<Uuid>() {
      Ok(ud) => {
          self.raw_uuid = Some(ud);
          Ok(())
      },
      Err(err) => {
          return Err(invalid_error(&err)) 
      }
    }
  }


  pub async fn broadcast(&self) -> bluer::Result<()>{
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;


    println!("Advertising on Bluetooth adapter {} with address {}", adapter.name(), adapter.address().await?);
    //let mut manufacturer_data = BTreeMap::new();
    
    // Error Occur When Over 6 Bytes.
    //let manufacturer_value = String::from("123456").into_bytes();
    //manufacturer_data.insert(MANUFACTURE_ID, manufacturer_value);
    //
    
    let service_uuid = match self.raw_uuid {
        Some(ud) => ud,
        None => Uuid::new_v4(),
    };

    let le_advertisement = Advertisement {
      advertisement_type: bluer::adv::Type::Peripheral,
      service_uuids: vec![service_uuid].into_iter().collect(),
      discoverable: Some(true),
      local_name: Some(
          self.display_name
          .as_ref()
          .unwrap_or(&"le_default".to_string())
          .clone()
          ),
     //manufacturer_data,
      ..Default::default()
    };

    println!("{:?}", &le_advertisement);
    let handle = adapter.advertise(le_advertisement).await?;

    println!("Press enter to quit");

    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;

    println!("Removing advertisement");
    drop(handle);
    sleep(Duration::from_secs(1)).await;

    Ok(())
    
  }
}

#[tokio::main(flavor="current_thread")]
async fn main() -> bluer::Result<()> {

    let mut beacon = Beacon::new();
    beacon.display("le_advertise_test").unwrap();
    //beacon.uuid("").unwrap();
    beacon.broadcast().await?;

    Ok(())
}
