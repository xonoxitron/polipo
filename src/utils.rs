pub fn is_method_public(method: &str) -> bool {
    [
        "Time",
        "Assets",
        "AssetPairs",
        "Ticker",
        "Depth",
        "Trades",
        "Spread",
        "OHLC",
    ]
    .contains(&method)
}

pub fn is_method_private(method: &str) -> bool {
    [
        "Balance",
        "TradeBalance",
        "OpenOrders",
        "ClosedOrders",
        "QueryOrders",
        "TradesHistory",
        "QueryTrades",
        "OpenPositions",
        "Ledgers",
        "QueryLedgers",
        "TradeVolume",
        "AddOrder",
        "CancelOrder",
        "DepositMethods",
        "DepositAddresses",
        "DepositStatus",
        "WithdrawInfo",
        "Withdraw",
        "WithdrawStatus",
        "WithdrawCancel",
        "GetWebSocketsToken",
    ]
    .contains(&method)
}

pub fn get_method_type(method: &str) -> &str {
    let method_type: &str = if is_method_public(method) {
        "public"
    } else if is_method_private(method) {
        "private"
    } else {
        "invalid"
    };
    method_type
}
