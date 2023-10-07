import { join } from "path";
import { readFileSync } from "fs";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SignerAuthorizationSafe } from "../target/types/signer_authorization_safe";
import { assert } from "chai";

describe("signer-authorization-safe", async () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  let connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.SignerAuthorization as Program<SignerAuthorizationSafe>;

  const WALLET_PATH = join(process.env["HOME"]!, ".config/solana/id.json");
  const admin = anchor.web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(readFileSync(WALLET_PATH, { encoding: "utf-8" })))
  );

  const myAccount = anchor.web3.Keypair.generate()
  const unAuthorizedUser = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      myAccount: myAccount.publicKey,
      user: admin.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([admin, myAccount]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("un authorized user cannot access it!", async () => {
    // Add your test here.
    const tx = await program.methods.update(100).accounts({
      myAccount: myAccount.publicKey,
      user: admin.publicKey,
    }).signers([unAuthorizedUser]).rpc();
    console.log("Your transaction signature", tx);

    const data = await program.account.myAccount.fetch(myAccount.publicKey);
    assert(data.data == 100);
  });
});
