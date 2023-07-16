import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StockpileV2 } from "../target/types/stockpile_v2";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";

describe("stockpile-v2", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StockpileV2 as Program<StockpileV2>;

  it("createProject", async () => {
    const payer = anchor.web3.Keypair.generate();
    let adminKp1 = anchor.web3.Keypair.generate();
    let adminKp2 = anchor.web3.Keypair.generate();
    let beneficiary = anchor.web3.Keypair.generate().publicKey;
    let project_id = new Uint8Array(Math.random())

    const [fundraiserPDA, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
        [utf8.encode("project"), project_id],
        program.programId
    );

    let name = "Sample Project";
    let admins = [
      adminKp1.publicKey,
      adminKp2.publicKey,
    ];

    let goal = 100;
  
    const tx = await program.methods.createProject(new anchor.BN(project_id), name, admins, beneficiary, goal)
    .accounts({
      payer: payer.publicKey,
      project: fundraiserPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();

    console.log("🚀 Project Created! Transaction Hash:", tx);
  });

  it("createPool", async () => {
    const payer = anchor.web3.Keypair.generate();
    let pool_id = new Uint8Array(Math.random())

    const [poolPDA, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
        [utf8.encode("pool"), pool_id],
        program.programId
    );

    let name = "Sample Project";
    let start = new anchor.BN(Math.floor(Date.now() / 1000));
    let end = new anchor.BN(Math.floor(Date.now() / 1000) + 30000);
  
    const tx = await program.methods.createPool(new anchor.BN(pool_id), name, start, end)
    .accounts({
      payer: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      pool: poolPDA,
    })
    .rpc();

    console.log("👾 Funding Round Initialized! Transaction Hash:", tx);
  });
});
