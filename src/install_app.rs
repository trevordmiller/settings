use std::process::Command;

pub fn install_app(app: &str) {
    let installation_status = match Command::new("brew")
        .arg("cask")
        .arg("list")
        .arg(app)
        .output()
    {
        Ok(output) => output.status,
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    if installation_status.success() {
        println!("{} is already installed.", app)
    } else {
        println!("Installing {}.", app);
        match Command::new("brew")
            .arg("cask")
            .arg("install")
            .arg(app)
            .output()
        {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}
