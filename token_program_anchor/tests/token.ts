import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Token } from "../target/types/token";

const logTx = async (provider, tx) => {
  await provider.connection.confirmTransaction(tx, "confirmed");
  console.log(
    (await provider.connection.getConfirmedTransaction(tx, "confirmed")).meta
      .logMessages
  );
};

describe("token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Token as Program<Token>;
  console.log(program.programId.toBase58());
  const mint_auth = anchor.web3.Keypair.generate();
  const user1 = anchor.web3.Keypair.generate();
  const user2 = anchor.web3.Keypair.generate();

  it("Initialize start state", async () => {
    // Airdropping tokens to a payer.
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        mint_auth.publicKey,
        10000000000
      ),
      "confirmed"
    );
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        user1.publicKey,
        10000000000
      ),
      "confirmed"
    );
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        user2.publicKey,
        10000000000
      ),
      "confirmed"
    );
  });

  it("Simple Test", async () => {
    // Add your test here.
    let mint = (await anchor.web3.PublicKey.findProgramAddress(
      [mint_auth.publicKey.toBuffer()],
      program.programId,
    ))[0];
    let txid = await program.rpc.initializeMint({
      accounts: {
        mint: mint,
        payer: mint_auth.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [mint_auth]
    });
    await logTx(program.provider, txid);
    let user1TA = (await anchor.web3.PublicKey.findProgramAddress(
      [user1.publicKey.toBuffer(), mint.toBuffer()], program.programId
    ))[0];
    txid = await program.rpc.initializeTokenAccount({
      accounts: {
        tokenAccount: user1TA,
        mint: mint,
        payer: user1.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [user1],
    });
    await logTx(program.provider, txid);
    let user2TA = (await anchor.web3.PublicKey.findProgramAddress(
      [user2.publicKey.toBuffer(), mint.toBuffer()], program.programId
    ))[0];
    txid = await program.rpc.initializeTokenAccount({
      accounts: {
        tokenAccount: user2TA,
        mint: mint,
        payer: user2.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [user2],
    });
    await logTx(program.provider, txid);

    txid = await program.rpc.mint(new anchor.BN(5), {
      accounts: {
        dst: user1TA,
        mint: mint,
        authority: mint_auth.publicKey,
      },
      signers: [mint_auth],
    });
    await logTx(program.provider, txid);

    txid = await program.rpc.mint(new anchor.BN(10), {
      accounts: {
        dst: user2TA,
        mint: mint,
        authority: mint_auth.publicKey,
      },
      signers: [mint_auth],
    });
    await logTx(program.provider, txid);

    txid = await program.rpc.transfer(new anchor.BN(7), {
      accounts: {
        src: user2TA,
        dst: user1TA,
        owner: user2.publicKey,
      },
      signers: [user2],
    });
    await logTx(program.provider, txid);

    txid = await program.rpc.burn(new anchor.BN(3), {
      accounts: {
        src: user2TA,
        mint: mint,
        owner: user2.publicKey,
      },
      signers: [user2],
    });
    await logTx(program.provider, txid);
  });
});
