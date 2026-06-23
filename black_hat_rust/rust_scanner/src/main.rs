mod ports;
mod error;
mod subdomains;

use rayon::prelude::*;

fn main() -> Result<()> { 
    // custom threadpool to improve speed
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    // pool.install is required to use our custom threadpool
    pool.install(|| {
        let scan_result: Vec<SubDomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();

        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("      {}", port.port);
            }

            println!("");
        }
    });
}
