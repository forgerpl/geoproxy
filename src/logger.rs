use chrono::Local;
use env_logger::fmt::{Color, Style, StyledValue};
use log::Level;
use std::io::Write;

fn colored_level<'a>(style: &'a mut Style, level: Level) -> StyledValue<'a, &'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}

pub(crate) fn init_logger() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "geoproxy=info");
    env_logger::Builder::from_env(env)
        .format(|f, record| {
            let mut style = f.style();
            let level = colored_level(&mut style, record.level());

            writeln!(
                f,
                "{} {} ⇁ {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                record.args(),
            )
        })
        .init();
}
