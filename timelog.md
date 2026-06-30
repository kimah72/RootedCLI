## Monday June 22, 2026
Start: 3:56pm MST
Stop: 5:04pm MST
Total: 1 hr 8 min

Tasks:
  - Filled out CSE 310 Module Plan form (selected module, description, schedule, risks) ✅
  - Installed Rust toolchain via rustup (rustc 1.96.0, cargo 1.96.0) ✅
  - Installed Visual Studio "Desktop development with C++" workload for MSVC linker ✅
  - Installed rust-analyzer extension in VS Code ✅
  - Troubleshot and resolved linker not found error (VS Installer was mid-update) ✅
  - Created rooted-cli project with cargo new, ran first program successfully ✅

  ## Tuesday June 23, 2026
Morning Commute and After Work
Total: 1 hr 15 min

Tasks:
  - Commute reading: Rust Book Chapter 3 (variables, mutability, data types, functions, control flow) ✅

  ## Wednesday June 24, 2026
Start: 6:14pm MST
Stop: 7:10pm MST
Total: 56 min

Tasks:
  - Defined Plant struct with all 10 fields ✅
  - Created test Plant instance, printed fields via println! ✅
  - Defined CareLog struct with all 5 fields ✅
  - Created test CareLog instance, printed fields via println! ✅
  - Practiced struct literal syntax and field access (dot notation) ✅

## Thursday June 25, 2026 (commute reading, delayed)
Total: ~1h

Tasks:
  - Read Rust Book Chapter 4 (ownership, references, borrowing) ✅

## Friday June 26, 2026
Start: ~5:59pm MST
Stop: 6:48pm MST
Total: ~49 min

Tasks:
  - Added serde, reqwest, tokio dependencies to Cargo.toml ✅
  - Added #[derive(Serialize, Deserialize, Debug)] and serde rename attributes to Plant and CareLog structs ✅
  - Converted main() to async with #[tokio::main] ✅
  - Wrote get_plants() async function: builds URL, sends GET request, deserializes JSON into Vec<Plant> using Result and the ? operator ✅
  - Called get_plants() from main(), confirmed live connection to AWS backend, successfully retrieved 34 real plants ✅

  ## Saturday June 27, 2026
Start: 8:10am MST
Stop: 9:41am MST
Total: ~1h 31m

Tasks:
  - Built list command: looped over Vec<Plant>, printed formatted output ✅
  - Added if/else conditional to handle empty watch_for field gracefully ✅
  - Added impl Plant block with display_name() method (struct + impl stretch challenge) ✅
  - Wired display_name() into the list loop output ✅
  - Added comments throughout main.rs explaining structs, serde, async, ownership/borrowing concepts ✅
  - Explored Cognito sub vs. login email distinction; decided to defer real auth, documented as future work ✅

  ## Tuesday June 30, 2026
Morning Commute
Total: 1h 15m

Tasks:
  - Read Rust Book Chapter 5 (Structs and Method Syntax) ✅

## Tuesday June 30, 2026 (afternoon session)
Start: 2:40pm MST
Stop: 4:23pm MST
Total: 1h 43m

Tasks:
  - Added Cognito USER_PASSWORD_AUTH login flow (email + password prompt → sub extraction via JWT decode) ✅
  - Added base64, uuid, chrono dependencies to Cargo.toml ✅
  - Built log_care() function: constructs CareLog, POSTs to /carelogs ✅
  - Added notes prompt to log_care for optional care notes ✅
  - Built interactive plant selection (numbered list, single plant or log-all) ✅
  - Tested log-all: successfully posted care logs to all 34 plants simultaneously ✅
  - Confirmed all entries in DynamoDB ✅