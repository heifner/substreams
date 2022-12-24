// substream modules
use substreams_antelope_core::pb::antelope::{Block};
use substreams::errors::Error;

// local modules
mod abi;
mod pb;
use crate::pb::actions::{Action, Actions};
use crate::pb::tables::{DatabaseOperation, DatabaseOperations};

#[substreams::handlers::map]
fn map_action_traces(block: Block) -> Result<Actions, Error> {
    let mut actions = vec![];

    for trx in block.clone().all_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            let action = trace.action.as_ref().unwrap().clone();

            // validate ABIs
            let name = action.name;
            let json_data = action.json_data;
            if name == "transfer" {
                if !abi::is_transfer(&json_data) { continue; }
            } else if name == "issue" {
                if !abi::is_issue(&json_data) { continue; }
            } else if name == "create" {
                if abi::is_create(&json_data) { continue; }
            } else if name == "close"{
                if abi::is_close(&json_data) { continue; }
            } else if name == "open"{
                if abi::is_open(&json_data) { continue; }
            } else if name == "retire"{
                if abi::is_retire(&json_data) { continue; }
            } else { continue; }

            actions.push(Action {
                // trace information
                block_num: block.number,
                timestamp: Some(block.header.as_ref().unwrap().timestamp.as_ref().unwrap().clone()),
                trx_id: trx.id.clone(),
                action_ordinal: trace.action_ordinal.clone(),

                // action
                account: action.account,
                receiver: trace.receiver.clone(),
                name,

                // action data
                json_data,
            })
        }
    }
    Ok(Actions { actions })
}


#[substreams::handlers::map]
fn map_db_ops(block: Block) -> Result<DatabaseOperations, Error> {
    let mut db_ops = vec![];
    for trx in block.clone().all_transaction_traces() {
        // table deltas
        for db_op in &trx.db_ops {

            // validate ABIs
            if db_op.table_name == "accounts" {
                // TO-DO add ABI validation //;
            } else if db_op.table_name == "stats" {
                // TO-DO add ABI validation //;
            } else { continue; }

            db_ops.push(DatabaseOperation {
                // trace information
                block_num: block.number,
                timestamp: Some(block.header.as_ref().unwrap().timestamp.as_ref().unwrap().clone()),
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // database operation
                code: db_op.code.clone(),
                table_name: db_op.table_name.clone(),
                scope: db_op.scope.clone(),
                primary_key: db_op.primary_key.clone(),

                // table data
                old_data: db_op.old_data.clone(),
                new_data: db_op.new_data.clone(),
            })
        }
    }
    Ok(DatabaseOperations { db_ops })
}
