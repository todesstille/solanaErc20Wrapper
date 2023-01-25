import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BN } from "bn.js";
import { Erc20 } from "../target/types/erc20";
import { Mock } from "@todesstille/mocksolana"
import { assert } from "chai";
import { AnchorError } from "@project-serum/anchor";

const mock = new Mock(anchor);
const provider = mock.getProvider();
const program = anchor.workspace.Erc20 as Program<Erc20>;

let admin, alice, bob;
let aliceAccount, bobAccount;

describe("ERC20", () => {
  
  before(async () => {
    admin = provider.wallet.payer;
    alice = new anchor.web3.Keypair();
    await mock.transfer(admin, alice.publicKey, 10000000);
    [aliceAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('createAccount'), alice.publicKey.toBuffer()],
      program.programId,
    );
    bob = new anchor.web3.Keypair();
    await mock.transfer(admin, bob.publicKey, 10000000);
    [bobAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('createAccount'), bob.publicKey.toBuffer()],
      program.programId,
    );
  });

  beforeEach(async () => {
    await mock.waitBlock();
  })

  it("Could create account", async () => {
    const tx = await program.methods.createAccount()
      .accounts({
        user: alice.publicKey,
        account: aliceAccount,
      })
      .signers([alice])
      .rpc();
  });

  it("Cant transfer with zero balance", async () => {
    let tx = await program.methods.createAccount()
      .accounts({
        user: bob.publicKey,
        account: bobAccount,
      })
      .signers([bob])
      .rpc();
    await mock.waitBlock();
    let error;
    try {
      tx = await program.methods.transfer(new BN(1000))
        .accounts({
          user: alice.publicKey,
          account1: aliceAccount,
          account2: bobAccount,
        })
        .signers([alice])
        .rpc();
      } catch (err) {
        error = err;
        assert.isTrue(err instanceof AnchorError);
        assert.equal(err.error.errorMessage, "Account has insufficient balance");
      }
      assert.ok(error != null);
  });

  it("Assets could be minted", async () => {
    const tx = await program.methods.mint(new BN(1000))
      .accounts({
        user: alice.publicKey,
        account: aliceAccount,
      })
      .signers([alice])
      .rpc();
  });

  it("Could transfer", async () => {
    let tx = await program.methods.transfer(new BN(1000))
    .accounts({
      user: alice.publicKey,
      account1: aliceAccount,
      account2: bobAccount,
    })
    .signers([alice])
    .rpc();
});

});
