import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { expect, assert  } from "chai";
import {
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddressSync,
  transfer,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { GigBasicContract } from "../target/types/gig_basic_contract";
import bs58 from "bs58";
import { v4 as uuid } from "uuid";
import secret from "/home/rrr/.config/solana/id.json";

describe("gig-basic-contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace
    .GigBasicContract as Program<GigBasicContract>;

  const connection = provider.connection;

  const CONTRACT_SEED = "gig_contract";
  const authKp = Keypair.fromSecretKey(new Uint8Array(secret));

  // const firstKp = new Keypair();
  // const secondKp = new Keypair();
  const firstKp = Keypair.fromSecretKey(
    Uint8Array.from(
      bs58.decode(
        "4bnMLLijWtwRMGanFqTtBpMmZ6u5SngacrjmeALYZikLCq5gTzZpvKY8jyh7y9GWfoMG8hUccCrwHytkwy1imYXh"
      )
    )
  ); // 1sol-account
  const secondKp = Keypair.fromSecretKey(
    Uint8Array.from(
      bs58.decode(
        "5sspNdxSSRfpj5pSoZULYmrYdMhuYn2vqN4ZzwLiJKLUAuyKmsf8J9ueA8o7FMR5Dr2dUtZ8sdpp6cSEHVgnfepq"
      )
    )
  ); // Test Account

  let payTokenMint = new PublicKey(
    "7FctSfSZ9GonfMrybp45hzoQyU71CEjjZFxxoSzqKWT"
  ); // BPT mint address

  let decimal = 8;

  // let contractAddress: any;
  let contractAddress = new PublicKey(
    "H65BKYrJMz4mK5dmT7GGVhKTAk6Va8wDuZgBRV4UKQ95"
  );
  // let contractBump: any;
  let contractBump = 254;

  let authAta: any;
  let firstAta: any;
  let secondAta: any;
  let contractAta: any;
  let contractId: any = "a4fc3529";

  let buyerReferral = new PublicKey(
    "At1PNJxjhr9NrThvDGCkNA17Q57ceWZqXe6pdbYb15T5"
  );

  let sellerReferral = new PublicKey(
    "E2kkuEFZktytswKCYYsXyKx483vfM2qqj1zPQDer4djg"
  );

  let buyerReferralAta: any;
  let sellerReferralAta: any;

  // it("[Success] Fetch all contracts!", async () => {
  //   try {
  //     // Fetch the pool account and assert the values
  //     const allContractAccount = await program.account.contract.all();

  //     console.log("allContractAccount:", allContractAccount);

  //   } catch (error) {
  //     console.log("Error while fetching all pools:", error);
  //   }
  // });

  it("[Success] Fetch all hourly contracts!", async () => {
    try {
      // Fetch the pool account and assert the values
      const allContractAccount = await program.account.hourlyContract.all();

      console.log("all hourly ContractAccount:", allContractAccount);

    } catch (error) {
      console.log("Error while fetching all hourly pools:", error);
    }
  });

  it("[Success] Set Up!", async () => {
    try {
      authAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        authKp,
        payTokenMint,
        authKp.publicKey
      );

      contractAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        authKp,
        payTokenMint,
        contractAddress,
        true
      );

      console.log("contractAta", contractAta);

      // await transfer(
      //   program.provider.connection,
      //   authKp,
      //   authAta.address,
      //   treasuryAta.address,
      //   authKp.publicKey,
      //   treasuryAmount
      // );

      firstAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        authKp,
        payTokenMint,
        firstKp.publicKey
      );

      // await transfer(
      //   program.provider.connection,
      //   authKp,
      //   authAta.address,
      //   firstAta.address,
      //   authKp.publicKey,
      //   firstAmount
      // );

      secondAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        secondKp,
        payTokenMint,
        secondKp.publicKey
      );

      buyerReferralAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        authKp,
        payTokenMint,
        buyerReferral
      );

      sellerReferralAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        authKp,
        payTokenMint,
        sellerReferral
      );

      // await transfer(
      //   program.provider.connection,
      //   authKp,
      //   authAta.address,
      //   secondAta.address,
      //   authKp.publicKey,
      //   secondAmount
      // );

      const authTokenAccount =
        await program.provider.connection.getTokenAccountBalance(
          authAta.address
        );

      console.log("authTokenAccount", authTokenAccount.value.amount);

      const firstTokenAccount =
        await program.provider.connection.getTokenAccountBalance(
          firstAta.address
        );

      console.log("firstTokenAccount", firstTokenAccount.value.amount);

      const secondTokenAccount =
        await program.provider.connection.getTokenAccountBalance(
          secondAta.address
        );

      console.log("secondTokenAccount", secondTokenAccount.value.amount);
    } catch (error) {
      console.log("Error while setting up:", error);
    }
  });

  // // Testing for all satisfied case
  // it("1-[Failure-Invalid dispute amount] Create a new contract!", async () => {
  //   try {
  //     // Create a new uuid to use as a new contract id
  //     contractId = uuid().slice(0, 8);
  //     console.log("new contractId:", contractId);

  //     const amount = new anchor.BN(10 * Math.pow(10, decimal)); // 10 BPT token; // 10 USDC
  //     const dispute = new anchor.BN(0.4 * Math.pow(10, decimal)); // 0.4 BPT token; // 0.4 USDC shoulb be 50 cent
  //     const deadline = Math.floor(Date.now() / 1000) + (10 * 24 * 60 * 60); // 10 days in seconds from Current timestamp

  //     const [contract, bump] = anchor.web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.from(anchor.utils.bytes.utf8.encode(CONTRACT_SEED)),
  //         Buffer.from(anchor.utils.bytes.utf8.encode(contractId)),
  //       ],
  //       program.programId
  //     );


  //     contractAddress = contract;
  //     contractBump = bump;
  //     console.log("contractAddress", contractAddress);
  //     console.log("contractBump", contractBump);

  //     contractAta = await getOrCreateAssociatedTokenAccount(
  //       program.provider.connection,
  //       authKp,
  //       payTokenMint,
  //       contractAddress,
  //       true
  //     );

  //     // Call startContract function
  //     const tx = await program.methods
  //       .startContract(
  //         contractId,
  //         amount, 
  //         dispute, 
  //         deadline,
          
  //       )
  //       .accounts({
  //         buyer: firstKp.publicKey,
  //         contract,
  //         seller: secondKp.publicKey,
  //         buyerAta: firstAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         rent: SYSVAR_RENT_PUBKEY,
  //       })

  //       .signers([firstKp])
  //       .rpc();
  //       // .rpc({ skipPreflight: true });


  //     console.log("Your transaction signature for creating a new contract", tx);

  //     // Fetch the contract account and assert the values
  //     const contractAccount = await program.account.contract.fetch(contract);

  //     console.log("new contract account:", contractAccount);

  //   } catch (error) {
  //     // console.log("Error while creating a new contract:", error);
  //     assert.equal(error.error.errorCode.code, "InvalidDisputeAmount");
  //     assert.equal(error.error.errorCode.number, 6004);
  //     assert.equal(error.error.errorMessage, "Dispute Amount should be 50 cent!");
  //   }
  // });
  ///===========================//
  // it("1-[Success] Create a new contract on buyer side!", async () => {
  //   try {
  //     // Create a new uuid to use as a new contract id
  //     contractId = uuid().slice(0, 8);
  //     console.log("new contractId:", contractId);

  //     const amount = new anchor.BN(10 * Math.pow(10, decimal)); // 10 BPT token; // 10 USDC
  //     const dispute = new anchor.BN(0.5 * Math.pow(10, decimal)); // 0.5 BPT token; // 0.5 USDC 50 cent
  //     const deadline = Math.floor(Date.now() / 1000) + (10 * 24 * 60 * 60); // 10 days in seconds from Current timestamp

  //     const [contract, bump] = anchor.web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.from(anchor.utils.bytes.utf8.encode(CONTRACT_SEED)),
  //         Buffer.from(anchor.utils.bytes.utf8.encode(contractId)),
  //       ],
  //       program.programId
  //     );


  //     contractAddress = contract;
  //     contractBump = bump;
  //     console.log("contractAddress", contractAddress);
  //     console.log("contractBump", contractBump);

  //     contractAta = await getOrCreateAssociatedTokenAccount(
  //       program.provider.connection,
  //       authKp,
  //       payTokenMint,
  //       contractAddress,
  //       true
  //     );

  //     // Call startContract function
  //     const tx = await program.methods
  //       .startContractOnBuyer(
  //         contractId,
  //         amount, 
  //         dispute, 
  //         deadline,
  //       )
  //       .accounts({
  //         buyer: firstKp.publicKey,
  //         contract,
  //         seller: secondKp.publicKey,
  //         buyerReferral,
  //         buyerAta: firstAta.address,
  //         payTokenMint,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         rent: SYSVAR_RENT_PUBKEY,
  //       })

  //       .signers([firstKp])
  //       .rpc();
  //       // .rpc({ skipPreflight: true });


  //     console.log("Your transaction signature for creating a new contract", tx);

  //     // Fetch the contract account and assert the values
  //     const contractAccount = await program.account.contract.fetch(contract);

  //     console.log("new contract account:", contractAccount);

  //   } catch (error) {
  //     console.log("Error while creating a new contract:", error);
  //   }
  // });

  ///===========================//
  // it("1-[Success] Activate the contract!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before activating:", contractAccount);

  //     // Call the buy_tickets function
  //     const tx = await program.methods
  //       .activateContract(contractId, true)
  //       .accounts({
  //         contract: contractAddress,
  //         seller: secondKp.publicKey,
  //         sellerAta: secondAta.address,
  //         sellerReferral: null,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([secondKp])
  //       .rpc({ skipPreflight: true });

  //     console.log("Your transaction signature for activating the contract:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after activating:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while activating contract!:", error);
  //   }
  // });

  // it("1-[Failure-invalid buyer] Approve by buyer(client)!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on buyer!:", contractAccount);

  //     const tx = await program.methods
  //       .buyerApprove(contractId, false)
  //       .accounts({
  //         contract: contractAddress,
  //         buyer: secondKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([secondKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on buyer:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on buyer!:", contractAccount);
  //   } catch (error) {
  //     // console.log("Error while approving on buyer!:", error);
  //     assert.equal(error.error.errorCode.code, "InvalidBuyer");
  //     assert.equal(error.error.errorCode.number, 6002);
  //     assert.equal(error.error.errorMessage, "Invalid buyer is trying to release funds!");
  //   }
  // });

  ///===========================//
  // it("1-[Success-Satisfied(no split)] Approve by buyer(client)!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on buyer!:", contractAccount);

  //     const tx = await program.methods
  //       .buyerApprove(contractId, false)
  //       .accounts({
  //         contract: contractAddress,
  //         buyer: firstKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([firstKp])
  //       .rpc({ skipPreflight: true });

  //     console.log("Your transaction signature for approving on buyer:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on buyer!:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while approving on buyer!:", error);
  //   }
  // });

  // it("1-[Failure-invalid seller] Approve by seller(freelancer)!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on seller!:", contractAccount);

  //     const tx = await program.methods
  //       .sellerApprove(contractId, false)
  //       .accounts({
  //         contract: contractAddress,
  //         seller: firstKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([firstKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on seller:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on seller!:", contractAccount);
  //   } catch (error) {
  //     // console.log("Error while approving on seller!:", error);
  //     assert.equal(error.error.errorCode.code, "InvalidSeller");
  //     assert.equal(error.error.errorCode.number, 6000);
  //     assert.equal(error.error.errorMessage, "Invalid seller is trying to release funds!");
  //   }
  // });

  ///===========================//
  // it("1-[Success-No split] Approve by seller(freelancer)!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on seller!:", contractAccount);

  //     const tx = await program.methods
  //       .sellerApprove(contractId, false)
  //       .accounts({
  //         contract: contractAddress,
  //         seller: secondKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         buyerReferralAta: buyerReferralAta.address,
  //         sellerReferralAta: null, // sellerReferralAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([secondKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on seller:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on seller!:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while approving on seller!:", error);
  //   }
  // });

  // it("1-[Failure] Approve by admin!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on admin!:", contractAccount);

  //     const tx = await program.methods
  //       .adminApprove(contractId, 0)
  //       .accounts({
  //         contract: contractAddress,
  //         admin: authKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([authKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on admin:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on admin!:", contractAccount);
  //   } catch (error) {
  //     // console.log("Error while approving on admin!:", error);
  //     assert.equal(error.error.errorCode.code, "NotReadyYet");
  //     assert.equal(error.error.errorCode.number, 6006);
  //     assert.equal(error.error.errorMessage, "Contract is not pending or disputed yet so admin can't approve now or already completed!");
  //   }
  // });

  // // Testing for invalid seller activate
  // it("2-[Success] Create a new contract!", async () => {
  //   try {
  //     // Create a new uuid to use as a new contract id
  //     contractId = uuid().slice(0, 8);
  //     console.log("new contractId:", contractId);

  //     const amount = new anchor.BN(10 * Math.pow(10, decimal)); // 10 BPT token; // 10 USDC
  //     const dispute = new anchor.BN(0.5 * Math.pow(10, decimal)); // 0.5 BPT token; // 0.5 USDC
  //     const deadline = Math.floor(Date.now() / 1000); // + (10 * 24 * 60 * 60); // 10 days in seconds from Current timestamp

  //     const [contract, bump] = anchor.web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.from(anchor.utils.bytes.utf8.encode(CONTRACT_SEED)),
  //         Buffer.from(anchor.utils.bytes.utf8.encode(contractId)),
  //       ],
  //       program.programId
  //     );


  //     contractAddress = contract;
  //     contractBump = bump;
  //     console.log("contractAddress", contractAddress);
  //     console.log("contractBump", contractBump);

  //     contractAta = await getOrCreateAssociatedTokenAccount(
  //       program.provider.connection,
  //       authKp,
  //       payTokenMint,
  //       contractAddress,
  //       true
  //     );

  //     // Call startContract function
  //     const tx = await program.methods
  //       .startContract(
  //         contractId,
  //         amount, 
  //         dispute, 
  //         deadline,
          
  //       )
  //       .accounts({
  //         buyer: firstKp.publicKey,
  //         contract,
  //         seller: secondKp.publicKey,
  //         buyerAta: firstAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         rent: SYSVAR_RENT_PUBKEY,
  //       })

  //       .signers([firstKp])
  //       .rpc();
  //       // .rpc({ skipPreflight: true });


  //     console.log("Your transaction signature for creating a new contract", tx);

  //     // Fetch the contract account and assert the values
  //     const contractAccount = await program.account.contract.fetch(contract);

  //     console.log("new contract account:", contractAccount);

  //   } catch (error) {
  //     console.log("Error while creating a new contract:", error);
  //   }
  // });

  // it("2-[Failure-invalid seller-activate] Activate the contract!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before activating:", contractAccount);

  //     // Call the buy_tickets function
  //     const tx = await program.methods
  //       .activateContract(contractId)
  //       .accounts({
  //         contract: contractAddress,
  //         seller: firstKp.publicKey,
  //         sellerAta: firstAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([firstKp])
  //       .rpc();

  //     console.log("Your transaction signature for activating the contract:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after activating:", contractAccount);
  //   } catch (error) {
  //     // console.log("Error while activating contract!:", error);
  //     assert.equal(error.error.errorCode.code, "InvalidActivator");
  //     assert.equal(error.error.errorCode.number, 6001);
  //     assert.equal(error.error.errorMessage, "Invalid seller is trying to activate contract!");
  //   }
  // });

  // it("2-[Success] Activate the contract!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before activating:", contractAccount);

  //     // Call the buy_tickets function
  //     const tx = await program.methods
  //       .activateContract(contractId)
  //       .accounts({
  //         contract: contractAddress,
  //         seller: secondKp.publicKey,
  //         sellerAta: secondAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([secondKp])
  //       .rpc({ skipPreflight: true });

  //     console.log("Your transaction signature for activating the contract:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after activating:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while activating contract!:", error);
  //   }
  // });

  // // Testing dispute cases
  // it("3-[Success] Create a new contract!", async () => {
  //   try {
  //     // Create a new uuid to use as a new contract id
  //     contractId = uuid().slice(0, 8);
  //     console.log("new contractId:", contractId);

  //     const amount = new anchor.BN(10 * Math.pow(10, decimal)); // 10 BPT token; // 10 USDC
  //     const dispute = new anchor.BN(0.5 * Math.pow(10, decimal)); // 0.5 BPT token; // 0.5 USDC 50 cent
  //     const deadline = Math.floor(Date.now() / 1000) + (10 * 24 * 60 * 60); // 10 days in seconds from Current timestamp

  //     const [contract, bump] = anchor.web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.from(anchor.utils.bytes.utf8.encode(CONTRACT_SEED)),
  //         Buffer.from(anchor.utils.bytes.utf8.encode(contractId)),
  //       ],
  //       program.programId
  //     );


  //     contractAddress = contract;
  //     contractBump = bump;
  //     console.log("contractAddress", contractAddress);
  //     console.log("contractBump", contractBump);

  //     contractAta = await getOrCreateAssociatedTokenAccount(
  //       program.provider.connection,
  //       authKp,
  //       payTokenMint,
  //       contractAddress,
  //       true
  //     );

  //     // Call startContract function
  //     const tx = await program.methods
  //       .startContract(
  //         contractId,
  //         amount, 
  //         dispute, 
  //         deadline,
          
  //       )
  //       .accounts({
  //         buyer: firstKp.publicKey,
  //         contract,
  //         seller: secondKp.publicKey,
  //         buyerAta: firstAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         rent: SYSVAR_RENT_PUBKEY,
  //       })

  //       .signers([firstKp])
  //       .rpc();
  //       // .rpc({ skipPreflight: true });


  //     console.log("Your transaction signature for creating a new contract", tx);

  //     // Fetch the contract account and assert the values
  //     const contractAccount = await program.account.contract.fetch(contract);

  //     console.log("new contract account:", contractAccount);

  //   } catch (error) {
  //     console.log("Error while creating a new contract:", error);
  //   }
  // });

  // it("3-[Success] Activate the contract!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before activating:", contractAccount);

  //     // Call the buy_tickets function
  //     const tx = await program.methods
  //       .activateContract(contractId)
  //       .accounts({
  //         contract: contractAddress,
  //         seller: secondKp.publicKey,
  //         sellerAta: secondAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([secondKp])
  //       .rpc({ skipPreflight: true });

  //     console.log("Your transaction signature for activating the contract:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after activating:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while activating contract!:", error);
  //   }
  // });

  // it("3-[Success-Dissatisfied(split)] Approve by buyer(client)!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on buyer!:", contractAccount);

  //     const tx = await program.methods
  //       .buyerApprove(contractId, true)
  //       .accounts({
  //         contract: contractAddress,
  //         buyer: firstKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([firstKp])
  //       .rpc({ skipPreflight: true });

  //     console.log("Your transaction signature for approving on buyer:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on buyer!:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while approving on buyer!:", error);
  //   }
  // });

  // // 1 - Agree with split
  // it("3-[Success-Agree with split] Approve by seller(freelancer)!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on seller!:", contractAccount);

  //     const tx = await program.methods
  //       .sellerApprove(contractId, true)
  //       .accounts({
  //         contract: contractAddress,
  //         seller: secondKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([secondKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on seller:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on seller!:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while approving on seller!:", error);
  //   }
  // });

  //## - Disagree with split so dispute
  // it("3-[Success-Disagree with split] Approve by seller(freelancer)!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on seller!:", contractAccount);

  //     const tx = await program.methods
  //       .sellerApprove(contractId, false)
  //       .accounts({
  //         contract: contractAddress,
  //         seller: secondKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([secondKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on seller:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on seller!:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while approving on seller!:", error);
  //   }
  // });
  // //## Invalid admin test
  // it("3-[Failure-invalid admin] Approve by admin!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on admin!:", contractAccount);

  //     const tx = await program.methods
  //       .adminApprove(contractId, 0)
  //       .accounts({
  //         contract: contractAddress,
  //         admin: firstKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([firstKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on admin:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on admin!:", contractAccount);
  //   } catch (error) {
  //     // console.log("Error while approving on admin!:", error);
  //     assert.equal(error.error.errorCode.code, "InvalidAdmin");
  //     assert.equal(error.error.errorCode.number, 6003);
  //     assert.equal(error.error.errorMessage, "Invalid admin is trying to release funds!");
  //   }
  // });
  //## 1.Agree with seller
  // it("3-[Success - Dispute-Agree with seller] Approve by admin!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on admin!:", contractAccount);

  //     const tx = await program.methods
  //       .adminApprove(contractId, 1)
  //       .accounts({
  //         contract: contractAddress,
  //         admin: authKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([authKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on admin:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on admin!:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while approving on admin!:", error);
  //   }
  // });

  // //## 2.Agree with buyer
  // it("3-[Success - Dispute-Agree with buyer] Approve by admin!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on admin!:", contractAccount);

  //     const tx = await program.methods
  //       .adminApprove(contractId, 2)
  //       .accounts({
  //         contract: contractAddress,
  //         admin: authKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([authKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on admin:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on admin!:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while approving on admin!:", error);
  //   }
  // });

  //## 3.Agree with split
  // it("3-[Success - Dispute-Agree with split] Approve by admin!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on admin!:", contractAccount);

  //     const tx = await program.methods
  //       .adminApprove(contractId, 3)
  //       .accounts({
  //         contract: contractAddress,
  //         admin: authKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([authKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on admin:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on admin!:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while approving on admin!:", error);
  //   }
  // });


  // it("4-[Success] Create a new contract on Seller!", async () => {
  //   try {
  //     // Create a new uuid to use as a new contract id
  //     contractId = uuid().slice(0, 8);
  //     console.log("new contractId:", contractId);

  //     const amount = new anchor.BN(10 * Math.pow(10, decimal)); // 10 BPT token; // 10 USDC
  //     const dispute = new anchor.BN(0.5 * Math.pow(10, decimal)); // 0.5 BPT token; // 0.5 USDC 50 cent
  //     const deadline = Math.floor(Date.now() / 1000) + (10 * 24 * 60 * 60); // 10 days in seconds from Current timestamp

  //     const [contract, bump] = anchor.web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.from(anchor.utils.bytes.utf8.encode(CONTRACT_SEED)),
  //         Buffer.from(anchor.utils.bytes.utf8.encode(contractId)),
  //       ],
  //       program.programId
  //     );


  //     contractAddress = contract;
  //     contractBump = bump;
  //     console.log("contractAddress", contractAddress);
  //     console.log("contractBump", contractBump);

  //     contractAta = await getOrCreateAssociatedTokenAccount(
  //       program.provider.connection,
  //       authKp,
  //       payTokenMint,
  //       contractAddress,
  //       true
  //     );

  //     // Call startContract function
  //     const tx = await program.methods
  //       .startContractOnSeller(
  //         contractId,
  //         amount, 
  //         dispute, 
  //         deadline,
          
  //       )
  //       .accounts({
  //         buyer: firstKp.publicKey,
  //         contract,
  //         seller: secondKp.publicKey,
  //         buyerAta: firstAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         rent: SYSVAR_RENT_PUBKEY,
  //       })

  //       .signers([firstKp])
  //       .rpc();
  //       // .rpc({ skipPreflight: true });


  //     console.log("Your transaction signature for creating a new contract", tx);

  //     // Fetch the contract account and assert the values
  //     const contractAccount = await program.account.contract.fetch(contract);

  //     console.log("new contract account:", contractAccount);

  //   } catch (error) {
  //     console.log("Error while creating a new contract:", error);
  //   }
  // });

   // it("4-[Success] Accept the contract!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before activating:", contractAccount);

  //     // Call the buy_tickets function
  //     const tx = await program.methods
  //       .acceptContractOnBuyer(contractId)
  //       .accounts({
  //         contract: contractAddress,
  //         seller: secondKp.publicKey,
  //         sellerAta: secondAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([secondKp])
  //       .rpc({ skipPreflight: true });

  //     console.log("Your transaction signature for activating the contract:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after activating:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while activating contract!:", error);
  //   }
  // });

  // it("4-[Success] Activate the contract!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before activating:", contractAccount);

  //     // Call the buy_tickets function
  //     const tx = await program.methods
  //       .activateContract(contractId, false)
  //       .accounts({
  //         contract: contractAddress,
  //         seller: secondKp.publicKey,
  //         sellerAta: secondAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([secondKp])
  //       .rpc({ skipPreflight: true });

  //     console.log("Your transaction signature for activating the contract:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after activating:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while activating contract!:", error);
  //   }
  // });

  // it("4-[Failure-invalid buyer] Approve by buyer(client)!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on buyer!:", contractAccount);

  //     const tx = await program.methods
  //       .buyerApprove(contractId, false)
  //       .accounts({
  //         contract: contractAddress,
  //         buyer: secondKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([secondKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on buyer:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on buyer!:", contractAccount);
  //   } catch (error) {
  //     // console.log("Error while approving on buyer!:", error);
  //     assert.equal(error.error.errorCode.code, "InvalidBuyer");
  //     assert.equal(error.error.errorCode.number, 6002);
  //     assert.equal(error.error.errorMessage, "Invalid buyer is trying to release funds!");
  //   }
  // });

  // it("4-[Success-Satisfied(no split)] Approve by buyer(client)!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on buyer!:", contractAccount);

  //     const tx = await program.methods
  //       .buyerApprove(contractId, false)
  //       .accounts({
  //         contract: contractAddress,
  //         buyer: firstKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([firstKp])
  //       .rpc({ skipPreflight: true });

  //     console.log("Your transaction signature for approving on buyer:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on buyer!:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while approving on buyer!:", error);
  //   }
  // });

  // it("1-[Failure-invalid seller] Approve by seller(freelancer)!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on seller!:", contractAccount);

  //     const tx = await program.methods
  //       .sellerApprove(contractId, false)
  //       .accounts({
  //         contract: contractAddress,
  //         seller: firstKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([firstKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on seller:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on seller!:", contractAccount);
  //   } catch (error) {
  //     // console.log("Error while approving on seller!:", error);
  //     assert.equal(error.error.errorCode.code, "InvalidSeller");
  //     assert.equal(error.error.errorCode.number, 6000);
  //     assert.equal(error.error.errorMessage, "Invalid seller is trying to release funds!");
  //   }
  // });

  // it("4-[Success-No split] Approve by seller(freelancer)!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on seller!:", contractAccount);

  //     const tx = await program.methods
  //       .sellerApprove(contractId, false)
  //       .accounts({
  //         contract: contractAddress,
  //         seller: secondKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([secondKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on seller:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on seller!:", contractAccount);
  //   } catch (error) {
  //     console.log("Error while approving on seller!:", error);
  //   }
  // });

  // it("4-[Failure] Approve by admin!", async () => {
  //   try {
  //     let contractAccount = await program.account.contract.fetch(contractAddress);
  //     console.log("new contract before approving on admin!:", contractAccount);

  //     const tx = await program.methods
  //       .adminApprove(contractId, 0)
  //       .accounts({
  //         contract: contractAddress,
  //         admin: authKp.publicKey,
  //         sellerAta: secondAta.address,
  //         buyerAta: firstAta.address,
  //         adminAta: authAta.address,
  //         contractAta: contractAta.address,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([authKp])
  //       .rpc();

  //     console.log("Your transaction signature for approving on admin:", tx);
   
  //     contractAccount = await program.account.contract.fetch(contractAddress);

  //     console.log("new contract after approving on admin!:", contractAccount);
  //   } catch (error) {
  //     // console.log("Error while approving on admin!:", error);
  //     assert.equal(error.error.errorCode.code, "NotReadyYet");
  //     assert.equal(error.error.errorCode.number, 6006);
  //     assert.equal(error.error.errorMessage, "Contract is not pending or disputed yet so admin can't approve now or already completed!");
  //   }
  // });


  // Hourly gig part
  it("Hourly-gig-part - start hourly gig contract on buyer!", async () => {
    try {
      // let contractAccount = await program.account.hourlyContract.fetch(contractAddress);
      // console.log("new contract before starting hourly contract on buyer!:", contractAccount);

      // Create a new uuid to use as a new contract id
      contractId = uuid().slice(0, 8);
      console.log("new contractId:", contractId);

      const amount = new anchor.BN(10 * Math.pow(10, decimal)); // 10 BPT token; // 10 USDC
      const dispute = new anchor.BN(0.5 * Math.pow(10, decimal)); // 0.4 BPT token; // 0.4 USDC shoulb be 50 cent
      const deadline = Math.floor(Date.now() / 1000) + (10 * 24 * 60 * 60); // 10 days in seconds from Current timestamp

      const [contract, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from(anchor.utils.bytes.utf8.encode(CONTRACT_SEED)),
          Buffer.from(anchor.utils.bytes.utf8.encode(contractId)),
        ],
        program.programId
      );


      contractAddress = contract;
      contractBump = bump;
      console.log("contractAddress", contractAddress);
      console.log("contractBump", contractBump);

      contractAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        authKp,
        payTokenMint,
        contractAddress,
        true
      );

      const hourly_rate = 30;
      const weekly_hours_limit = 40;

      const tx = await program.methods
        .startHourlyContractOnBuyer(contractId, hourly_rate, weekly_hours_limit, dispute, deadline)
        .accounts({
          contract: contractAddress,
          buyer: firstKp.publicKey,
          seller: secondKp.publicKey,
          buyerAta: firstAta.address,
          buyerReferral: null,
          contractAta: contractAta.address,
          payTokenMint,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([firstKp])
        .rpc();

      console.log("Your transaction signature for starting hourly contract on buyer:", tx);
   
      let contractAccount = await program.account.hourlyContract.fetch(contractAddress);

      console.log("new contract after starting hourly contract on buyer!:", contractAccount);
    } catch (error) {
      console.log("Error while starting hourly contract on buyer!:", error);
    }
  });

  it("Hourly-gig-part - activate hourly contract on seller!", async () => {
    try {
      let contractAccount = await program.account.hourlyContract.fetch(contractAddress);
      console.log("new contract before activating hourly contract on seller!:", contractAccount);

      const tx = await program.methods
        .activateHourlyContract(contractId, true)
        .accounts({
          contract: contractAddress,
          seller: secondKp.publicKey,
          sellerAta: secondAta.address,
          sellerReferral: null,
          contractAta: contractAta.address,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondKp])
        .rpc();

      console.log("Your transaction signature for activating hourly contract on seller:", tx);
   
      contractAccount = await program.account.hourlyContract.fetch(contractAddress);

      console.log("new contract after activating hourly contract on seller!:", contractAccount);
    } catch (error) {
      console.log("Error while activating hourly contract on seller!:", error);
    }
  });

  it("Hourly-gig-part - update worked hour!", async () => {
    try {
      let contractAccount = await program.account.hourlyContract.fetch(contractAddress);
      console.log("new contract before updating worked hour on seller!:", contractAccount);

      const tx = await program.methods
        .updateWorkedHour(contractId, 30)
        .accounts({
          contract: contractAddress,
          seller: secondKp.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondKp])
        .rpc();

      console.log("Your transaction signature for updating worked hour on seller:", tx);
   
      contractAccount = await program.account.hourlyContract.fetch(contractAddress);

      console.log("new contract after updating worked hour on seller!:", contractAccount);
    } catch (error) {
      console.log("Error while updating worked hour on seller!:", error);
    }
  });

  it("Hourly-gig-part - pay worked hours!", async () => {
    try {
      let contractAccount = await program.account.hourlyContract.fetch(contractAddress);
      console.log("new contract before paying worked hours on seller!:", contractAccount);

      const amount = new anchor.BN(contractAccount.weekWorkedHour * contractAccount.hourlyRate * Math.pow(10, decimal)); // 10 BPT token; // 10 USDC
      const tx = await program.methods
        .payWorkedHour(contractId, amount)
        .accounts({
          contract: contractAddress,
          buyer: firstKp.publicKey,
          buyerAta: firstAta.address,
          contractAta: contractAta.address,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([firstKp])
        .rpc();

      console.log("Your transaction signature for paying worked hours on seller:", tx);
   
      contractAccount = await program.account.hourlyContract.fetch(contractAddress);

      console.log("new contract after paying worked hours on seller!:", contractAccount);
    } catch (error) {
      console.log("Error while paying worked hours on seller!:", error);
    }
  });

  it("Hourly-gig-part - seller approve hourly contract!", async () => {
    try {
      let contractAccount = await program.account.hourlyContract.fetch(contractAddress);
      console.log("new contract before approving hourly contract on seller!:", contractAccount);

      const tx = await program.methods
        .sellerApproveHourlyContract(contractId, false)
        .accounts({
          contract: contractAddress,
          seller: secondKp.publicKey,
          sellerAta: secondAta.address,
          buyerAta: firstAta.address,
          adminAta: authAta.address,
          buyerReferral,
          sellerReferral,
          buyerReferralAta,
          sellerReferralAta,
          payTokenMint,
          contractAta: contractAta.address,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondKp])
        .rpc();

      console.log("Your transaction signature for approving hourly contract on seller:", tx);
   
      contractAccount = await program.account.hourlyContract.fetch(contractAddress);

      console.log("new contract after approving hourly contract on seller!:", contractAccount);
    } catch (error) {
      console.log("Error while approving hourly contract on seller!:", error);
    }
  });

  it("Hourly-gig-part - pause hourly contract!", async () => {
    try {
      let contractAccount = await program.account.hourlyContract.fetch(contractAddress);
      console.log("new contract before pausing hourly contract:", contractAccount);

      const tx = await program.methods
        .sellerApprove(contractId, false)
        .accounts({
          contract: contractAddress,
          seller: secondKp.publicKey,
          sellerAta: secondAta.address,
          buyerAta: firstAta.address,
          adminAta: authAta.address,
          contractAta: contractAta.address,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondKp])
        .rpc();

      console.log("Your transaction signature for pausing hourly contract:", tx);
   
      contractAccount = await program.account.hourlyContract.fetch(contractAddress);

      console.log("new contract after pausing hourly contract:", contractAccount);
    } catch (error) {
      console.log("Error while pausing hourly contract:", error);
    }
  });

  it("Hourly-gig-part - resume hourly contract!", async () => {
    try {
      let contractAccount = await program.account.contract.fetch(contractAddress);
      console.log("new contract before approving on seller!:", contractAccount);

      const tx = await program.methods
        .sellerApprove(contractId, false)
        .accounts({
          contract: contractAddress,
          seller: secondKp.publicKey,
          sellerAta: secondAta.address,
          buyerAta: firstAta.address,
          adminAta: authAta.address,
          contractAta: contractAta.address,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondKp])
        .rpc();

      console.log("Your transaction signature for approving on seller:", tx);
   
      contractAccount = await program.account.contract.fetch(contractAddress);

      console.log("new contract after approving on seller!:", contractAccount);
    } catch (error) {
      console.log("Error while approving on seller!:", error);
    }
  });

  it("Hourly-gig-part - end hourly contract!", async () => {
    try {
      let contractAccount = await program.account.contract.fetch(contractAddress);
      console.log("new contract before approving on seller!:", contractAccount);

      const tx = await program.methods
        .sellerApprove(contractId, false)
        .accounts({
          contract: contractAddress,
          seller: secondKp.publicKey,
          sellerAta: secondAta.address,
          buyerAta: firstAta.address,
          adminAta: authAta.address,
          contractAta: contractAta.address,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondKp])
        .rpc();

      console.log("Your transaction signature for approving on seller:", tx);
   
      contractAccount = await program.account.contract.fetch(contractAddress);

      console.log("new contract after approving on seller!:", contractAccount);
    } catch (error) {
      console.log("Error while approving on seller!:", error);
    }
  });
});
