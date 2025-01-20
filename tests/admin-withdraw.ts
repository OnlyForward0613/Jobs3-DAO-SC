import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { expect, assert } from "chai";
import {
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddressSync,
  transfer,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  mintTo,
} from "@solana/spl-token";
import { GigBasicContract } from "../target/types/gig_basic_contract";
import { v4 as uuid } from "uuid";

describe("Admin Withdraw", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace
    .GigBasicContract as Program<GigBasicContract>;
  const wallet = anchor.Wallet.local();
  const signer = wallet.payer;
  let employer: anchor.web3.Keypair;
  let jobContract: anchor.web3.PublicKey;
  let employerAta: anchor.web3.PublicKey;
  let contractAta: anchor.web3.PublicKey;
  let PAY_TOKEN_MINT_ADDRESS: anchor.web3.PublicKey;
  let ASSOCIATED_TOKEN_PROGRAM_ID: anchor.web3.PublicKey;
  let contractId: string;

  before(async () => {
    console.log("------------- before -------------");
    // Create employer account
    employer = anchor.web3.Keypair.generate();

    // sol transfer
    const transaction = new anchor.web3.Transaction();

    transaction.add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: signer.publicKey,
        toPubkey: employer.publicKey,
        lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
      })
    );

    contractId = uuid().slice(0, 8);
    console.log("new contractId:", contractId);

    // Create job contract account
    [jobContract] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("gig_contract")),
        Buffer.from(anchor.utils.bytes.utf8.encode(contractId)),
      ],
      program.programId
    );

    console.log("jobContract: ", jobContract.toBase58());

    const txHash = await provider.connection.sendTransaction(transaction, [
      signer,
    ]);
    console.log("Transaction hash: ", txHash);

    const txConfirm = await provider.connection.confirmTransaction(txHash);
    console.log("Transaction hash confirm: ", txConfirm);

    const balance = await provider.connection.getBalance(signer.publicKey);
    const employerBalance = await provider.connection.getBalance(
      employer.publicKey
    );
    const jobContractBalance = await provider.connection.getBalance(
      jobContract
    );
    console.log(`
          balance: ${balance / anchor.web3.LAMPORTS_PER_SOL},
          employerBalance: ${employerBalance / anchor.web3.LAMPORTS_PER_SOL},
          jobContractBalance: ${
            jobContractBalance / anchor.web3.LAMPORTS_PER_SOL
          }
        `);

    PAY_TOKEN_MINT_ADDRESS = new anchor.web3.PublicKey(
      "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"
    );

    // Create the custom USDT-like token mint
    PAY_TOKEN_MINT_ADDRESS = await createMint(
      provider.connection,
      employer,
      employer.publicKey,
      null,
      6
    );

    console.log("New token mint: ", PAY_TOKEN_MINT_ADDRESS.toBase58());

    // Create associated token accounts for employer and contract
    // Create associated token accounts for employer and contract
    employerAta = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        employer,
        PAY_TOKEN_MINT_ADDRESS,
        signer.publicKey
      )
    ).address; // Access the address property

    console.log("employerAta: ", employerAta.toBase58());

    contractAta = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        employer,
        PAY_TOKEN_MINT_ADDRESS,
        jobContract,
        true
      )
    ).address; // Access the address property

    console.log("contractAta: ", contractAta.toBase58());

    await mintTo(
      provider.connection,
      employer,
      PAY_TOKEN_MINT_ADDRESS,
      employerAta,
      employer.publicKey,
      1_000_000_000
    );

    const tokenBalance = await provider.connection.getTokenAccountBalance(
      employerAta
    );
    console.log("Token balance: ", tokenBalance);

    // Initialize the job contract and associated token accounts as needed
    // (You would need to implement this part based on your contract's logic)
  });

  it("should create a job listing", async () => {
    console.log("------------- creating a job listing -------------");
  });

  it("Should list a job with a $1 fee on the employer side", async () => {
    // Call the job_listing_with_one_fee_employer function
    const tx = await program.methods
      .jobListingWithOneFeeEmployer(contractId)
      .accounts({
        employer: signer.publicKey,
        jobContract: jobContract,
        employerAta: employerAta,
        contractAta: contractAta,
      })
      .signers([signer])
      .rpc();

    console.log("Transaction signature", tx);

    // Add assertions to check the state after the transaction
    // For example, check if the job contract status is updated
    const updatedJobContract = await program.account.jobContract.fetch(
      jobContract
    );

    const statusKeys = Object.keys(updatedJobContract.status);
    console.log("statusKeys: ", statusKeys);

    const tokenBalance = await provider.connection.getTokenAccountBalance(
        employerAta
      );
      console.log("Token balance: ", tokenBalance);

    assert.equal(
      statusKeys[0],
      "created",
      "Job contract status should be 'Listed'"
    );
  });

  it("have to withdraw the admin fee", async () => {
    const balance = await provider.connection.getTokenAccountBalance(contractAta);
    console.log("Contract token balance: ", contractAta.toBase58(), "\n", balance);
    

    const withdrawATA = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        signer,
        PAY_TOKEN_MINT_ADDRESS,
        signer.publicKey
    );

    const withdrawAddressBalance = await provider.connection.getTokenAccountBalance(withdrawATA.address);
    console.log("Withdraw address balance: ", withdrawATA.address.toBase58(), "\n", contractId, withdrawAddressBalance);





    const tx = await program.methods
      .adminWithdrawJobContract(contractId)
      .accounts({
        contract: jobContract,
        contractAta: contractAta,
        admin: signer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        withdrawAddress: withdrawATA.address,
        payTokenMint: PAY_TOKEN_MINT_ADDRESS,
      })
      .signers([signer])
      .rpc();

    console.log("Transaction signature", tx);
  });

