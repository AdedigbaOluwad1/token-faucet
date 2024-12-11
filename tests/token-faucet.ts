import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { TokenFaucet } from '../target/types/token_faucet';
import { Keypair, PublicKey } from '@solana/web3.js';
import { assert } from 'chai';
import { BN } from 'bn.js';
import * as dotenv from 'dotenv';

// Load environment variables
dotenv.config();

describe('token-faucet', async () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const program = anchor.workspace.TokenFaucet as Program<TokenFaucet>;

	const faucetAccount = Keypair.fromSecretKey(
		Uint8Array.from(JSON.parse(process.env.FAUCET_ACCOUNT_PRIVATE_KEY!))
	);
	const recipient = Keypair.generate();

	it("Checks if the token PDA exists. If it doesn't, initializes the token faucet PDA. If it does, logs the PDA data.", async () => {
		try {
			const [faucet_pda] = PublicKey.findProgramAddressSync(
				[Buffer.from('faucet_pda')],
				program.programId
			);
			const data = await program.account.faucetPda.fetch(faucet_pda);

			console.log('Faucet PDA Data:', JSON.stringify(data, null, 2));
		} catch (err) {
			console.log('Faucet PDA not found, creating..');

			const tx = await program.methods.initializeFaucetPda().rpc();
			console.log('Token Faucet PDA created ::', tx);
		}
		assert.isTrue(true);
	});

	it("Checks if the recipient PDA exists. If it doesn't, initializes the recipient PDA. If it does, logs the PDA data.", async () => {
		try {
			const [recipient_pda] = PublicKey.findProgramAddressSync(
				[Buffer.from('recipient_pda'), recipient.publicKey.toBuffer()],
				program.programId
			);

			const data = await program.account.recipientPda.fetch(
				recipient_pda
			);
			console.log('Recipient PDA Data:', JSON.stringify(data, null, 2));
		} catch (err) {
			console.log('Recipient PDA not found, creating..');

			const tx = await program.methods
				.initializeRecipientPda()
				.accounts({ recipient: recipient.publicKey })
				.rpc();
			console.log('Recipient PDA created ::', tx);
		}

		assert.isTrue(true);
	});

	it("Initiates transfer of sol tokens from token faucet wallet to recipient's wallet", async () => {
		const tx = await program.methods
			.transferSol(new BN(2_000_000))
			.accounts({
				recipient: recipient.publicKey,
				faucetAccount: faucetAccount.publicKey,
			})
			.signers([faucetAccount])
			.rpc();
		console.log('Token Transfer Successful ::', tx);

		assert.isTrue(true);
	});
});
