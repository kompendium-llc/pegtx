#![allow(non_snake_case)]
mod args;
mod files;
mod parsing;
mod utils; 

use utils::*;
use parsing::*;
use pegnetd::*;
use args::Config;
use structopt::StructOpt;
use rust_decimal::Decimal;
use indicatif::ProgressBar;
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<()> {
  let opts = Config::from_args();
  let api: Pegnetd;
  if opts.node.is_some() {
    let node = opts.node.clone().unwrap();
    api = Pegnetd::custom_node(string_to_static_str(node));
  } else {
    api = Pegnetd::open_node();
  }
  let mut params = parse_params(&opts);
  params.offset = Some(0);
  let filepath = match opts.output {
    Some(output) => output,
    None => params.address.clone().unwrap() + ".csv"
  };

  println!("\nRetrieving Address: {}\n", params.address.clone().unwrap());
  let writer =  files::create_writer(&filepath);
  let mut wtr = csv::WriterBuilder::new()
                  .has_headers(false)
                  .from_writer(writer);
  // Write Headers
  wtr.serialize(vec!["hash", "height", "timestamp", "executed", "txindex",
                "txaction", "fromaddress", "fromasset", "fromamount",
                  "toasset", "toamount", "address", "amount"]).unwrap();
  let response = transactions(&api, params.clone()).await;
  update_offset(response.result.nextoffset, &mut params);
  for tx in response.result.actions {
    wtr.serialize(parse_tx(tx)).expect("CSV Serialising");
  }
  let pb = ProgressBar::new(response.result.count as u64 + 1);
  pb.inc(50);
  while params.offset.is_some(){
    let response = transactions(&api, params.clone()).await;
    update_offset(response.result.nextoffset, &mut params);
    for tx in response.result.actions {
      wtr.serialize(parse_tx(tx)).expect("CSV Serialising");
    }
    pb.inc(50);
    wtr.flush().expect("Flushing CSV Writer");
  }
  pb.finish_with_message("Done");
  Ok(())
}
