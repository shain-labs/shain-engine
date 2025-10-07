import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { derivePdas } from "./pda";
import type { ShainClientOptions } from "./types";

export class ShainClient {
  public readonly connection: Connection;
  public readonly programId: PublicKey;
  public readonly wallet: Keypair;

  constructor(options: ShainClientOptions) {
    this.connection = options.connection;
    this.programId = options.programId;
    this.wallet = options.wallet as Keypair;
  }

  pdas(holder?: PublicKey) {
    return derivePdas({ programId: this.programId, holder });
  }
}
