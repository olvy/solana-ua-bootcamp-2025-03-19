import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Favorites } from "../target/types/favorites";
import { airdropIfRequired, getCustomErrorMessage } from "@solana-developers/helpers";
import { expect, describe, test } from '@jest/globals';
import { systemProgramErrors } from "./system-program-errors";

describe("favorites", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("Writes and updates favorites on the blockchain", async () => {
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
      anchor.getProvider().connection,
      user.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    // Initial favorite values
    const favoriteNumber = new anchor.BN(23);
    const favoriteColor = "red";

    // Make a transaction to set initial favorites
    let tx: string | null = null;
    try {
      tx = await program.methods
        .setFavorites(favoriteNumber, favoriteColor)
        .accounts({
          user: user.publicKey,
        })
        .signers([user])
        .rpc();
    } catch (thrownObject) {
      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Initial favorites Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );

    // Fetch the initial favorites and validate
    const initialDataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(initialDataFromPda.color).toEqual(favoriteColor);
    expect(initialDataFromPda.number.toNumber()).toEqual(favoriteNumber.toNumber());

    // Now, update the favorites
    const updatedFavoriteNumber = new anchor.BN(42);
    const updatedFavoriteColor = "blue";

    let updateTx: string | null = null;
    try {
      updateTx = await program.methods
        .updateFavorites(updatedFavoriteNumber, updatedFavoriteColor)
        .accounts({
          user: user.publicKey,
        })
        .signers([user])
        .rpc();
    } catch (thrownObject) {
      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Update favorites Tx signature: ${updateTx}`);

    // Fetch the updated favorites and validate
    const updatedDataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(updatedDataFromPda.color).toEqual(updatedFavoriteColor);
    expect(updatedDataFromPda.number.toNumber()).toEqual(updatedFavoriteNumber.toNumber());
  });
});
