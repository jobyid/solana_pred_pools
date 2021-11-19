const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...");
  
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Myepicproject;

  const baseAccount = anchor.web3.Keypair.generate();

  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    },
    signers: [baseAccount],
  });

  console.log("Your transaction signiture", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Total Pools', account.totalPools.toString())

  //call add_gif
  await program.rpc.addGif("insert image linke here", "Test Pool","This is a test Pool", "option 1; option 2", 9897, "Verify here", 5,{
    accounts: {
      baseAccount: baseAccount.publicKey,
    }
  });
  await program.rpc.addGif("insert image linke here 3", "Test Pool 2","This is a test Pool 2", "2option 1; 2option 2", 789897, "2Verify here", 2,{
    accounts: {
      baseAccount: baseAccount.publicKey,
    }
  });
  await program.rpc.addResult("winner 1",0,{
    accounts: {
      baseAccount: baseAccount.publicKey,
    }
  });
  await program.rpc.placeBet("winner 1",1,125,"user",  {
    accounts: {
      baseAccount: baseAccount.publicKey,
    }
  })

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Total Pools count ", account.totalPools.toString())

  console.log("Pool list: ", account.poolList)

  // await program.rpc.addResult(0,"winner 1",{
  //   accounts: {
  //     baseAccount: baseAccount.publicKey,
  //   }
  // });

  //console.log("Pool list: ", account.poolList)
};

const runMain = async () => {
  try{
    await main();
    process.exit(0);
  }catch (error){
    console.error(error);
  }
};

runMain();

// describe('myepicproject', () => {

//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.Provider.env());

//   it('Is initialized!', async () => {
//     // Add your test here.
//     const program = anchor.workspace.Myepicproject;
//     const tx = await program.rpc.initialize();
//     console.log("Your transaction signature", tx);
//   });
// });
