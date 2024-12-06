# Veritas V2

**Veritas V2** is a Solana-based program designed to leverage NFTs for supply chain tracking, anti-counterfeiting, and authenticity verification. This program supports creating collections, minting NFTs, and verifying collections, making it ideal for applications like pharmaceutical product traceability and transparency.

CA: DK9ZAmcy3qCW15mDxLvhhKMVoqTRHdHbGSYHHmMBQGjL
---

## Features
1. **Collection NFT Creation**  
   Create a parent NFT to act as a collection or grouping for related NFTs.
   
2. **Mint Individual NFTs**  
   Mint unique NFTs associated with metadata like names and symbols, and link them to a collection.
   
3. **Verify NFT Collection Membership**  
   Ensure that an NFT is a verified member of a specified collection.

---

## Prerequisites

### Tools and Libraries:
- **Node.js** (v16+)
- **Anchor Framework**
- **@solana/web3.js** for interacting with the Solana blockchain
- **@coral-xyz/anchor** for program and provider setup
- **@solana/spl-token** for token utilities

### Accounts and Dependencies:
- A Solana wallet funded with test SOL for transactions.
- Access to a Solana devnet or localnet RPC endpoint.
- Keypair files for the NFT mint and collection mint.

---

## Setup Instructions

### 1. Install Dependencies
Clone the repository and run the following command to install required packages:
```bash
npm install
```
###  2. Environment Configuration
 Ensure your environment is correctly configured to point to a Solana cluster (e.g., Devnet or Localnet):

bash
```bash
export ANCHOR_PROVIDER_URL=https://api.devnet.solana.com
export ANCHOR_WALLET=path/to/your/wallet.json
```
### 3. Build and Deploy the Program
  Compile and deploy the Solana program:

```bash
anchor build
anchor deploy
```
## How to Use
Test the Program
### 1. Create a Collection NFT
Run the test script to create a collection NFT:

```bash

npm run test
```
The following script demonstrates creating a collection NFT:

```typescript

program.methods.createCollection("CollectionName", "Symbol").accountsPartial({
  user: wallet.publicKey,
  mint: collectionMint,
  // other required accounts
}).rpc();
```
### 2. Mint an NFT
Mint an individual NFT and associate it with the collection:

```typescript
program.methods.mintNft("NFTName", "Symbol").accountsPartial({
  owner: wallet.publicKey,
  mint: mint,
  collectionMint: collectionMint,
  // other required accounts
}).rpc();
```
### 3. Verify Collection Membership
Verify that the minted NFT belongs to the collection:

```typescript
program.methods.verifyCollection().accountsPartial({
  authority: wallet.publicKey,
  mint,
  collectionMint,
  // other required accounts
}).rpc();
```
# Key Functions

## Create Collection NFT
Creates a parent NFT with a metadata account and a master edition account.
Transfers the minted token to the user’s associated token account.

## Mint Individual NFT
Creates a unique NFT with metadata.
Links the NFT to the collection by referencing the collection metadata.

## Verify Collection Membership
Validates the NFT’s collection association by checking its metadata and master edition accounts.
