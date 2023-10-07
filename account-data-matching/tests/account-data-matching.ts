import { join } from "path";
import { readFileSync } from "fs";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountDataMatching } from "../target/types/account_data_matching";

describe("account-data-matching", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AccountDataMatching as Program<AccountDataMatching>;

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

  it("un authorized user can access it!", async () => {
    // Add your test here.
    const tx = await program.methods.update(100).accounts({
      myAccount: myAccount.publicKey,
      user: unAuthorizedUser.publicKey,
    }).signers([]).rpc();
    console.log("Your transaction signature", tx);

    const data = await program.account.myAccount.fetch(myAccount.publicKey);
    assert(data.data == 100);
  });
});
