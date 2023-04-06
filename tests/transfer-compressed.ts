import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TransferCompressed } from "../target/types/transfer_compressed";

describe("transfer-compressed", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TransferCompressed as Program<TransferCompressed>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
