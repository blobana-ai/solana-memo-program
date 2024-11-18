import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorMemo } from "../target/types/anchor_memo";

describe("blob-memo", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorMemo as Program<AnchorMemo>;

  it("Is called by the owner", async () => {
    // Add your test here.
    const tx = await program.methods
      .postMessage(Buffer.from("message"))
      .accounts({
        signerAccount: program.provider.publicKey,
      })
      .rpc();

    console.log(program.provider.publicKey.toString());

    console.log("Transaction hash:", tx);
  });
});
