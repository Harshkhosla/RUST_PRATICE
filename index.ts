import { Connection, Keypair , LAMPORTS_PER_SOL, SystemProgram, Transaction} from "@solana/web3.js";


  

async function main (){
      const connection = new Connection('http://127.0.0.1:8899')

    const kp = new Keypair();
    const dataAccount = new Keypair();
    const sig = await connection.requestAirdrop(kp.publicKey,3000_000_000)
    await connection.confirmTransaction(sig);
    const balance = await connection.getBalance(kp.publicKey);
    console.log(balance)

    const instruction = SystemProgram.createAccount({
        fromPubkey:kp.publicKey,
        newAccountPubkey:dataAccount.publicKey,
        lamports:LAMPORTS_PER_SOL,
        programId:SystemProgram.programId,
        space:8
    })

    const trx = new Transaction().add(instruction);
    trx.feePayer= kp.publicKey;
    trx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    trx.sign(kp);
    await connection.sendTransaction(trx,[kp,dataAccount]);


}

main()