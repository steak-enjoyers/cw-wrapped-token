use cosmwasm_std::{to_binary, Addr, Coin, Response, StdResult, WasmMsg};
use cw_bank::msg as bank;

use crate::{error::ContractError, NAMESPACE};

pub fn wrap(sender: Addr, funds: Vec<Coin>) -> Result<Response, ContractError> {
    let mint_msgs = funds
        .into_iter()
        .map(|coin| {
            Ok(WasmMsg::Execute {
                contract_addr: "bank".into(),
                msg: to_binary(&bank::ExecuteMsg::Mint {
                    to: sender.to_string(),
                    denom: format!("{NAMESPACE}/{}", coin.denom),
                    amount: coin.amount,
                })?,
                funds: vec![],
            })
        })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(Response::new()
        .add_messages(mint_msgs)
        .add_attribute("action", "cw-wrapped-token/wrap"))
}

pub fn unwrap(self_addr: Addr, sender: Addr, funds: Vec<Coin>) -> Result<Response, ContractError> {
    // TODO: change this to a BankMsg::Send once support for it is implemented
    // in cw-sdk state machine
    let send_msg = WasmMsg::Execute {
        contract_addr: "bank".into(),
        msg: to_binary(&bank::ExecuteMsg::Send {
            to: sender.to_string(),
            coins: funds
                .iter()
                .map(|coin| {
                    Ok(Coin {
                        denom: subdenom_of(&coin.denom)?,
                        amount: coin.amount,
                    })
                })
                .collect::<Result<Vec<_>, ContractError>>()?,
        })?,
        funds: vec![],
    };

    let burn_msgs = funds
        .into_iter()
        .map(|coin| {
            Ok(WasmMsg::Execute {
                contract_addr: "bank".into(),
                msg: to_binary(&bank::ExecuteMsg::Burn {
                    from: self_addr.to_string(),
                    denom: coin.denom,
                    amount: coin.amount,
                })?,
                funds: vec![],
            })
        })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(Response::new()
        .add_message(send_msg)
        .add_messages(burn_msgs)
        .add_attribute("action", "cw-wrapped-token/unwrap"))
}

fn subdenom_of(denom: &str) -> Result<String, ContractError> {
    let Some((namespace, subdenom)) = denom.split_once('/') else {
        return Err(ContractError::NotWrappedToken {
            denom: denom.into(),
        });
    };

    if namespace != NAMESPACE {
        return Err(ContractError::NotWrappedToken {
            denom: denom.into(),
        });
    }

    Ok(subdenom.into())
}
