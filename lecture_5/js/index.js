const {
  Connection,
  sendAndConfirmTransaction,
  Keypair,
  Transaction,
  SystemProgram,
  PublicKey,
  TransactionInstruction,
} = require("@solana/web3.js");

const BN = require("bn.js");

const MARKETPLACE_SIZE = 32 + 256 * 80;

const stack = (buffer, user, programId) => {
  const idx = Buffer.from(new Uint8Array([0]));
  return new TransactionInstruction({
    keys: [
      {
        pubkey: buffer,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: user,
        isSigner: true,
        isWritable: false,
      },
    ],
    data: Buffer.concat([idx]),
    programId: programId,
  });
};

const zc = (buffer, user, programId) => {
  const idx = Buffer.from(new Uint8Array([3]));
  return new TransactionInstruction({
    keys: [
      {
        pubkey: buffer,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: user,
        isSigner: true,
        isWritable: false,
      },
    ],
    data: Buffer.concat([idx]),
    programId: programId,
  });
};

const echo = (buffer, size, programId) => {
  const idx = Buffer.from(new Uint8Array([4]));
  const len = Buffer.from(new Uint8Array(new BN(size).toArray("le", 4)));
  let content = [];
  for (let i = 0; i < size; i += 1) {
    content.push(Math.floor(32 + Math.random() * 95));
  }
  content = Buffer.from(new Uint8Array(content));
  return new TransactionInstruction({
    keys: [
      {
        pubkey: buffer,
        isSigner: false,
        isWritable: true,
      },
    ],
    data: Buffer.concat([idx, len, content]),
    programId: programId,
  });
};

const runtime = (i, programId) => {
  const iters = Buffer.from(new Uint8Array(new BN(i).toArray("le", 8)));
  return new TransactionInstruction({
    keys: [],
    data: Buffer.concat([Buffer.from(new Uint8Array([1])), iters]),
    programId: programId,
  });
};

const cpi = (from, to, s, programId) => {
  const size = Buffer.from(new Uint8Array(new BN(s).toArray("le", 8)));
  return new TransactionInstruction({
    keys: [
      {
        pubkey: from,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: to,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    data: Buffer.concat([Buffer.from(new Uint8Array([2])), size]),
    programId: programId,
  });
};

const main = async () => {
  var args = process.argv.slice(2);
  const programId = new PublicKey(
    "2MZcvPeZv8C3H9VxkByexUJpGwnJgkWgyYsC5bQorJhL"
  );

  console.log("Program: ", programId.toBase58());

  const connection = new Connection("http://127.0.0.1:8899");
  let feePayer = new Keypair();

  let tx = new Transaction();

  if ((await connection.getBalance(feePayer.publicKey)) < 0.1) {
    console.log("Requesting Airdrop of 1 SOL...");
    await connection.requestAirdrop(feePayer.publicKey, 2e9);
    console.log("Airdrop received");
  }

  let signers = [feePayer];
  const to = new Keypair();

  const ix = parseInt(args[0]);

  if (ix == 0) {
    let createIx = SystemProgram.createAccount({
      fromPubkey: feePayer.publicKey,
      newAccountPubkey: to.publicKey,
      /** Amount of lamports to transfer to the created account */
      lamports: await connection.getMinimumBalanceForRentExemption(
        MARKETPLACE_SIZE 
      ),
      /** Amount of space in bytes to allocate to the created account */
      space: MARKETPLACE_SIZE,
      /** Public key of the program to assign as the owner of the created account */
      programId: programId,
    });
    signers.push(to);
    tx.add(createIx);
    const stackIx = stack(to.publicKey, feePayer.publicKey, programId);
    tx.add(stackIx);
  } else if (ix == 1) {
    const runtimeIx = runtime(parseInt(args[1]), programId);
    tx.add(runtimeIx);
  } else if (ix == 2) {
    const cpiIx = cpi(
      feePayer.publicKey,
      to.publicKey,
      parseInt(args[1]),
      programId
    );
    signers.push(to);
    tx.add(cpiIx);
  } else if (ix == 3) {
    let createIx = SystemProgram.createAccount({
      fromPubkey: feePayer.publicKey,
      newAccountPubkey: to.publicKey,
      /** Amount of lamports to transfer to the created account */
      lamports: await connection.getMinimumBalanceForRentExemption(
        MARKETPLACE_SIZE 
      ),
      /** Amount of space in bytes to allocate to the created account */
      space: MARKETPLACE_SIZE,
      /** Public key of the program to assign as the owner of the created account */
      programId: programId,
    });
    signers.push(to);
    tx.add(createIx);
    const zcIx = zc(to.publicKey, feePayer.publicKey, programId);
    tx.add(zcIx);
  } else if (ix == 4) {
    let size = parseInt(args[1]);
    let createIx = SystemProgram.createAccount({
      fromPubkey: feePayer.publicKey,
      newAccountPubkey: to.publicKey,
      /** Amount of lamports to transfer to the created account */
      lamports: await connection.getMinimumBalanceForRentExemption(size),
      /** Amount of space in bytes to allocate to the created account */
      space: size,
      /** Public key of the program to assign as the owner of the created account */
      programId: programId,
    });
    signers.push(to);
    tx.add(createIx);
    const echoIx = echo(to.publicKey, size, programId);
    tx.add(echoIx);
  } else {
    throw Error;
  }

  let txid = await sendAndConfirmTransaction(connection, tx, signers, {
    skipPreflight: true,
    preflightCommitment: "confirmed",
    confirmation: "confirmed",
    commitment: "confirmed",
  });
  console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet`);
};

main()
  .then(() => {
    console.log("Success");
  })
  .catch((e) => {
    console.error(e);
  });
