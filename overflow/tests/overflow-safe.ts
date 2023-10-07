import { join } from "path";
import { readFileSync } from "fs";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OverflowSafe } from "../target/types/overflow_safe";
import { assert } from "chai";

describe("overflow safe", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  let connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.OverflowSafe as Program<OverflowSafe>;

  const WALLET_PATH = join(process.env["HOME"]!, ".config/solana/id.json");
  const admin = anchor.web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(readFileSync(WALLET_PATH, { encoding: "utf-8" })))
  );

  const dataAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(200, 100).accounts({
      data: dataAccount.publicKey,
      user: admin.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([admin, dataAccount]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Overflow!", async () => {
    // Add your test here.
    const tx = await program.methods.add().accounts({
      data: dataAccount.publicKey,
      user: admin.publicKey,
    }).signers([admin]).rpc();
    
    // const val = await program.account.data.fetch(dataAccount.publicKey);
  });
});
