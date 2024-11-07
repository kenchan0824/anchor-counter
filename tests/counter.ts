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

});