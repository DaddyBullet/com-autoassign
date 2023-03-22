mod update_script;
use windows::{core::*, Devices::Enumeration::*, Foundation::*};
use update_script::update_script;

fn disp_device_information_update(info: &DeviceInformationUpdate) {
  println!("ID: {}", info.Id().unwrap());
  println!("KIND: {}", info.Kind().unwrap().0);

  println!("Number of properies: {}", info.Properties().unwrap().Size().unwrap());
  println!("Property1: {}", info.Properties().unwrap().First().unwrap().Current().unwrap().Key().unwrap());

  let key = info.Properties().unwrap().First().unwrap().Current().unwrap().Key().unwrap();
  if key.to_string().contains("System.ItemNameDisplay") {
    println!("Display name type: {}", info.Properties().unwrap().First().unwrap().Current().unwrap().Value().unwrap().GetRuntimeClassName().unwrap());
  }
  else if key.to_string().contains("System.Devices.InterfaceEnabled") {
    println!("IEnabled type: {}", info.Properties().unwrap().First().unwrap().Current().unwrap().Value().unwrap().GetRuntimeClassName().unwrap());
    println!("IEnabled: {}", info.Properties().unwrap().First().unwrap().Current().unwrap().Value().unwrap().cast::<IReference<bool>>().unwrap().GetBoolean().unwrap());
  }
}

const TARGER_DEV_NAME: &str = "Silicon Labs CP210x USB to UART Bridge";
fn main() -> Result<()> {
  let watcher = DeviceInformation::CreateWatcherAqsFilter(&HSTRING::from(
    "System.ItemNameDisplay:~~\"Silicon Labs\""))?;

    watcher.Updated(&TypedEventHandler::new(|_, info: &Option<DeviceInformationUpdate>| {
      disp_device_information_update(info.as_ref().unwrap());
      update_script(TARGER_DEV_NAME).unwrap();
      Ok(())
    }))?;

    watcher.Start()?;

    std::thread::sleep(std::time::Duration::MAX);

    watcher.Stop()?;

  Ok(())
}