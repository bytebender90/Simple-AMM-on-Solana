#!/usr/bin/env node

import inquirer from "inquirer";
import chalk from "chalk";
import figlet from "figlet";
import * as anchor from '@project-serum/anchor';
import * as web3 from '@solana/web3.js';
import * as fs from 'fs';
import { BN } from 'bn.js';
import idl from './amm.json' assert { type: "json" };

const keypairFile = `${process.env.HOME}/.config/solana/id.json`;
const secretKey = Uint8Array.from(JSON.parse(fs.readFileSync(keypairFile)));
const keypair = anchor.web3.Keypair.fromSecretKey(secretKey);
const wallet = new anchor.Wallet(keypair);
const connection = new web3.Connection('http://127.0.0.1:8899');
const provider = new anchor.AnchorProvider(connection, wallet, {
  commitment: 'processed'
});
const programId = new anchor.web3.PublicKey('4sRbFuajHVG181psKiK7G2JBSzbcvVD9RBVbo72DE9TQ');
const program = new anchor.Program(idl, programId, provider);

console.log(
  chalk.yellow(figlet.textSync("Anchor AMM CLI", { horizontalLayout: "full" }))
);

console.log(chalk.red(`Using Localnet...`));

console.log(chalk.blue(`AMM program is already deploy on 4tPXqXq5WiLpHPaJSRhpA1we5GhCpQrK3wpdRZFNoFQS`));

await inquirer
  .prompt([
    {
      type: "confirm",
      name: "confirm",
      message: "Do you want to initialize it?",
    },
  ])
  .then((answers) => {
    if (!answers.confirm) {
      console.log(chalk.red("Operation cancelled."));
      process.exit();
    }
  });

let feeTo, fee;

await inquirer
  .prompt([
    {
      type: "input",
      name: "feeTo",
      message: "Fee_To Address?",
    },
  ])
  .then((answers) => {
    feeTo = answers.feeTo;
  });

await inquirer
  .prompt([
    {
      type: "input",
      name: "fee",
      message: "Fee(%)?",
    },
  ])
  .then((answers) => {
    fee = answers.fee * 100;
  });

console.log(feeTo, fee);

const [configPDA] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from(anchor.utils.bytes.utf8.encode('config'))],
  program.programId
);

await program.rpc.initialize(
  feeTo,  // fee_to: The public key of the fee recipient
  new BN(fee),  // fee: The fee value
  {
    accounts: {
      owner: provider.wallet.publicKey,  // The owner who pays for the transaction and initializes the config
      config: configPDA,  // PDA for the config account
      system_program: anchor.web3.SystemProgram.programId,  // System program
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,  // Rent sysvar
    },
    signers: [provider.wallet.payer],
  }
);

console.log(chalk.green(`initialization success!`));


await inquirer
  .prompt([
    {
      type: "confirm",
      name: "confirm",
      message: "Do you want to change the fee?",
    },
  ])
  .then((answers) => {
    if (!answers.confirm) {
      console.log(chalk.red("Operation cancelled."));
      process.exit();
    }
  });

await inquirer
  .prompt([
    {
      type: "input",
      name: "fee",
      message: "Fee(%)?",
    },
  ])
  .then((answers) => {
    fee = answers.fee * 100;
  });

await program.rpc.setFee(
  new BN(fee),  // fee: The fee value
  {
    accounts: {
      owner: provider.wallet.publicKey,  // The owner who pays for the transaction and initializes the config
      config: configPDA,  // PDA for the config account
      system_program: anchor.web3.SystemProgram.programId,  // System program
    },
    signers: [provider.wallet.payer],
  }
);

console.log(chalk.green(`New Fee Set!`));