//   it("Should list a job with a featured fee on the employer side", async () => {
//     console.log("------------- creating a featured job listing -------------");

//     const featuredDay = 3; // Example: listing for 3 days
//     const expectedFee = 36_000_000; // Expected fee for 3 days

//     // Fetch the initial token balance of the employer
//     const initialEmployerTokenBalance =
//       await provider.connection.getTokenAccountBalance(employerAta);
//     console.log(
//       "Initial Employer Token Balance: ",
//       initialEmployerTokenBalance.value.amount
//     );

//     // Fetch the initial token balance of the contract
//     const initialContractTokenBalance =
//       await provider.connection.getTokenAccountBalance(contractAta);
//     console.log(
//       "Initial Contract Token Balance: ",
//       initialContractTokenBalance.value.amount
//     );

//     // Call the job_listing_with_feature_employer function
//     const tx = await program.methods
//       .jobListingWithFeatureEmployer(contractId, featuredDay)
//       .accounts({
//         employer: employer.publicKey,
//         jobContract: jobContract,
//         employerAta: employerAta,
//         contractAta: contractAta,
//         tokenProgram: TOKEN_PROGRAM_ID,
//         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([employer])
//       .rpc();

//     console.log("Transaction signature", tx);

//     // Fetch the updated job contract
//     const updatedJobContract = await program.account.jobContract.fetch(
//       jobContract
//     );
//     console.log("updatedJobContract: ", updatedJobContract);

//     // Check if the employer's token balance is reduced by the listing fee
//     const employerTokenBalance =
//       await provider.connection.getTokenAccountBalance(employerAta);
//     assert.equal(
//       employerTokenBalance.value.amount,
//       (1_000_000_000 - expectedFee).toString(),
//       "Employer's token balance should reflect the listing fee deduction"
//     );

//     // Fetch the updated token balance of the employer after job posting
//     const updatedEmployerTokenBalance =
//       await provider.connection.getTokenAccountBalance(employerAta);
//     console.log(
//       "Updated Employer Token Balance: ",
//       updatedEmployerTokenBalance.value.amount
//     );

//     // Fetch the updated token balance of the contract after job posting
//     const updatedContractTokenBalance =
//       await provider.connection.getTokenAccountBalance(contractAta);
//     console.log(
//       "Updated Contract Token Balance: ",
//       updatedContractTokenBalance.value.amount
//     );

//     console.log("Featured job listing created successfully!");
//   });
});
