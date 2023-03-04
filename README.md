# token-stream

Substream to capture all erc20 721 1155 token balance changes.

## Getting Started

Follow the quickstart guide here https://substreams.streamingfast.io/getting-started/quickstart

### Modules

#### map_transfers

Captures balance updates for all erc20,721,1155 tokens.

```
substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_transfers --start-block 16000080 -t +1
```

#### jsonout

Run using sinkfiles

```
substreams-sink-files run --encoder=lines --state-store=./output/state.yaml mainnet.eth.streamingfast.io:443 substreams.yaml jsonout ./output/files
```
