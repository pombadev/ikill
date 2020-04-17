use clap::{crate_description, crate_name, crate_version, App, AppSettings};

pub fn new() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::ColorAuto)
}
