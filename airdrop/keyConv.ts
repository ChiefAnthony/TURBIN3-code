import { Keypair } from '@solana/web3.js';
import base58 from "bs58";
import * as fs from 'fs';

const PRIVATE_KEY = "";
const PUBLIC_KEY = "9L59nmqeU4ju3iJ2ybab1JTUkQRQiJ41RoxFhSccW1i7";
const secret = base58.decode(PRIVATE_KEY);

// Check if the pk is correct 
const pair = Keypair.fromSecretKey(secret);

if (pair.publicKey.toString() == PUBLIC_KEY) {
  fs.writeFileSync(
    'private_key.json',
    JSON.stringify(Array.from(secret))
  );
}

