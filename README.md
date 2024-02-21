# p2p_handshake_solana
Peer 2 peer handshake in Solana

## Example usage:

Firstly it is necessary to have a node running on Solana network with which communication can be established.
Officially solana supported local node can be installed with Solana CLI tool suite [solana-test-validator](https://docs.solanalabs.com/cli/install).
After successful installation, to start a local node running, in a separate terminal following has to be run:

```bash
solana-test-validator
```

To communicate with the local node that has been provided by the solana-test-validator, following command has to be run:

```bash
$ cargo run http://127.0.0.1:8899 -t 1000
```

If the handshake is successful, following message should appear:

```bash
[2024-02-21T21:26:59.873Z]  INFO: p2p_handshake_solana/39566 on PC: Successfully performed handshake (file=src/main.rs,line=20,target=p2p_handshake_solana)
```

If the handshake is not successful, based on the error, something similar should appear:

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
