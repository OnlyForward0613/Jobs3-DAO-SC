import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GigContract } from "../target/types/gig_contract";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  createMint,
} from "@solana/spl-token";
import { Keypair, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("gig-contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.GigContract as Program<GigContract>;

  // Constants
  const CONTRACT_SEED = "job-contract";

  // Accounts
  let employer: Keypair;
  let jobContract: Keypair;
  let employerAta: PublicKey;
  let contractAta: PublicKey;
  let employerReferral: Keypair;
  let employerReferralAta: PublicKey;
  let contractId: string;
  let payTokenMint: PublicKey; // Custom token mint

  before(async () => {
    employer = Keypair.generate();
    employerReferral = Keypair.generate();
    jobContract = Keypair.generate();
    contractId = `contract-${Date.now()}`; // Unique contract ID

    // Airdrop SOL to employer
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        employer.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      ),
      "confirmed"
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        employerReferral.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      ),
      "confirmed"
    );

    // Create a custom token mint
    payTokenMint = await createMint(
      provider.connection,
      employer, // Payer
      employer.publicKey, // Mint authority
      null, // Freeze authority (optional)
      6, // Decimals
      TOKEN_PROGRAM_ID
    );
    console.log("Token Mint:", payTokenMint.toString());

    // Derive associated token accounts
    employerAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      employer,
      payTokenMint,
      employer.publicKey
    ).then(ata => ata.address);

    contractAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      employer,
      payTokenMint,
      provider.wallet.publicKey // Using provider's wallet as contract authority for simplicity
    ).then(ata => ata.address);

    employerReferralAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      employerReferral,
      payTokenMint,
      employerReferral.publicKey
    ).then(ata => ata.address);


    // Mint tokens to employer's ATA
    await mintTo(
      provider.connection,
      employer,
      payTokenMint,
      employerAta,
      provider.wallet.publicKey,
      1_000_000_000 // 1,000 tokens
    );

    console.log("Employer ATA", employerAta.toString());
    console.log("Contract ATA", contractAta.toString());
    console.log("Employer Referral ATA", employerReferralAta.toString());
  });

  it("Job Listing with Featured Employer (with referral)", async () => {
    const featuredDay = 1;
    const listingFee = 21_000_000; //For 1 day
    const referralAmount = (listingFee * 10) / 100;
    const contractAmount = (listingFee * 90) / 100;

    const [jobContractPDA, _jobContractBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [anchor.utils.bytes.utf8.encode(CONTRACT_SEED),
        anchor.utils.bytes.utf8.encode(contractId)],
        program.programId
      );

    console.log("jobContractPDA", jobContractPDA.toString());

    try {
      const tx = await program.methods
        .jobListingWithFeatureEmployer(contractId, featuredDay)
        .accounts({
          employer: employer.publicKey,
          jobContract: jobContractPDA,
          employerAta: employerAta,
          contractAta: contractAta,
          employerReferralAta: employerReferralAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([employer])
        .rpc();

      console.log("Your transaction signature", tx);

      // Fetch employer referral ATA balance and assert
      const employerReferralAtaAccount = await provider.connection.getAccountInfo(employerReferralAta);
      const employerReferralBalance = anchor.utils.bytes.u64.decode(employerReferralAtaAccount.data.slice(64, 72));
      assert.equal(employerReferralBalance, referralAmount, "Referral amount not transferred correctly");

      // Fetch contract ATA balance and assert
      const contractAtaAccount = await provider.connection.getAccountInfo(contractAta);
      const contractAtaBalance = anchor.utils.bytes.u64.decode(contractAtaAccount.data.slice(64, 72));
      assert.equal(contractAtaBalance, contractAmount, "Contract amount not transferred correctly");


      // Fetch job contract account and assert
      const jobContractAccount = await program.account.jobContract.fetch(jobContractPDA);
      assert.ok(jobContractAccount.contractId === contractId, "Contract ID mismatch");
      assert.ok(jobContractAccount.status.created === true, "Status is not Created");
      assert.ok(jobContractAccount.featured === true, "Job is not featured");
      assert.ok(jobContractAccount.featuredDay === featuredDay, "Featured day mismatch");
      assert.ok(jobContractAccount.employerReferral.equals(employerReferral.publicKey), "Employer referral mismatch");
      console.log("jobContractAccount", jobContractAccount);

    } catch (error) {
      console.log("Error: ", error);
      assert.fail(error);
    }
  });

  it("Job Listing with Featured Employer (without referral)", async () => {
    const featuredDay = 7;
    const listingFee = 71_000_000; // For 7 days
    const contractIdNoReferral = `contract-${Date.now() + 1}`;

    const [jobContractPDANoReferral, _jobContractBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [anchor.utils.bytes.utf8.encode(CONTRACT_SEED),
        anchor.utils.bytes.utf8.encode(contractIdNoReferral)],
        program.programId
      );

    try {
      const tx = await program.methods
        .jobListingWithFeatureEmployer(contractIdNoReferral, featuredDay)
        .accounts({
          employer: employer.publicKey,
          jobContract: jobContractPDANoReferral,
          employerAta: employerAta,
          contractAta: contractAta,
          employerReferralAta: null, // No referral
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([employer])
        .rpc();

      console.log("Your transaction signature", tx);

      // Fetch contract ATA balance and assert
      const contractAtaAccount = await provider.connection.getAccountInfo(contractAta);
      const contractAtaBalance = anchor.utils.bytes.u64.decode(contractAtaAccount.data.slice(64, 72));

      // initial balance from previous test + full listing fee
      assert.equal(contractAtaBalance, listingFee + 63900000, "Contract amount not transferred correctly");

      // Fetch job contract account and assert
      const jobContractAccount = await program.account.jobContract.fetch(jobContractPDANoReferral);
      assert.ok(jobContractAccount.contractId === contractIdNoReferral, "Contract ID mismatch");
      assert.ok(jobContractAccount.status.created === true, "Status is not Created");
      assert.ok(jobContractAccount.featured === true, "Job is not featured");
      assert.ok(jobContractAccount.featuredDay === featuredDay, "Featured day mismatch");
      assert.ok(jobContractAccount.employerReferral.equals(anchor.web3.SystemProgram.programId), "Employer referral should be default value");

    } catch (error) {
      console.log("Error: ", error);
      assert.fail(error);
    }
  });

  it("Job Listing with Featured Employer (excluded referral)", async () => {
    const featuredDay = 30;
    const listingFee = 150_000_000; // For 30 days
    const contractIdExcludedReferral = `contract-${Date.now() + 2}`;

    const [jobContractPDAExcludedReferral, _jobContractBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [anchor.utils.bytes.utf8.encode(CONTRACT_SEED),
        anchor.utils.bytes.utf8.encode(contractIdExcludedReferral)],
        program.programId
      );

    // Generate a new keypair for the excluded referral ATA, and fund it with SOL
    const excludedReferral = Keypair.generate();
    await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(
            excludedReferral.publicKey,
            2 * anchor.web3.LAMPORTS_PER_SOL
        ),
        "confirmed"
    );

    // Create the associated token account for the excluded referral
    const excludedReferralAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        excludedReferral,
        payTokenMint,
        excludedReferral.publicKey
    ).then(ata => ata.address);

    try {
      const tx = await program.methods
        .jobListingWithFeatureEmployer(contractIdExcludedReferral, featuredDay)
        .accounts({
          employer: employer.publicKey,
          jobContract: jobContractPDAExcludedReferral,
          employerAta: employerAta,
          contractAta: contractAta,
          employerReferralAta: excludedReferralAta, // No referral
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([employer])
        .rpc();

      console.log("Your transaction signature", tx);

      // Fetch contract ATA balance and assert
      const contractAtaAccount = await provider.connection.getAccountInfo(contractAta);
      const contractAtaBalance = anchor.utils.bytes.u64.decode(contractAtaAccount.data.slice(64, 72));

      // initial balance from previous test + full listing fee
      assert.equal(contractAtaBalance, listingFee + 221900000, "Contract amount not transferred correctly");

      // Fetch job contract account and assert
      const jobContractAccount = await program.account.jobContract.fetch(jobContractPDAExcludedReferral);
      assert.ok(jobContractAccount.contractId === contractIdExcludedReferral, "Contract ID mismatch");
      assert.ok(jobContractAccount.status.created === true, "Status is not Created");
      assert.ok(jobContractAccount.featured === true, "Job is not featured");
      assert.ok(jobContractAccount.featuredDay === featuredDay, "Featured day mismatch");
      assert.ok(jobContractAccount.employerReferral.equals(anchor.web3.SystemProgram.programId), "Employer referral should be default value");

    } catch (error) {
      console.log("Error: ", error);
      assert.fail(error);
    }
  });

  it("Invalid featured day", async () => {
    const invalidFeaturedDay = 99;
    const contractIdInvalidDay = `contract-${Date.now() + 3}`;

    const [jobContractPDAInvalidDay, _jobContractBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [anchor.utils.bytes.utf8.encode(CONTRACT_SEED),
        anchor.utils.bytes.utf8.encode(contractIdInvalidDay)],
        program.programId
      );

    try {
       await program.methods
        .jobListingWithFeatureEmployer(contractIdInvalidDay, invalidFeaturedDay)
        .accounts({
          employer: employer.publicKey,
          jobContract: jobContractPDAInvalidDay,
          employerAta: employerAta,
          contractAta: contractAta,
          employerReferralAta: null,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([employer])
        .rpc();

        assert.fail("Should have thrown an error due to invalid featured day");

    } catch (error) {
      console.log("Error: ", error);
      // Check if the error message contains the expected error
      assert.ok(error.message.includes("InvalidFeaturedDay"), "Error message does not match");
    }
  });
});
