const { readFile } = require("mz/fs");
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

const initialize = (tracker, user, authority, counter, trackerProgramId) => {
  return new TransactionInstruction({
    keys: [
      {
        pubkey: tracker,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: user,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: authority,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: counter,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    data: Buffer.from(new Uint8Array([0])),
    programId: trackerProgramId,
  });
};

const increment = (tracker, user, authority, counter, counterProgramId, trackerProgramId) => {
  return new TransactionInstruction({
    keys: [
      {
        pubkey: tracker,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: user,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: counterProgramId,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: counter,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: authority,
        isSigner: false,
        isWritable: false,
      },
    ],
    data: Buffer.from(new Uint8Array([1])),
    programId: trackerProgramId,
  });
};

const main = async () => {
  var args = process.argv.slice(2);
  // args[0] (Optional): Counter buffer account
  const trackerProgramId = new PublicKey("CeAA9rP7xNJZWgrsPp84TSiQr32KGjh5VYvGSuYk7iSt");
  const counterProgramId = new PublicKey("EnDET6JzFbb9uAaskPmAde6ooYXzsiRM224ZJ7w2Qw6r");

  console.log("Tracker Program", trackerProgramId.toBase58());
  console.log("Counter Program", counterProgramId.toBase58());

  const connection = new Connection("https://api.devnet.solana.com/");
  let feePayer = new Keypair();

  const counter = new Keypair();
  let counterKey = counter.publicKey;

  let tx = new Transaction();
  if (args.length > 0) {
    console.log("Found counter address");
    counterKey = new PublicKey(args[0]);
  }

  if (args.length > 1) {
    secretKeyString = await readFile(args[1], {
      encoding: "utf8",
    });
    console.log("Loaded Keypair from ", args[1]);
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    feePayer = Keypair.fromSecretKey(secretKey);
  }

  if ((await connection.getBalance(feePayer.publicKey)) < 0.1) {
    console.log("Requesting Airdrop of 1 SOL...");
    await connection.requestAirdrop(feePayer.publicKey, 2e9);
    console.log("Airdrop received");
  }

  let signers = [feePayer];

  if (args.length == 0) {
    console.log("Generating new counter address");
    let createIx = SystemProgram.createAccount({
      fromPubkey: feePayer.publicKey,
      newAccountPubkey: counterKey,
      /** Amount of lamports to transfer to the created account */
      lamports: await connection.getMinimumBalanceForRentExemption(40),
      /** Amount of space in bytes to allocate to the created account */
      space: 40,
      /** Public key of the program to assign as the owner of the created account */
      programId: counterProgramId,
    });
    signers.push(counter);
    tx.add(createIx);
  }

  const trackerKey = (await PublicKey.findProgramAddress(
    [feePayer.publicKey.toBuffer(), counterKey.toBuffer()],
    trackerProgramId
  ))[0];
  const authKey = (await PublicKey.findProgramAddress(
    [counterKey.toBuffer()],
    trackerProgramId
  ))[0];

  let trackerData = await connection.getAccountInfo(trackerKey, "confirmed")
  if (!trackerData) {
    console.log("    -> No tracker account found. Creating new tracker account");
    const initializeIx = initialize(
      trackerKey,
      feePayer.publicKey,
      authKey,
      counterKey,
      trackerProgramId
    );
    tx.add(initializeIx);
  }

  console.log("Incrementing counter");
  const trackerInstruction = increment(
    trackerKey,
    feePayer.publicKey,
    authKey,
    counterKey,
    counterProgramId,
    trackerProgramId
  )
  tx.add(trackerInstruction);

  let txid = await sendAndConfirmTransaction(connection, tx, signers, {
    skipPreflight: true,
    preflightCommitment: "confirmed",
    commitment: "confirmed",
  });
  console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet`);

  let data = (await connection.getAccountInfo(counterKey, "confirmed")).data;
  const auth = new PublicKey(data.slice(0, 32));
  const globalCount = new BN(data.slice(32, 36), "le");
  console.log("Global:")
  console.log("   Counter Key:", counterKey.toBase58());
  console.log("   Counter Authority Key:", auth.toBase58());
  console.log("   Global Count: ", globalCount.toNumber());
  
  data = (await connection.getAccountInfo(trackerKey, "confirmed")).data;
  const trackerCount = new BN(data.slice(34, 42), "le");
  console.log("User:")
  console.log("   User Key:", feePayer.publicKey.toBase58());
  console.log("   User Count: ", trackerCount.toNumber());
};

main()
  .then(() => {
    console.log("Success");
  })
  .catch((e) => {
    console.error(e);
  });
