use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    /// Send native tokens to contract, receive wrapped tokens.
    Wrap {},

    /// Send wrapped tokens to contract, receive native tokens.
    Unwrap {},
}
