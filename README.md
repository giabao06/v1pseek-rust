# v1pseek now in Rust

An inside joke over at the CuteFunny Discord server. Written in Rust using [serenity](https://github.com/serenity-rs/serenity)

Acts as an interface to Gemini and reply to Discord with System prompt fine-tuning.

## Set up and run

- Install rust toolchain. You should be able to run `cargo`.
- Clone this repo
- Create a `.env` with bot token, Gemini token, and Gemini model choice.
  - If you use free tier Gemini, it's recommended you use `gemini-3.5-flash-lite` or some other flash lite model for high request volume and fast answers
- Edit `SYSTEM_PROMPT.md` however you like.
- `cargo run`
