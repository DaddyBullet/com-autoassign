use anyhow::Result;
use std::process::Command;

pub fn update_script(dev_ident: &str) -> Result<()> {
    let dev_list_raw = String::from_utf8(Command::new("usbipd")
        .arg("wsl")
        .arg("list")
        .output()?
        .stdout)?;

    dev_list_raw.split('\n').filter_map(|row| {
        if row.contains(dev_ident) {
            let bus_id = row.split_ascii_whitespace().next().unwrap();
            println!("{bus_id}");
            Some(bus_id)
        }
        else {
            None
        }
    }).for_each(|usb_id| {
        Command::new("usbipd")
        .arg("wsl")
        .arg("attach")
        .arg("--busid")
        .arg(usb_id)
        .arg("--distribution")
        .arg("ubuntu")
        .output().unwrap();
    });

    Ok(())
}