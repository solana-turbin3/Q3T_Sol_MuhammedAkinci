import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import makerWallet from "../wallets/wba-wallet.json";
import takerWallet from "../wallets/dev-wallet.json";
import { PublicKey } from "@solana/web3.js";
import { randomBytes } from "crypto";
import { getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("escrow", () => {
  
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Escrow as Program<Escrow>;

  const maker = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(makerWallet)
  );

  const taker = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(takerWallet)
  );

  const mintA = new PublicKey("4nPE5BApRVTDJHo6E4mwAriAYRmtB8eAvfxB6Fr5kMW9");
  const mintB = new PublicKey("4nPE5BApRVTDJHo6E4mwAriAYRmtB8eAvfxB6Fr5kMW9");

  const [makerAtaA, makerAtaB, takerAtaA, takerAtaB] = [maker, taker]
    .map((a) =>
      [mintA, mintB].map((m) =>
        getAssociatedTokenAddressSync(m, a.publicKey, false, TOKEN_PROGRAM_ID)
      )
    )
    .flat();
  
  const seed = new BN(randomBytes(8));

  const escrow = PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), maker.publicKey.toBuffer(), seed.toArrayLike(Buffer, "le", 8)],
    program.programId
  )[0];

  const vault = getAssociatedTokenAddressSync(mintA, escrow, true, TOKEN_PROGRAM_ID);

  const accounts = {
    maker: maker.publicKey,
    taker: taker.publicKey,
    mintA,
    mintB,
    makerAtaA,
    makerAtaB,
    takerAtaA,
    takerAtaB,
    escrow: escrow,
    vault: vault,
    tokenProgram: TOKEN_PROGRAM_ID
  }

  console.log(`Maker: ${maker.publicKey.toString()}`);
  console.log(`Taker: ${taker.publicKey.toString()}`);
  console.log(`Escrow: ${escrow.toString()}`);
  console.log(`Vault: ${vault.toString()}`);

  it("Making!", async () => {
    // Add your test here.
    const tx = await program.methods
    .make(seed, new BN(1_000_000), new BN(500_000))
    .accounts({...accounts})
    .signers([maker])
    .rpc();
    console.log("Your maker transaction signature", tx);
  });


  it("Taking!", async () => {
    // Add your test here.
    const tx = await program.methods
      .take()
      .accounts({...accounts})
      .signers([taker])
      .rpc();
    console.log("Your taker transaction signature", tx);
  });

  xit("Refunding!", async () => {
    // Add your test here.
    const tx = await program.methods
      .refund()
      .accounts({...accounts})
      .signers([maker])
      .rpc();
    console.log("Your Refund transaction signature", tx);
  });
});