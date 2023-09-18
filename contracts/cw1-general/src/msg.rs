use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CosmosMsg;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))] // cw-orch automatic
pub enum ExecuteMsg {
    Execute { msgs: Vec<CosmosMsg> },
}

#[cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))] // cw-orch automatic
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(cw1::CanExecuteResponse)]
    CanExecute { sender: String, msg: CosmosMsg },
}
