# p2p_handshake_solana
Peer 2 peer handshake in Solana

## Example usage:

To make a handshake, it is necessary to have a node running on Solana network with which communication can be established.
Officially solana supported local node can be installed with Solana CLI tool suite [solana-test-validator](https://docs.solanalabs.com/cli/install).
After successful installation, to start a local node, in a separate terminal following command has to be run:

```bash
solana-test-validator
```

To communicate with the local node that has been provided by the solana-test-validator, following command has to be run:

```bash
$ cargo run http://127.0.0.1:8899 -t 1000
```

If the handshake was successful, following message should appear:

```bash
[2024-02-21T21:26:59.873Z]  INFO: p2p_handshake_solana/6568 on PC: Successfully performed handshake for Node http://127.0.0.1:8899 (file=src/solana/client_pool.rs,line=72,target=p2p_handshake_solana::solana::client_pool)
```

If the handshake was not successful, based on the error, something similar should appear:

```bash
[2024-02-21T21:46:21.919Z] ERROR: p2p_handshake_solana/40551 on PC: Error performing handshake: Failed to invoke get version (file=src/main.rs,line=22,target=p2p_handshake_solana)
Error: Failed to invoke get version

Caused by:
    0: Failed to get response from the remote node
    1: error sending request for url (http://127.0.0.1:8899/): error trying to connect: tcp connect error: Connection refused (os error 111)
    2: error trying to connect: tcp connect error: Connection refused (os error 111)
    3: tcp connect error: Connection refused (os error 111)
    4: Connection refused (os error 111)
```

To run handshakes on multiple nodes, following command has to be run:

```bash
$ cargo run http://127.0.0.1:8899 http://api.testnet.solana.com http://api.devnet.solana.com -t 1000
```

It is possible to also run it with the bunyan formatter which would output a nice looking log:

```bash
$ cargo run http://127.0.0.1:8899 http://api.testnet.solana.com http://api.devnet.solana.com -t 1000 | bunyan
```
