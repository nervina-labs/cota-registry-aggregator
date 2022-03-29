use crate::error::Error;
use ckb_jsonrpc_types::{BlockNumber, CellOutput, JsonBytes, OutPoint, Uint32};
use dotenv::dotenv;
use jsonrpc_http_server::jsonrpc_core;
use serde::Deserialize;
use serde_json::{from_str, json, Map, Value};
use std::env;

const TESTNET_REGISTRY_COTA_CODE_HASH: &str =
    "0x9302db6cc1344b81a5efee06962abcb40427ecfcbe69d471b01b2658ed948075";
const TESTNET_REGISTRY_COTA_ARGS: &str = "0xf9910364e0ca81a0e074f3aa42fe78cfcc880da6";
const MAINNET_REGISTRY_COTA_CODE_HASH: &str =
    "0x90ca618be6c15f5857d3cbd09f9f24ca6770af047ba9ee70989ec3b229419ac7";
const MAINNET_REGISTRY_COTA_ARGS: &str = "0x563631b49cee549f3585ab4dde5f9d590f507f1f";

pub async fn get_registry_smt_root() -> Result<Option<Vec<u8>>, Error> {
    dotenv().ok();

    let ckb_indexer_url = env::var("CKB_INDEXER")
        .map_err(|_e| Error::CKBIndexerError("CKB_INDEXER must be set".to_owned()))?;

    let mut req_json = Map::new();
    req_json.insert("id".to_owned(), json!("1"));
    req_json.insert("jsonrpc".to_owned(), json!("2.0"));
    req_json.insert("method".to_owned(), json!("get_cells"));
    req_json.insert("params".to_owned(), generate_params()?);

    let client = reqwest::Client::new();

    let resp = client
        .post(ckb_indexer_url)
        .json(&req_json)
        .send()
        .await
        .map_err(|e| Error::CKBIndexerError(e.to_string()))?;
    let output = resp
        .json::<jsonrpc_core::response::Output>()
        .await
        .map_err(|e| Error::CKBIndexerError(e.to_string()))?;

    let result: CellPagination = match output {
        jsonrpc_core::response::Output::Success(success) => {
            serde_json::from_value::<CellPagination>(success.result)
                .map_err(|_e| Error::CKBIndexerError("Parse response error".to_owned()))
        }
        jsonrpc_core::response::Output::Failure(failure) => {
            Err(Error::CKBIndexerError(failure.error.message))
        }
    }?;
    if result.objects.is_empty() {
        return Err(Error::CKBIndexerError(
            "Registry live cells should not empty".to_owned(),
        ));
    }
    let cell_data = result.objects.first().unwrap().output_data.as_bytes();
    match cell_data.len() {
        1 => Ok(None),
        33 => Ok(Some(cell_data[1..].to_vec())),
        _ => Err(Error::CKBIndexerError(
            "Registry cell data length error".to_owned(),
        )),
    }
}

fn generate_params() -> Result<Value, Error> {
    dotenv().ok();

    let is_mainnet: bool = match env::var("IS_MAINNET") {
        Ok(mainnet) => from_str::<bool>(&mainnet).unwrap(),
        Err(_e) => false,
    };

    let (code_hash, args) = if is_mainnet {
        (MAINNET_REGISTRY_COTA_CODE_HASH, MAINNET_REGISTRY_COTA_ARGS)
    } else {
        (TESTNET_REGISTRY_COTA_CODE_HASH, TESTNET_REGISTRY_COTA_ARGS)
    };

    Ok(json!([
        {
            "script": {
                "code_hash": code_hash,
                "hash_type": "type",
                "args": args,
            },
            "script_type": "type",
        },
        "asc",
        "0x1"
    ]))
}

#[derive(Deserialize)]
struct Cell {
    #[serde(skip_deserializing)]
    _output:       CellOutput,
    output_data:   JsonBytes,
    #[serde(skip_deserializing)]
    _out_point:    OutPoint,
    #[serde(skip_deserializing)]
    _block_number: BlockNumber,
    #[serde(skip_deserializing)]
    _tx_index:     Uint32,
}

#[derive(Deserialize)]
struct CellPagination {
    objects:      Vec<Cell>,
    #[serde(skip_deserializing)]
    _last_cursor: JsonBytes,
}
