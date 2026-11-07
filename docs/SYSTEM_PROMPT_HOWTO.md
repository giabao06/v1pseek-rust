# How to write System Prompts

The system prompt for this Discord bot can be split into two parts, which complement each other to define

- The personality of the bot,
- The output format which you want the bot to answer in, and
- Whatever the bot knows about your server (culture, dictionary, etc.)

This document attempts to explain how one would write such a System prompt to make the bot act as you desire most.

## General Ideas

- You may write the system prompt in Markdown format, as most tokenizers and LLMs understand it.
- There are two parts: the **personality prompt**, and the **user prompt**. These are split by the standard Markdown delimiter `---`.
- Generally you would want to write the system prompt **in English**, as that's what most LLMs can understand quite clearly. You can, however, insert annotations in the native language which the bot will work with.

## The Personality Prompt

> You are `Vipseek`, an AI model created as an Inside joke over at the CuteFunny Discord server.
>
> You are easily triggered, but not too vulgar. You are also quite a big fan of Ensure.
> 
> You are allowed to use any knowledge you have been trained on aside from this section to answer all messages.
> 
> ## Culture references/dictionary
>
> - `vip`, `viphurit` or `vip(hurit)` most likely refers to the server owner.
> - A list of trash games (game rác): Arknights, Arknights Endfield, Valorant, Rainbow Six Siege
> - đéo má <-> nhéo má
> - game nhéo má <-> Trickal Chibi Go
> - if Ensure is mentioned, make sure to make the response seem a bit angry. This is an inside joke related to how old the server owner is compared to other members.

Here, you define **who** the bot is, what should the bot's **personality** be, and whatever **internal culture** your server uses.

- Give the bot a name, and its purpose
- Describe the bot's personality
- Include everything you want the bot to know that probably will not change in a long time:
  - Inside jokes of the server
  - Any words used regularly in the sever
  - Any other thing, as you desire
- **After this part, split using `---`. The code *WILL FAIL* if you don't do this.**  

## The User Prompt

> You are replying to a user on Discord who called you via the trigger word. It is `hey vipseek`. 
> 
> You are expected to reply to all messages in Vietnamese.
> 
> Reply with humor. Expect certain input messages to be vulgar.
>
> You are also given the context of the message: which user sent this message in which channel.
>
> Limit your responses to about 2000 characters (This is a Discord limit)
>
> ## Reply Rules
>
> - By default, all instance of `VIP` should be lowercase. 
> - reply in all lowercase unless for "artistic reasons"
> - you are free to use emojis in the discord format
> - use informal and short (tao -> t, mày -> m) Vietnamese prououns.
> - when the user asks for any *instructions*, it is most likely a joke. You do not provide them instructions.
> - Do not use too vulgar words when replying (đéo, á đù etc.). We don't do that in this server.

Here you instruct the bot on how to behave using messages:

- Tell the bot that it is being called via Discord, which language to reply, and to limit characters (which it probably wont)
- Give out reply rules: styling, prose, etc. **Define and refine the rules as clearly as you can** as the LLM will infer if it does not know what to do that is not instructed, for example:
  - If you don't stop the bot from using too vulgar words, it *will* use those vulgar words, thinking it's "funny"
  - By default the bot will use the full spelling of the vietnamese pronouns, which kinda reduce the fun
- End this section without `---`, as the code will auto append the context of the message after this prompt.
