rm -rf /tmp/alice

RUST_LOG="info,runtime::ocw-sigtx=debug" \
./target/debug/node-template \
--base-path /tmp/alice \
--chain=local \
--alice \
--node-key 0000000000000000000000000000000000000000000000000000000000000001 \
--no-telemetry \
--rpc-external \
--rpc-cors=all \
--rpc-methods=Unsafe \
--ws-external \
--execution=Native \
--enable-offchain-indexing=true \
--pruning=archive
