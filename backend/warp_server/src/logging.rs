use serde::Serialize;
use std::fmt::Debug;


use simplelog::CombinedLogger;
use simplelog::TermLogger;
use simplelog::WriteLogger;
use std::fs::File;
use simplelog::LevelFilter;

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete
}

pub fn log_attach(method: HttpMethod, text: &str) {
    let method: &str = match method {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Put => "PUT",
        HttpMethod::Delete => "DELETE"
    } ;
    info!("Attaching: {:6}| api/{}", method, text);
}

pub fn log_attach_in_out<IN: Debug + Default, OUT: Debug + Serialize + Default >(method: HttpMethod, text: &str) {
    let method: &str = match method {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Put => "PUT",
        HttpMethod::Delete => "DELETE"
    } ;
    let in_name = format!("{:?}", IN::default());
    let in_name = in_name.split_whitespace().next().unwrap();
    let out_name = format!("{:?}", OUT::default());
    let out_name = out_name.split_whitespace().next().unwrap();
    info!("Attaching: {:6}| {:25} | In: {} | Out: {}", method, text, in_name, out_name);
}


/// Sets up logging for the server
pub fn setup_logging() {
    const LOGFILE_NAME: &'static str = "weekend.log";
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, simplelog::Config::default())
            .expect("Couldn't get terminal logger"),
        WriteLogger::new(
            LevelFilter::Debug,
            simplelog::Config::default(),
            File::create(LOGFILE_NAME).expect(
                "Couldn't create logfile",
            )
        ),
    ]).expect("Can't get logger.");
}

