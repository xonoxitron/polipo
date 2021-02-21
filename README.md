![polipo](./polipo-logo.png)
=

### 🐙 Minimal, elegant, fast, async Kraken exchange API client | Written in Rust

<br>

# Description

**```polipo```** library aims to interface your software with the [Kraken exchange API](https://support.kraken.com/hc/en-us/categories/360000080686-API) in no time.

<br>

# Prerequisites
The [Kraken exchange API](https://kraken.com) allows interaction with both **```public```** and **```private```** data.


Only for the **private** one, you need to issue an **```API-Key```** and an **```API-Secret```** to the **```polipo```** library, in order to [generate authentication strings](https://support.kraken.com/hc/en-us/articles/360022635592-Generate-authentication-strings-REST-API-) for signed requests.

If you are not familiar with, please have a look at [API documentation](https://support.kraken.com/hc/en-us/categories/360001806372) for a general overview, or at [Generate API keys](https://support.kraken.com/hc/en-us/articles/360022839451-Generate-API-keys) document.

<br>

# Implementation

Add this to your **```Cargo.toml```**:
```toml
[dependencies]
polipo = {version = 0.1.7}
```

and then add this to your **```code```**:
```rust
use polipo;
```

<br>

# Methods

```rust
polipo::print_crate_info();
```
**Description**: prints **```crate```** information (name, description, version,author and repository).

<br>

```rust
polipo::set_kraken_api_credentials(api_key: String, api_secret: String);
```

**Description**: sets **```Kraken API```** credentials for **```private```** data access.

**Required**:
* *api_key*: ```String```
* *api_secret*: ```String```

<br>

```rust
polipo::get_kraken_api_response(method: String, url_encoded_body: String).await;
```
**Description**: performs **```public```** or **```private```** requests, according to issued **```method```** and **```url_encoded_body```** data.

**Required**:
* *method*: ```String``` (eg: *"public"* or *"private"*)
* *url_encoded_body*: ```String``` (eg: *"pair=ethusd&since=1574067140000000000"*)

**Output**: any response is in stringified **```JSON```** format (parse accordingly with the outcome shape).

<br>

# Example

The example below shows how easy is to implement **```polipo```** from zero knowledge.

```rust
use polipo;

async fn query(method: &str, url_encoded_body: &str) -> String {
    polipo::get_kraken_api_response(method.to_string(), url_encoded_body.to_string()).await
}

#[tokio::main]
async fn main() {
    
    // printing crate information
    polipo::print_crate_info();

    // performing some queries on public data
    println!("public/Assets: {}", query("Assets", "").await);
    println!("public/Ticker: {}", query("Ticker", "pair=xdgeur").await);

    // issuing credentials enables private data requests
    polipo::set_kraken_api_credentials(
        "YOUR_KRAKEN_API_KEY_HERE".to_string(),
        "YOUR_KRAKEN_API_SECRET".to_string(),
    );

    // performing some queries on private data
    println!("private/Balance: {}", query("Balance", "").await);
    println!("private/TradeBalance: {}", query("TradeBalance","asset=ada").await);
    println!("private/Trades: {}", query("Trades", "pair=ethusd&since=1574067140000000000").await);
}
```

<br>

# Disclaimer
This software comes without any kind of warranties. 

I will not be liable for any damages related to the use or the misuse of this software.

You are the sole responsible.