use models::*;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountRequest {
    pub id: AccountId,
    pub user_id: WorkspaceId,
    pub currency: Currency,
    pub name: String,
    pub daily_limit_type: Option<DailyLimitType>,
}

impl From<(CreateAccount, WorkspaceId)> for CreateAccountRequest {
    fn from(req: (CreateAccount, WorkspaceId)) -> Self {
        Self {
            id: req.0.id,
            name: req.0.name,
            currency: req.0.currency,
            user_id: req.1,
            daily_limit_type: req.0.daily_limit_type,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUsersAccountsParams {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionRequest {
    pub id: TransactionId,
    pub user_id: WorkspaceId,
    pub from: AccountId,
    pub to: Receipt,
    pub to_type: ReceiptType,
    pub to_currency: Currency,
    pub value_currency: Currency,
    pub exchange_id: Option<ExchangeId>,
    pub exchange_rate: Option<f64>,
    pub value: Amount,
    pub fee: Amount,
}

impl From<(CreateTransaction, WorkspaceId)> for CreateTransactionRequest {
    fn from(req: (CreateTransaction, WorkspaceId)) -> Self {
        Self {
            id: req.0.id,
            user_id: req.1,
            from: req.0.from,
            to: req.0.to,
            to_type: req.0.to_type,
            to_currency: req.0.to_currency,
            value_currency: req.0.value_currency,
            exchange_id: req.0.exchange_id,
            exchange_rate: req.0.exchange_rate,
            value: req.0.value,
            fee: req.0.fee,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUsersTransactionsParams {
    pub limit: i64,
    pub offset: i64,
}
