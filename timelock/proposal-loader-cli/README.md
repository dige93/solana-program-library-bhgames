# SPL Governance program command-line utility

The proposal-loader-cli, when used for the first time, also takes your program like a normal deployment
and creates a Buffer account for you, but this time it delegates the authority of the Buffer account not to your wallet
but to the [Program Derived Address (PDA)](https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses)
given by the Config you passed into it, and because you also have authority over your Program,
you can delegate your authority to the PDA (notice once transferred, you cannot regain the authority back).
From here on out, now Governance has the authority to upgrade, not you, and proposal-loader-cli will print out commands
you can use in your proposals to upgrade your programs.
