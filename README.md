# on-call-scheduler

A high-performance Rust library compiled to WebAssembly for computing fair on-call schedules.

This library distributes **weekends and public holidays** as evenly as possible across a team of people. It is designed to run inside client-side web applications for responsive and fast scheduling.

---

## ✨ Features

✅ Fair distribution of weekends and public holidays among a team  
✅ Handles connected days as single blocks (e.g. multi-day holidays, weekends)  
✅ Optionally assign each day individually instead of connected blocks  
✅ Distribution aims to be **random and as even as possible**  
✅ Designed for seamless integration with web frontends via WebAssembly

---

## 🔗 Input Data

### Public Holidays

The scheduler takes public holiday data from the [Nager.Date API](https://date.nager.at/swagger/index.html). This API provides holidays in JSON like:

```json
[
  {
    "date": "2025-01-01",
    "localName": "Neujahr",
    "name": "New Year's Day",
    "countryCode": "DE",
    "fixed": true,
    "global": true,
    "counties": null,
    "launchYear": null,
    "type": "Public"
  }
]
````

**Note:** The scheduler expects holidays in this exact format as input.

---

### Other Dates

In addition to public holidays, the scheduler can accept **other arbitrary dates** that should also be distributed fairly (e.g. special company days).

---

### Persons / Identifiers

You provide a list of **unique person identifiers** representing those who participate in the on-call rotation, for example:

```json
["alice", "bob", "carol"]
```

* Each identifier **must be unique**.
* The scheduler will assign days to these persons as evenly as possible.

---

### Configuration Options

The scheduler supports an option to define how connected days are treated:

* **Block Mode (default):**
  If days are connected (e.g. a weekend or multi-day holiday), the entire block is assigned to a single person. No switching in the middle of a block.

* **Daily Mode:**
  Each day is assigned individually, even if days are connected.

---

## 🔄 Output

The scheduler returns a **mapping of person identifiers to lists of dates** assigned to them. Example result:

```json
{
  "alice": ["2025-01-01", "2025-03-30"],
  "bob": ["2025-01-06", "2025-04-20"],
  "carol": ["2025-02-14", "2025-05-01"]
}
```

* All dates are distributed randomly yet as evenly as possible across all persons.
* Each connected block (or individual day) is fully assigned to one person in block mode.

---

## ⚠️ Scope

This scheduler’s goal is to **distribute only weekends and public holidays** evenly. It does not currently handle ordinary weekdays outside these special days.

---

## 📦 How to npm Release

### Prerequisites

Before releasing a new version of your Rust library compiled to WebAssembly, ensure you have the latest version of `wasm-pack` installed:

```bash
cargo install wasm-pack
```

### Steps to Release

1. **Set Version in Cargo.toml**: Update the version number in your `Cargo.toml` file to indicate the new release.

   ```toml
   [package]
   name = "your-package-name"
   version = "x.y.z"  # Update this to the new version number
   ```

2. **Build and Pack the WebAssembly**:

   ```bash
   wasm-pack build --target bundler
   ```

   This command compiles your Rust code to WebAssembly and prepares it for publishing.

3. **Pack**: Create a tarball with `wasm-pack pack`. This step bundles your Rust library into a format suitable for npm publishing.

   ```bash
   wasm-pack pack
   ```

4. **Publish to npm**: Use `wasm-pack publish` to publish your package to the npm registry.

   ```bash
   wasm-pack publish
   ```

5. **Verify the Release**: After publishing, verify the release on npm by checking your package page.
---

## 📦 Usage

To integrate this library into your JavaScript or TypeScript frontend, install the npm package and follow the usage instructions provided in the npm README:

👉 [View the npm package and integration guide](https://www.npmjs.com/package/on-call-sheduler?activeTab=readme)