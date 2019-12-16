use super::*;

pub fn parse_params(opts: &Config) -> TxParams {
  TxParams{
    address: Some(opts.address.clone()),
    transfer: Some(opts.transfer),
    conversion: Some(opts.conversion),
    coinbase: Some(opts.mining),
    burn: Some(opts.burn),
    ..Default::default()
  }
}

pub fn parse_tx(tx: TxsAction) -> ParsedTx {
  let toamount = match tx.toamount {
    Some(amount) => Some(p_amount(amount)),
    None => None
  };
  let mut parsed_outputs = Vec::new();
  if tx.outputs.len() == 0 {
    parsed_outputs.push(ParsedOutput{..Default::default()})
  } else {
    for output in tx.outputs {
      parsed_outputs.push(ParsedOutput {
        address: output.address,
        amount: p_amount(output.amount)
      })
    }
  }
  
  ParsedTx {
    hash: tx.hash,
    height: tx.height,
    timestamp: tx.timestamp,
    executed: tx.executed,
    txindex: tx.txindex,
    txaction: tx.txaction,
    fromaddress: tx.fromaddress,
    fromasset: tx.fromasset,
    fromamount: p_amount(tx.fromamount),
    toasset: tx.toasset,
    toamount,
    outputs: parsed_outputs
  }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedTx {
  pub hash: String,
  pub height: usize,
  pub timestamp: String,
  pub executed: isize,
  pub txindex: usize,
  pub txaction: usize,
  pub fromaddress: String,
  pub fromasset: String,
  pub fromamount: Decimal,
  pub toasset: Option<String>,
  pub toamount: Option<Decimal>,
  #[serde(default)]
  pub outputs: Vec<ParsedOutput>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedOutput {
  pub address: String,
  pub amount: Decimal,
}