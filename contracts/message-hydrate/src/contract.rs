#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, DepsMut, Env, MessageInfo, Response, StdResult, CosmosMsg, WasmMsg};
use cw2::set_contract_version;
use serde_json_wasm;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg};
use crate::state::{State, STATE};
use crate::vars::{get_variables_from_string, replace_variables};
use cw20::Cw20ExecuteMsg;


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:message_hydration";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::HydrateMsg {input_msg, vars} => hydrate_message(deps, info, input_msg, vars),
    }
}

pub fn hydrate_message(
        _deps:DepsMut, 
        _info: MessageInfo, 
        mut input_msg: String, 
        vars: String) -> 
    Result<Response, ContractError> {
        let var_list = get_variables_from_string(vars);
        // Replace variable name with value in string
        replace_variables(&mut input_msg, &var_list);

        // Create CosmosMsg struct from string
        let wasm_msg: CosmosMsg = serde_json_wasm::from_str(&input_msg).unwrap();
        
        let wasm_hydrated_msg: CosmosMsg = match wasm_msg {
            CosmosMsg::Wasm(WasmMsg::Execute {contract_addr, msg, funds} ) => {
                // Looking for and replacing variable names within msg
                // Inthis particular case there is nothing to replace but there can be a scenario
                // where contract address or amount could be variables
                let mut decoded_msg = String::from_utf8(msg.0).unwrap();
                replace_variables(&mut decoded_msg, &var_list);
                
                // Create Send struct from slice
                let cw20_message: Cw20ExecuteMsg = serde_json_wasm::from_str(&decoded_msg).unwrap();
                
                let cw20_hydrated_message = match cw20_message {
                    Cw20ExecuteMsg::Send {contract, amount, msg } => {
                        // Looking for and replacing variables within msg
                        let mut decoded_msg = String::from_utf8(msg.0).unwrap();
                        replace_variables(&mut decoded_msg, &var_list);

                        serde_json_wasm::to_string(&Cw20ExecuteMsg::Send {contract, amount, msg: to_json_binary(&decoded_msg)?}).unwrap()
                        // Returning Send Cw20ExecuteMsg struct with hydrated Send message
                    },
                    _ => todo!(), // Add here more cases
                };
                // Returning CosmosMsg sturct with hydrated Wasm message
                // For some reason, my msg does not get converted to binary in the next line.
                // I tried different things and I was only able to do it if I use the function twice.
                // That would be completerlly wrong in my opinion so here I think I'm missing something
                CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, msg: to_json_binary(&cw20_hydrated_message)?, funds })
            },
            _ => todo!(), // Add here more cases
        };
        Ok(Response::new().add_message(wasm_hydrated_msg))
    }


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::coins;
    use serde_json::json;

    #[test]
    fn test_hydrate() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::HydrateMsg {
            input_msg: json!({
                "wasm": {
                    "execute":{
                        "contract_addr":"$warp.var.variable1",
                        "msg":"eyJzZW5kIjp7ImNvbnRyYWN0IjoidGVycmE1NDMyMSIsImFtb3VudCI6IjEyMzQ1IiwibXNnIjoiZXlKbGVHVmpkWFJsWDNOM1lYQmZiM0JsY21GMGFXOXVjeUk2ZXlKdmNHVnlZWFJwYjI1eklqcGJleUpoYzNSeWIxOXpkMkZ3SWpwN0ltOW1abVZ5WDJGemMyVjBYMmx1Wm04aU9uc2lkRzlyWlc0aU9uc2lZMjl1ZEhKaFkzUmZZV1JrY2lJNklpUjNZWEp3TG5aaGNpNTJZWEpwWVdKc1pURWlmWDBzSW1GemExOWhjM05sZEY5cGJtWnZJanA3SW01aGRHbDJaVjkwYjJ0bGJpSTZleUprWlc1dmJTSTZJaVIzWVhKd0xuWmhjaTUyWVhKcFlXSnNaVElpZlgxOWZWMHNJbTFwYm1sdGRXMWZjbVZqWldsMlpTSTZJaVIzWVhKd0xuWmhjaTUyWVhKcFlXSnNaVE1pTENKMGJ5STZJaVIzWVhKd0xuWmhjaTUyWVhKcFlXSnNaVFFpTENKdFlYaGZjM0J5WldGa0lqb2lKSGRoY25BdWRtRnlMblpoY21saFlteGxOU0o5ZlE9PSJ9fQ==",
                        "funds":[]
                    }
                }
            }).to_string(),
            vars: "[\"$warp.var.variable1\": \"terra12345\", \"$warp.var.variable2\": \"uterra\", \"$warp.var.variable3\": \"54321\", \"$warp.var.variable4\": \"terra11111\", \"$warp.var.variable5\": \"0.05\",]".to_string()
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        println!("{:?}", res);

        /*
        This is how my response looks like
        { messages: [SubMsg { id: 0, msg: Wasm(Execute 
            { contract_addr: "terra12345", 
            msg: "{\"send\":{\"contract\":\"terra54321\",\"amount\":\"12345\",\"msg\":\"IntcImV4ZWN1dGVfc3dhcF9vcGVyYXRpb25zXCI6e1wib3BlcmF0aW9uc1wiOlt7XCJhc3Ryb19zd2FwXCI6e1wib2ZmZXJfYXNzZXRfaW5mb1wiOntcInRva2VuXCI6e1wiY29udHJhY3RfYWRkclwiOlwidGVycmExMjM0NVwifX0sXCJhc2tfYXNzZXRfaW5mb1wiOntcIm5hdGl2ZV90b2tlblwiOntcImRlbm9tXCI6XCJ1dGVycmFcIn19fX1dLFwibWluaW11bV9yZWNlaXZlXCI6XCI1NDMyMVwiLFwidG9cIjpcInRlcnJhMTExMTFcIixcIm1heF9zcHJlYWRcIjpcIjAuMDVcIn19Ig==\"}}",
            funds: [] 
        }
         */
    }
}
