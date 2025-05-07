# Backend

## Development requirements

- [sea-orm-cli](https://crates.io/crates/sea-orm-cli) (only for database schema modifications)
  ```
  cargo install sea-orm-cli
  ```
  or with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)
  ```
  cargo binstall sea-orm-cli
  ```

## DB modification workflow
1. Generate a new migration with the sea-orm cli
   ```bash
   sea-orm-cli migrate generate <name>
   ```
2. Start the backend to apply the new migration
3. Generate the rust entities with the sea-orm cli
   ```bash
   sea-orm-cli generate entity -o entity/src/entities
   ```
4. Create a `<name>.rs` file in the `src/db/tables` dir with your db logic and add your table to the tables struct in `src/db/tables/mod.rs`
