use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, AppSettings, Arg,
};
use evtx::EvtxParser;
use failure::Error;
use log::*;
use pretty_env_logger;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let matches = app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::from_usage("<INPUT> 'Sets input evtx file path'").required(true))
        .get_matches();

    debug!("{:?}", &matches);

    let fp = PathBuf::from(matches.value_of("INPUT").unwrap());

    let mut parser = EvtxParser::from_path(fp).unwrap();
    for record in parser.records() {
        match record {
            Ok(r) => println!("Record {}\n{}", r.event_record_id, r.data),
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(println!())
}
