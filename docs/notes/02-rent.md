# Rent accounting

- The session PDA is opened `init_if_needed` by the holder who pays the rent.
- `close_session` uses `close = user`, refunding the rent to the original
  owner regardless of who issues the close.
- Treasury ATA rent is paid once during `initialize` by the program authority.
- Because the program has no upgrade authority wired on mainnet, no one can
  later drain rent from either PDA.
