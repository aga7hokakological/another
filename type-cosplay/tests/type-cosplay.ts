import { join } from "path";
import { readFileSync } from "fs";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import * as token from "@solana/spl-token";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TypeCosplay } from "../target/types/type_cosplay";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

describe("type-cosplay", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  let connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.TypeCosplay as Program<TypeCosplay>;

  const WALLET_PATH = join(process.env["HOME"]!, ".config/solana/id.json");
  const admin = anchor.web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(readFileSync(WALLET_PATH, { encoding: "utf-8" })))
  );

  const adminAccount = anchor.web3.Keypair.generate()
  const userAccount = anchor.web3.Keypair.generate()
  const anotherUser = anchor.web3.Keypair.generate();
  const vault = anchor.web3.Keypair.generate()

  it("Initialized Admin!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeAdmin(admin.publicKey).accounts({
      adminAccount: adminAccount.publicKey,
      admin: admin.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Initialize User!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeUser(userAccount.publicKey).accounts({
      userAccount: userAccount.publicKey,
      user: anotherUser.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Initialized Vault!", async () => {
    let tokenmintPubkey = await token.createMint(
      connection,
      admin,
      admin.publicKey,
      null,
      9
    );

    let vaultTokenAcc = await token.createAssociatedTokenAccount(
      connection,
      admin,
      tokenmintPubkey,
      vault.publicKey
    );
    // Add your test here.
    const tx = await program.methods.initializeVault().accounts({
      vaultAccount: vault.publicKey,
      admin: admin.publicKey,
      tokenMint: tokenmintPubkey,
      token: vaultTokenAcc,
      tokenProgram: token.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Update Fees!", async () => {
    // Add your test here.
    const tx = await program.methods.updateVaultFees(new BN(10)).accounts({
      vaultAccount: vault.publicKey,
      user: anotherUser.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([]).rpc();
    console.log("Your transaction signature", tx);
  });
});
