import * as web3 from "@solana/web3.js";
const PROGRAM_ID = new web3.PublicKey(
  "8Jary8qeULbgUf181XGYpigZzCAyjwpv7NN7qQy1bKMF"
);
const connection = new web3.Connection(
  "http://localhost:8899",
  "confirmed"
);

describe("Test", () => {
  it("Create comment", async () => {
    const commentKeypair = new web3.Keypair();
    console.log(connection);
  });
});
