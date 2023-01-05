# Course 2 - NFTs

## Preamble

For CosmWasm, we delve into the CW721 standard; following the model of the `cw721-base` contract from CosmWasm, we'll replicate parts of that contract. This allows us to explore a breadth of functionality, some new, some previously seen and explored more deeply. It also touches on the use of `cw-storage-plus` which we did not use in the first course and replaces the deprecated `cosmwasm-storage` package.

This course highlights the logic of how NFTs work; there is one contract, but multiple uniquely identified tokens so there needs to be many checks (especially around approval, admin, and transfer/send) to ensure integrity. Metadata is an optional feature which we will also cover.

For Rust, the `cw721-base` applies a few advanced design patterns, including using `Cw721Contract` as an `impl` in order to allow for extensibility as well as extensive use of lifetimes. 

## Synopsis

### Story 

Not all planets are freely open for Jump Ring travel. The Exodites inhabit a cluster of planets that they are protective of. Their lush and beautiful worlds are limited to only a select few who pass the rigorous interview process. If one passes, then they will receive a visa to travel and stay within the Exodite cluster. The Exodites are a simian race, with dimorphic sexes, the females having long, semi-prehensile tails and the males have short, nubby tails. Incredibly intelligent, they long ago cracked the code of how to re-program the Jump Rings, the sign of a truly high sapience level.

Visas are unique to the applicant; they can be approved and revoked by an Administrator of Planetary Experience (an "Ape"). A visa is good for only the Jump Ring it is assigned to. To move between planets, one has to register with the local Ape and the proper Jump Ring transfer their visa there. If they do not transfer in a given period of time, they will be atomized.

The visa tracks sapience levels; some of the Exodite planets do not allow for certain sapience levels to visit the planets at all, as they are considered ecological sanctuaries and only responsible sapients are allowed to view their levels. Such worlds are do not support visa transfers for non-Exodites, and travel even by Exodites must be approved by the Council of High Imperial Managers and Peerages of State (the "CHIMPS").

