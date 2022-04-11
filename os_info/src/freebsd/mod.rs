use std::process::Command;
use std::str;

use log::{error, trace};

use crate::{bitness, uname::uname, Info, Type, Version};

pub fn current_platform() -> Info {
    trace!("freebsd::current_platform is called");

    let version = uname()
        .map(Version::from_string)
        .unwrap_or_else(|| Version::Unknown);

    let info = Info {
        os_type: get_os(version.to_string()),
        version,
        bitness: bitness::get(),
        ..Default::default()
    };

    trace!("Returning {:?}", info);
    info
}

fn get_os(ver: String) -> Type {
    let os = Command::new("uname")
        .arg("-s")
        .output()
        .expect("Failed to get OS");

    match str::from_utf8(&os.stdout).trim() {
        Ok("FreeBSD") => {
            let checkHardening = Command::new("sysctl")
                .arg("-a hardening")
                .output()
                .expect("Failed to check if is hardened");
            let isHardened = Command::new("echo")
                .arg("$?")
                .output()
                .expect("Could not get a return value");
            match str::from_utf8(&isHardened.stdout).trim() {
                Ok("0") => return Type::HardenedBSD,
                Err(_) => return Type::FreeBSD,
            }
        },
        Ok("MidnightBSD") => Type::MidnightBSD,
        Err(_) => Type::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::FreeBSD, version.os_type());
    }
}
