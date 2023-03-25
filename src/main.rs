use std::{process::Command, fs::File, time::Duration};

use anyhow::Result;
use daemonize::Daemonize;
use dialoguer::{Input, theme::ColorfulTheme};

fn main() -> Result<()> {

    let notif_title = get_text_input("Notification title")?;
    let notif_msg = get_text_input("Notification message")?;
    let timer = get_min_input("Timer in minutes")?;

    let stdout = File::create("/tmp/daemon.out")?;
    let stderr = File::create("/tmp/daemon.err")?;

    let daemonize = Daemonize::new()
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Exectuted before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            std::thread::sleep(Duration::from_secs(timer * 60));
            Command::new("/usr/bin/dunst").spawn().expect("could not spawn dunst");
            libnotify::init("notifyi").unwrap();

            let n = libnotify::Notification::new(&notif_title, Some(notif_msg.as_str()), None);
            n.show()?;
            libnotify::uninit();
        },
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

fn get_text_input(prompt: &str) -> Result<String> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()?;

    Ok(input)
}

fn get_min_input(prompt: &str) -> Result<u64> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()?;

    let input: u64 = input.trim().parse()?; 

    Ok(input)
}
