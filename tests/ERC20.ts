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

let admin, alice, bob, charlie;
let aliceAccount, bobAccount, charlieAccount, aliceBobApprove;
let token, ercAccount, infoAccount, wrappedAccount;
let aliceToken;

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
    charlie = new anchor.web3.Keypair();
    await mock.transfer(admin, charlie.publicKey, 10000000);
    [charlieAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('createAccount'), charlie.publicKey.toBuffer()],
      program.programId,
    );
    [aliceBobApprove] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('approveAccount'), aliceAccount.toBuffer(), bob.publicKey.toBuffer()],
      program.programId,
    );

    token = await mock.createToken(6);
    [infoAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('accountInfo')],
      program.programId,
    );
    aliceToken = await token.getOrCreateAssociatedAccount(alice.publicKey);
    [wrappedAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('wrappedAccount')],
      program.programId,
    );

  });

  beforeEach(async () => {
    await mock.waitBlock();
  })

  it("Could initialize Info", async () => {
    let tx = await program.methods.initErc20("Token","TKN", 6)
      .accounts({
        user: admin.publicKey,
        info: infoAccount,
        mintAccount: token.mintAddress,
        vault: wrappedAccount,
      })
      .signers([admin])
      .rpc();
  });

  it("Could deposit", async () => {
    await token.mint(aliceToken.address, 1000);
    let tx = await program.methods.deposit(new BN(1000))
      .accounts({
        user: alice.publicKey,
        info: infoAccount,
        tokenAccount: aliceToken.address,
        vault: wrappedAccount,
      })
      .signers([alice])
      .rpc()
  });

  it("Could create account", async () => {
    let tx = await program.methods.createAccount()
      .accounts({
        user: alice.publicKey,
        account: aliceAccount,
      })
      .signers([alice])
      .rpc();
      tx = await program.methods.createAccount()
      .accounts({
        user: bob.publicKey,
        account: bobAccount,
      })
      .signers([bob])
      .rpc();
    tx = await program.methods.createAccount()
      .accounts({
        user: charlie.publicKey,
        account: charlieAccount,
      })
      .signers([charlie])
      .rpc();
  });

  it("Cant transfer with zero balance", async () => {
    let error;
    try {
      let tx = await program.methods.transfer(new BN(1000))
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

  it("Cant transfer with wrong authority", async () => {
    let error;
    try {
      let tx = await program.methods.transfer(new BN(1000))
      .accounts({
        user: bob.publicKey,
        account1: aliceAccount,
        account2: bobAccount,
      })
      .signers([bob])
      .rpc();
    } catch(err) {
      error = err;
      assert.isTrue(err instanceof AnchorError);
      assert.equal(err.error.errorMessage, "You are not authorised for this action");
    }
    assert.ok(error != null);
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

  it("Cant approve with wrong authority", async () => {
    let error;
    try {
      let tx = await program.methods.approve(aliceAccount, bob.publicKey, new BN(1000))
      .accounts({
        user: bob.publicKey,
        approveAccount: aliceBobApprove,
      })
      .signers([bob])
      .rpc();
    } catch(err) {
      error = err;
      assert.isTrue(err instanceof AnchorError);
      assert.equal(err.error.errorMessage, "You are not authorised for this action");
    }
    assert.ok(error != null);
  });

  it("Could approve", async () => {
    let tx = await program.methods.approve(aliceAccount, bob.publicKey, new BN(1000))
    .accounts({
      user: alice.publicKey,
      approveAccount: aliceBobApprove,
    })
    .signers([alice])
    .rpc();

  // transfer to avoid insufficient funds in the next test
  tx = await program.methods.transfer(new BN(1000))
    .accounts({
      user: bob.publicKey,
      account1: bobAccount,
      account2: aliceAccount,
    })
    .signers([bob])
    .rpc();

  });

  it("Could not transferFrom with wrong credentials", async () => {
  let error;
  try {
    let tx = await program.methods.transferFrom(new BN(1000))
      .accounts({
        user: charlie.publicKey,
        from: aliceAccount,
        to: charlieAccount,
        approveAccount: aliceBobApprove,
      })
      .signers([charlie])
      .rpc();
  } catch(err) {
      error = err;
      assert.isTrue(err instanceof AnchorError);
      assert.equal(err.error.errorMessage, "You are not authorised for this action");
  }
  assert.ok(error != null);
  });
  
  it("Could not transferFrom more than account funds", async () => {
    let tx = await program.methods.approve(aliceAccount, bob.publicKey, new BN(10000))
    .accounts({
      user: alice.publicKey,
      approveAccount: aliceBobApprove,
    })
    .signers([alice])
    .rpc();
    
    let error;
    try {
      let tx = await program.methods.transferFrom(new BN(10000))
        .accounts({
          user: bob.publicKey,
          from: aliceAccount,
          to: charlieAccount,
          approveAccount: aliceBobApprove,
        })
        .signers([bob])
        .rpc();
    } catch(err) {
        error = err;
        assert.isTrue(err instanceof AnchorError);
        assert.equal(err.error.errorMessage, "Account has insufficient balance");
    }
    assert.ok(error != null);
    });
  
    it("Could not transferFrom more than allowance limit", async () => {
      let tx = await program.methods.approve(aliceAccount, bob.publicKey, new BN(1000))
      .accounts({
        user: alice.publicKey,
        approveAccount: aliceBobApprove,
      })
      .signers([alice])
      .rpc();

    tx = await program.methods.mint(new BN(10000))
      .accounts({
        user: alice.publicKey,
        account: aliceAccount,
      })
      .signers([alice])
      .rpc();

      let error;
      try {
        let tx = await program.methods.transferFrom(new BN(10000))
          .accounts({
            user: bob.publicKey,
            from: aliceAccount,
            to: charlieAccount,
            approveAccount: aliceBobApprove,
          })
          .signers([bob])
          .rpc();
      } catch(err) {
          error = err;
          assert.isTrue(err instanceof AnchorError);
          assert.equal(err.error.errorMessage, "Yor are trying to transfer more than you allowed to");
      }
      assert.ok(error != null);
  });
  
  it("Could transfer from", async () => {
    let tx = await program.methods.transferFrom(new BN(1000))
    .accounts({
      user: bob.publicKey,
      from: aliceAccount,
      to: charlieAccount,
      approveAccount: aliceBobApprove,
    })
    .signers([bob])
    .rpc();
  });

});
