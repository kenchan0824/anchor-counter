import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN} from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
const assert = require("assert"); 

describe("Anchor Counter Program", () => {

  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;
  const owner = provider.wallet.publicKey;
  const [counterPK, bump] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("counter"), owner.toBuffer()],
    program.programId
  )

  it("counter account is initialized properly", async () => {
    await program.methods.initCounter()
      .accounts({
        owner,
        counter: counterPK
      })
      .rpc();

    const counter = await program.account.counter.fetch(counterPK);
    assert.ok(counter.owner.toBase58() === owner.toBase58());
    assert.ok(counter.count.toNumber() === 0);
  });

  it("update() should increase counter by 1", async () => {
    let counter = await program.account.counter.fetch(counterPK);
    const initCount = counter.count.toNumber()

    await program.methods.update()
      .accounts({
        owner,
        counter: counterPK
      })
      .rpc();

    counter = await program.account.counter.fetch(counterPK);
    assert.ok(counter.count.toNumber() === initCount + 1);
  });

  it("update() should only handle owner's counter", async () => {
    const hacker = web3.Keypair.generate();
    let passed = false;

    try {
      await program.methods.update()
      .accounts({
        owner : hacker.publicKey,
        counter: counterPK
      })
      .signers([hacker])
      .rpc();

      passed = true;
    } catch {}
    
    return assert.ok(!passed);
  });
  
});