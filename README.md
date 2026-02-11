# gbpcli

CLI for Google Business Profile API.

## Install

```bash
cargo install --git https://github.com/nasustim/gbpcli_rs
```

## Setup

Create a `.env` file with your Google OAuth credentials:

```
GOOGLE_BUSINESS_API_CLIENT_ID=your_client_id
GOOGLE_BUSINESS_API_CLIENT_SECRET=your_client_secret
GOOGLE_BUSINESS_API_REFRESH_TOKEN=your_refresh_token
```

## Usage

### List accounts

```bash
gbpcli list-accounts
```

With optional parameters:

```bash
gbpcli list-accounts \
  --parent-account accounts/123 \
  --page-size 10 \
  --page-token <token> \
  --filter "type=USER_GROUP"
```
