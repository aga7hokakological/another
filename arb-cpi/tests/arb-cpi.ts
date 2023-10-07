import { join } from "path";
import { readFileSync } from "fs";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ArbCpi } from "../target/types/arb_cpi";
import { ArbCpiProgram } from "../target/types/arb_cpi_program";

describe("arb-cpi", async () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  let connection = provider.connection;
  anchor.setProvider(provider);

  const cpi = anchor.workspace.ArbCpi as Program<ArbCpi>;
  const cpiProgram = anchor.workspace.ArbCpi as Program<ArbCpiProgram>;

  const WALLET_PATH = join(process.env["HOME"]!, ".config/solana/id.json");
  const admin = anchor.web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(readFileSync(WALLET_PATH, { encoding: "utf-8" })))
  );

  const userAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx1 = await cpiProgram.methods.initialize().accounts({
      userAccount: userAccount.publicKey,
      user: admin.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([admin]).rpc();

    const tx = await cpi.methods.login("SecurePassword").accounts({
      user: userAccount.publicKey,
      storePasswordProgram: cpiProgram.programId,
    }).signers([]).rpc();
    console.log("Your transaction signature", tx);

    
  });
});
