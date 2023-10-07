import { join } from "path";
import { readFileSync } from "fs";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DuplicateMutableAcc } from "../target/types/duplicate_mutable_acc";

describe("duplicate-mutable-acc", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  let connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.DuplicateMutableAcc as Program<DuplicateMutableAcc>;

  const WALLET_PATH = join(process.env["HOME"]!, ".config/solana/id.json");
  const admin = anchor.web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(readFileSync(WALLET_PATH, { encoding: "utf-8" })))
  );

  const myAccount1 = anchor.web3.Keypair.generate();
  const myAccount2 = anchor.web3.Keypair.generate();
  const unAuthorizedUser = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeAccounts().accounts({
      accountOne: myAccount1.publicKey,
      accountTwo: myAccount1.publicKey,
      signer: admin.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([admin]).rpc();

    console.log("Your transaction signature", tx);
  });

  it("same account has been passed!", async () => {
    // Add your test here.
    const tx = await program.methods.whoWon(new BN(100), new BN(40)).accounts({
      accountOne: myAccount1.publicKey,
      accountTwo: myAccount1.publicKey,
      // signer: admin.publicKey,
      // systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([]).rpc();
    console.log("Your transaction signature", tx);
  });
});
