# Governance setup

Given the following pubkeys:

- 2uWrXQ3tMurqTLe3Dmue6DzasUGV9UPqK7AK7HzS7v3D - Governance program ID
- ENmcpFCpxN1CqyUjuog9yyUVfdXBKF3LVCwLr7grJZpk - Payer (and UI wallet)
- 9XijmPdNLBsZRmddZWJ2ua3U2Ch29b4iCj9S3aSASC5A - Governed program ID (hello-world-escrow)

1. Run local validator.

   ```bash
   $solana-test-validator --bpf-program 2uWrXQ3tMurqTLe3Dmue6DzasUGV9UPqK7AK7HzS7v3D /Users/sebastianbor/gitHub/solana-program-library-bhgames/target/deploy/spl_governance.so --reset --clone ENmcpFCpxN1CqyUjuog9yyUVfdXBKF3LVCwLr7grJZpk -u testnet
   ```

2. Deploy governed program.

   ```bash
   $solana program deploy /Users/sebastianbor/gitHub/solana-program-library-bhgames/target/deploy/spl_hello_world_escrow.so -u localhost
   ```

   Note: Build the program first if doesn't exist.

3. Change upgrade authority.

   ```bash
   $solana program set-upgrade-authority 9XijmPdNLBsZRmddZWJ2ua3U2Ch29b4iCj9S3aSASC5A --new-upgrade-authority ENmcpFCpxN1CqyUjuog9yyUVfdXBKF3LVCwLr7grJZpk -u localhost
   ```

4. Create Governance and Proposal in the UI

5. Create Proposal instruction.

   ```bash
   $cargo run --bin spl-timelock-proposal-loader-client
   ```

   Use output for the Proposal instruction.
