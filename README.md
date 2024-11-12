# Solana Counter Program with Anchor

This project is a simple counter program built on the Solana blockchain using the Anchor framework. The counter program allows users to initialize a counter and increment its value.

## Project Structure

```
programs/
  counter/
    src/
      lib.rs
tests/
  counter.ts
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://project-serum.github.io/anchor/getting-started/installation.html)
- [Node.js](https://nodejs.org/)

## Build and Deploy

1. **Install Dependencies**

   ```sh
   npm install
   ```

2. **Build the Program**

   ```sh
   anchor build
   ```

3. **Deploy the Program**

   ```sh
   anchor deploy
   ```

## Testing

To run the tests, use the following command:

```sh
anchor test
```

## Program Overview

### Initialization

The counter program initializes a counter account with the owner's public key and sets the count to zero.

```rust
pub fn init_counter(ctx: Context<InitCounter>) -> Result<()>
```

### Update

The counter program increments the counter by one.

```rust
pub fn update(ctx: Context<UpdateCounter>) -> Result<()>
```

### Accounts

The program uses the following account structures:

```rust
#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    #[account(mut, has_one = owner)]
    pub counter: Account<'info, Counter>,
    pub owner: Signer<'info>,
}
```

## Tests

The tests for the counter program are located in 

counter.ts

. The tests include:

- Initializing the counter account
- Ensuring the counter is incremented by one
- Verifying that only the owner can update the counter


## License

This project is licensed under the MIT License. See the LICENSE file for details.