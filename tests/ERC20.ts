import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BN } from "bn.js";
import { Erc20 } from "../target/types/erc20";
import { Mock } from "@todesstille/mocksolana"

const mock = new Mock(anchor);
const provider = mock.getProvider();
const program = anchor.workspace.Erc20 as Program<Erc20>;

let admin, alice, bob;
let aliceAccount;

describe("ERC20", () => {
  
  before(async () => {
    admin = provider.wallet.payer;
    alice = new anchor.web3.Keypair();
    await mock.transfer(admin, alice.publicKey, 10000000);
    [aliceAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('createAccount'), alice.publicKey.toBuffer()],
      program.programId,
    );

  });

  it("Assets could be minted", async () => {
    const tx = await program.methods.mint(new BN(123))
      .accounts({
        user: alice.publicKey,
        account: aliceAccount,
      })
      .signers([alice])
      .rpc();
  });
});
