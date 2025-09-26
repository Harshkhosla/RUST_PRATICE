import {expect , test }from "bun:test"
import {Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction} from "@solana/web3.js"
import { COUNTER_SIZE, schema } from "../types";
import * as borsh from "borsh"

let adminAccount = Keypair.generate();
let dataAccount = Keypair.generate();

test ('Account is initlize',async ()=>{
    const connection = new Connection('http://127.0.0.1:8899');
    const trxacc =   await connection.requestAirdrop(adminAccount.publicKey ,1 * LAMPORTS_PER_SOL)
    // const data = await connection.getAccountInfo(adminAccount.publicKey)
    await connection.confirmTransaction(trxacc);
    const data = await connection.getAccountInfo(adminAccount.publicKey);
    console.log(data)

    const programId = new PublicKey("GoNraH4Fp56Y2aYCt5Bt5iwY7xvrPjjPBkWAwxDR2YDN");
    const lamports = await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE);
    const createAccountTrx =SystemProgram.createAccount({
        fromPubkey:adminAccount.publicKey,
        lamports,
        newAccountPubkey:dataAccount.publicKey,
        programId:programId,
        space:COUNTER_SIZE
    })
    const trx = new Transaction()
    trx.add(createAccountTrx);
    const sig = await connection.sendTransaction(trx,[adminAccount,dataAccount])
    await connection.confirmTransaction(sig);
    console.log(dataAccount.publicKey.toBase58())
    const dataAcc = await connection.getAccountInfo(dataAccount.publicKey) ;
    const count = borsh.deserialize(schema,dataAcc?.data);
    console.log(count?.count)

})
