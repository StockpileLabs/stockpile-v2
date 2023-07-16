import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StockpileV2 } from "../target/types/stockpile_v2";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";

describe("stockpile-v2", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl("devnet"));

  const program = anchor.workspace.StockpileV2 as Program<StockpileV2>;

  it("createProject", async () => {
    // Generate keypairs for payer, and admins
    const payer = anchor.web3.Keypair.generate();
    let adminKp1 = anchor.web3.Keypair.generate();
    let adminKp2 = anchor.web3.Keypair.generate();

    // Fund payer account
    connection.requestAirdrop(payer.publicKey, 2);

    // Generate a beneficiary keypair and random projectId
    let beneficiary = anchor.web3.Keypair.generate().publicKey;
    let projectId = new Uint8Array(Math.random())

    // Find PDA address
    const [fundraiserPDA, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
        [utf8.encode("project"), projectId],
        program.programId
    );

    // Define dummy values
    let name = "Sample Project";
    let admins = [adminKp1.publicKey, adminKp2.publicKey];
    let goal = 100;
  
    // Let it fly
    const tx = await program.methods.createProject(new anchor.BN(projectId), name, admins, beneficiary, goal)
    .accounts({
      payer: payer.publicKey,
      project: fundraiserPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();

    // If it passes, we get a friendly message
    console.log("🚀 Project Created! Transaction Hash:", tx);
  });

  it("createPool", async () => {
    // Generate payer keypair, and random poolId
    const payer = anchor.web3.Keypair.generate();
    let poolId = new Uint8Array(Math.random())

    // Fund payer account
    connection.requestAirdrop(payer.publicKey, 2);

    // Find PDA address
    const [poolPDA, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
        [utf8.encode("pool"), poolId],
        program.programId
    );

    // Define dummy values
    let name = "Sample Project";
    let start = new anchor.BN(Math.floor(Date.now() / 1000));
    let end = new anchor.BN(Math.floor(Date.now() / 1000) + 30000);
  
    // Alea iacta est
    const tx = await program.methods.createPool(new anchor.BN(poolId), name, start, end)
    .accounts({
      payer: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      pool: poolPDA,
    })
    .rpc();

    // If it passes, we get a friendly message
    console.log("👾 Funding Round Initialized! Transaction Hash:", tx);
  });
});