When an Ape sends the visa over to a new Jump Ring addy, it may be rejected if the species is on the forbidden list (as they don't want any ecological contamination to happen).

### Code

This will require two contracts:
1. **A new CW721 Visa Contract**
> The on-chain metadata will contain the details of the port of entry (first planet that accepted the traveler), along with other appropriate data including of course a photo of the traveler and their DNA sample. Each Jump Ring must have its own Visa handler, so the Visa contract and the Jump Ring contract IDs must match up.
> These tokens are non-transferable except by an APE (see cw721-non-transferable). 
> Visa token lives with the NFT contract initially; it is not valid for travel to any Exodite world, as a Jump Ring requires the NFT to be controlled (i.e., the Jump Ring has to be the) which happens as a function of `SendNft` where the destination contract becomes the owner of the token. This happens after `AssignVisa`.

2. **Modifications to the Jump Ring Contract**
> This will have a "Receive" function that can accept the incoming visa token. When the Jump Ring is activated, it will confirm the sender addy matches that of the list of allowed visas before allowing the traveler to pass. The contract should check that the origin matches that of the planet being transferred from [ed: might have some trouble with this since we put a maximum sapience check in place somewhere too]. 

3. **Workflow** *(* ✅ *represents is tested in code)*
  - User desires to travel to Exodite planet; User must get Visa.
    - Represented by filling out a form and submitting to an approval body.
    - User must provide additional information for later validation (TBD).
  - Visa is acquired by Minting
    - For now, forego the purchase step; assume they are free.
    - Visa NFT becomes associated with the User's account addy.
    - Specific data of the User is filled in with the `VisaMetadata` extension. Need: 
      - ✅ Name for completeness;
      - ✅ Account addy for later validation;
      - Image, i.e., their passport photo for later facial recognition;
      - ✅ `Species` for the approval process at `ReceiveNft`;
      - Planet of origin (later on may include logic to prevent re-use of Visa);
      - Key/Value attributes (for later use in a dapp);
      - DNA string (for later use in a dapp).
  - An Ape "reviews" the Visa and if preliminary approval is okay, submits to the Jump Ring of the destination planet the for pre-approval. 
    - ✅ Pre-approval is represented by sending sending the `token_id` and `VisaMetadata` to the `AssignVisa` function of the Jump Ring.
    - Only Apes can do this, so need to validate.
    - This simply means the Jump Ring has the Visa NFT token id on a whitelist, but not fully approved. The Jump Ring will not work for any user without being approved, which is determined at the time of receiving the Visa to the Jump Ring.
  - When the User (or Ape) sends the NFT via `SendNft` to the Jump Ring's `AssignVisa` function, it will:
    1. ✅ Check that the NFT token id is on the whitelist (from `AssignVisa`).
    2. ✅ Validate other factors (species, sapience levels; TBD, unique to each Jump Ring). Since all the Jump Ring has is the `token_id`, this will require a call back to the NFT contract to get the data.
    3. ✅ If validation passes, then set the whitelist entry will be set to approved.
    4. If not validated, then reject the reception of the NFT (which will then remain with the NFT contract or the last Jump Ring that tried to send the NFT).
  - Once a User is validated, then the `JumpRingTravel` function will allow the User through.

#### Additional Todos
- Add all of this to [Area-52 Jira](https://phi-labs.atlassian.net/jira/software/c/projects/AFT/boards/2).

##### Pending/Updates
- Add SendNft to the Jump Ring, so Visas can be sent between rings.
- Expand the Minter role to include any Ape (add admin list).
- Allow an Ape to revoke a Visa.
- Allow user to change Origin field of `VisaMetadata` at a cost.
- Check to see if the visa is on another Jump Ring.
- Double check rejection method if things like sapience level aren't approved. 

#### Refactors
- Update the cw721-visa readme to be more specific.
- Remove `Option` from some of the `VisaMetadata` fields (currently their for convenience's sake).
- Make the Portal contract more generic; in this Course, the Exodites' Jump Rings have unique conditions for allowing travelers through; this not true for all.
- Rename Visa to ExoVisa or other to make it unique (make Visa generic for later use).

##### New Features
- Add payment system using CW20 tokens.
- Write a different administrative contact that manages the Visa contract so that there is an external contract-contract examples for creating and moving Visas around between the Visa contract and the Jump Rings.
- Dapp
  - Make use of the attributes field on `VisaMetadata` to show information; could use a `Species` (an oracle or db?) to fill that part of the form in automatically as part of the `Mint` process. 
  - Registration page that let's the User apply for a Visa (where they put their destination and personal information).
  - Pre-approval page that allows an Ape to approve (i.e., Mint) or deny the Visa application and send the token id to the preapprove function of a specific Jump Ring. This has an IRL aspect to it, so maybe there is room for a "nice letter" input on the Registration page that the Ape can read before minting.
  - Send Visa page that allows an Ape or the User to send the NFT to a specific Jump Ring contract.
  - Page to show list of Jump Rings(including visitation requirements), Apes, and other relevant information.
  - JumpRingTravel page that allows the user to "move" to the next planet.
- Expand on features allowing Chimps to do more than Apes (perhaps fodder for another story line). 
- Make a Planets contract (low priority, since a Jump Gate basically represents a planet).

## Outline
*This is a suggestion; subject to change as a result of the actual effort.*

### Lesson 1 - The Essentials of the CW721 Spec
- [Description](/course-02-cw721/lesson-01_cw721-base_contract-structure/description.md)
- [Summary](/course-02-cw721/lesson-01_cw721-base_contract-structure/summary.md)

### Lesson 2 - Instantiate and Mint for the Cw721-Base Contract
- [Description](/course-02-cw721/lesson-02_cw721-base_instantiate-mint/description.md)
- [Summary](/course-02-cw721/lesson-02_cw721-base_instantiate-mint/summary.md)

### Lesson 3 - Create the Visa Contract
- [Description](/course-02-cw721/lesson-03_cw721-visa/description.md)
- [Summary](/course-02-cw721/lesson-03_cw721-visa/summary.md)

### Lesson 4 - Creating Cw721ReceiveMsg
- [Description](/course-02-cw721/lesson-04-send_nft-cw721receive/description.md)
- [Summary](/course-02-cw721/lesson-04-send_nft-cw721receive/summary.md)

### Lesson 4 - Add `ReceiveNft` to Portal Contract
- [Description](/course-02-cw721/lesson-05-portal-receive-nft/description.md)
- [Summary](/course-02-cw721/lesson-05-portal-receive-nft/summary.md)
