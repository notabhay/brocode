<p align="center"><code>npm i -g @openai/codex</code><br />or <code>brew install --cask brocode</code></p>
<p align="center"><strong>Brocode CLI</strong> is a coding agent from OpenAI that runs locally on your computer.
<p align="center">
  <img src="https://github.com/openai/codex/blob/main/.github/codex-cli-splash.png" alt="Brocode CLI splash" width="80%" />
</p>
</br>
If you want Brocode in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.openai.com/codex/ide">install in your IDE.</a>
</br>If you want the desktop app experience, run <code>brocode app</code> or visit <a href="https://chatgpt.com/codex?app-landing-page=true">the Brocode App page</a>.
</br>If you are looking for the <em>cloud-based agent</em> from OpenAI, <strong>Brocode Web</strong>, go to <a href="https://chatgpt.com/codex">chatgpt.com/codex</a>.</p>

---

## Quickstart

### Installing and running Brocode CLI

Install globally with your preferred package manager:

```shell
# Install using npm
npm install -g @openai/codex
```

```shell
# Install using Homebrew
brew install --cask brocode
```

Then simply run `brocode` to get started.

<details>
<summary>You can also go to the <a href="https://github.com/openai/codex/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `brocode-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `brocode-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `brocode-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `brocode-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `brocode-x86_64-unknown-linux-musl`), so you likely want to rename it to `brocode` after extracting it.

</details>

### Using Brocode with your ChatGPT plan

Run `brocode` and select **Sign in with ChatGPT**. We recommend signing into your ChatGPT account to use Brocode as part of your Plus, Pro, Team, Edu, or Enterprise plan. [Learn more about what's included in your ChatGPT plan](https://help.openai.com/en/articles/11369540-codex-in-chatgpt).

You can also use Brocode with an API key, but this requires [additional setup](https://developers.openai.com/codex/auth#sign-in-with-an-api-key).

## Docs

- [**Brocode Documentation**](https://developers.openai.com/codex)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
