import { join } from "path";
import { readFileSync } from "fs";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ReInitialization } from "../target/types/re_initialization";

describe("re-initialization", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ReInitialization as Program<ReInitialization>;

  const WALLET_PATH = join(process.env["HOME"]!, ".config/solana/id.json");
  const admin = anchor.web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(readFileSync(WALLET_PATH, { encoding: "utf-8" })))
  );

  const myAccount = anchor.web3.Keypair.generate()
  const unAuthorizedUser = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const tx1 = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: admin.publicKey,
        newAccountPubkey: myAccount.publicKey,
        space: 32,
        lamports: await provider.connection.getMinimumBalanceForRentExemption(
          32
        ),
        programId: program.programId,
      })
    )

    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx1, [
      admin,
      myAccount,
    ])

    // await provider.connection.confirmTransaction(
    //   await provider.connection.requestAirdrop(
    //     unAuthorizedUser.publicKey,
    //     1 * anchor.web3.LAMPORTS_PER_SOL
    //   ),
    //   "confirmed"
    // )
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      myAccount: myAccount.publicKey,
      user: admin.publicKey,
      // systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([admin]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can be initialized again!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      myAccount: myAccount.publicKey,
      user: admin.publicKey,
      // systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([admin]).rpc();
    console.log("Your transaction signature", tx);
  });

});
