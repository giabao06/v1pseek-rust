# v1pseek now in Rust

An inside joke over at the CuteFunny Discord server. Written in Rust using [serenity](https://github.com/serenity-rs/serenity)

Acts as an interface from Discord to Gemini API with a system prompt to customize the bot's personality and knowledge.

## Set up and run

- Install rust toolchain. You should be able to run `cargo`.
- Clone this repo
- Create a `.env` with bot token, Gemini token, and Gemini model choice.
  - If you use free tier Gemini, it's recommended you use `gemini-3.5-flash-lite` or some other flash lite model for high request volume and fast answers
- Edit `SYSTEM_PROMPT.md` however you like. In this example it's a simple humor bot with some knowledge on the server's internal culture
- `cargo run`, or `cargo build --release`. Make sure to stay in the directory with `.env` and `SYSTEM_PROMPT.md` when executing a binary.

## Trivia?

- This is a one-shot project I did in one day while new to Rust with minimal AI assistance.
- This state will be as far as the project gets in my hands.
  - I do not plan to implement continious interaction and/or reply tracking, or any other features
- The prompt currently in production, written by [Shiina Kochiya](https://github.com/ShiinaKochiya), turns this bot into a クソガキ
- thanks [swyrin](https://github.com/swyrin)

## Licensing

MIT License
