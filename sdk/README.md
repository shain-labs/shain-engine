# @shain/sdk

TypeScript client SDK for the Shain on-chain program.

## Install

```bash
git clone https://github.com/shain-labs/shain-engine
cd shain-engine/sdk
npm install
npm run build
```

## Usage

```ts
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { ShainClient } from "@shain/sdk";

const connection = new Connection("https://api.devnet.solana.com", "confirmed");
const wallet = Keypair.generate();

const client = new ShainClient({
  connection,
  programId: new PublicKey("5BXxVhThrj7irsqNRGzHG3y1CSap4u3HoBmkVHfs4CNx"),
  wallet,
});

// open a 24h private session
const session = await client.startSession();
// { sessionPda, expiresAt, startedAt, actionsCount: 0 }

// gate a downstream action
const hook = await client.gatedAction({ tag: 1n });
// { signature, actionsCount: 1 }

// close after expiry (anyone can call)
const closed = await client.closeSession({ user: wallet.publicKey });
// { refundedLamports }
```

## Surface

| Export             | Kind       | Description                                     |
| ------------------ | ---------- | ----------------------------------------------- |
| `ShainClient`    | class      | High-level client (create / gate / close)       |
| `derivePdas`       | function   | Compute config, treasury, session PDAs          |
| `SHAIN_SEEDS`    | constant   | Program seed byte arrays                        |
| `ShainConfig`    | interface  | Config account layout                           |
| `ShainSession`   | interface  | Session account layout                          |
| `ShainSdkError`  | class      | SDK-level error with typed reasons              |

See [`src/index.ts`](./src/index.ts) for the exhaustive public surface.
