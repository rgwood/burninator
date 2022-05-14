use anyhow::Result;
use argh::FromArgs;
use wmi::{COMLibrary, WMIConnection};

use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use wmi::Variant;

fn default_num_threads() -> usize {
    num_cpus::get() / 2
}

#[derive(FromArgs)]
/// 🔥🥵 Keep the computer above a constant temperature 🥵🔥
struct Args {
    /// the minimum temperature in Celsius (default = 35)
    #[argh(option, default = "35")]
    min_temp: usize,

    /// the number of threads to use for busy work (default = half the number of logical cores)
    #[argh(option, default = "default_num_threads()")]
    num_threads: usize,
}

fn main() -> Result<()> {
    const SLEEP_DURATION_SECONDS: u64 = 10;
    let args: Args = argh::from_env();

    println!("🔥🚒 The Burninator v0.1 🚒🔥");
    println!("Will do busy work to keep computer above {}C. Press ctrl+c to exit.", args.min_temp);
    println!();

    loop {
        let system_temp_c = get_system_temp_c()?;
        println!("Average temperature: {system_temp_c:.1}C");

        if system_temp_c < args.min_temp as f64 {
            println!("Temperature too low! Doing busy work 📈");
            let mut thread_handles: Vec<JoinHandle<()>> = vec![];

            for _ in 0..args.num_threads {
                thread_handles.push(thread::spawn(|| {
                    busy_work(10);
                }));
            }

            for handle in thread_handles {
                handle.join().unwrap();
            }
        } else {
            println!("Temperature is high enough! Going to sleep for a bit 💤");
            thread::sleep(Duration::from_secs(SLEEP_DURATION_SECONDS));
        }
    }

    Ok(())
}

fn busy_work(seconds: u64) {
    let start = Instant::now();
    let end = start + Duration::from_secs(seconds);
    // use up lots of CPU
    while Instant::now() < end {}
}

fn get_system_temp_c() -> Result<f64, anyhow::Error> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;
    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query(
            "SELECT * FROM Win32_PerfFormattedData_Counters_ThermalZoneInformation",
        )
        .unwrap();

    
    let mut temps_c: Vec<f64> = vec![];
    for os in results {
        // println!("{:#?}", os);

        let temp_kelvin: f64 = match os.get("HighPrecisionTemperature").unwrap() {
            Variant::UI4(val) => *val as f64 / 10.0,
            _ => panic!("Could not retrieve temperature"),
        };

        if temp_kelvin >= 273.0 {
            let temp_celsius = temp_kelvin - 273.0;
            temps_c.push(temp_celsius);
        }
    }

    Ok(mean(&temps_c))
}

fn mean(list: &[f64]) -> f64 {
    let sum: f64 = Iterator::sum(list.iter());
    sum / (list.len() as f64)
}
