#![deny(clippy::all)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub struct Oauth {
  client_id: String,
}

#[napi]
pub struct DeviceCode {
    client_id: String,
}

#[napi]
impl Oauth {
  #[napi(constructor)]
  pub fn new(client_id: String) -> Self {
      Self { client_id }
  }

  #[napi]
  pub fn url(&self) -> String {
    self.client_id.clone()
  }

  #[napi]
  pub fn launch() -> String {
    "launch called".to_string()
  }
}



#[napi]
impl DeviceCode {
  #[napi(constructor)]
  pub fn new(client_id: String) -> Self {
      Self { client_id }
  }

  #[napi]
  pub fn prelaunch(&self) -> String {
    self.client_id.clone()
  }

  #[napi]
  pub fn launch() -> String {
    "Device launch called".to_string()
  }
}

