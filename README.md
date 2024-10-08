# vocab-builder-ui (Work in Progress)

This is the front-end UI component (implemented in python) of the Vocabulary Builder app. The back-end component is https://github.com/MidnightJava/vocab-builder. Eventually the front-end and back-end will be combined into a single AppImage installable product using Tauri (https://tauri.app/)

The purpose of the app is to support the learning of a new language by providing an adaptive vocabulary test capability. The app does not teach you a new language. Rather in works in concert with a language-learning app, such as DuoLingo or Babble. It can support any language, even one you just make up, and it can work with words and/or phrases. Thus you can use it to memorize answers to questions in a specific domain.

The user loads the vocabulary information into the app by either importing a csv file, enering words/phrases on a command line, or adding words/phrases to a table in the User Interface (UI). The user can specify the translation (or answer) for each entry, or the app can lookup translations online via the Microsoft Azure-hosted language translation API. The user must supply an API key for the online lookup to work. The service is free with a usage limit of 500,000 characters per month.

## Project Setup

### Install node.js

Preferably, install at least the latest LTS version. The best way to manage your node.js/npm installatio is to use nvm, which you can fine here: https://github.com/nvm-sh/nvm

### Install project dependencies

```sh
npm install
```

### Compile and Hot-Reload for Development

```sh
npm run dev
```

Point your web browser to http://localhost:5173

### To Build the Tauri App

#### Install Tauri and itialize it

```
1. Install Rust (see https://tauri.app/v1/guides/getting-started/prerequisites)

2. npm install --save-dev @tauri-apps/cli (one time only)
```

#### Build the App

```
1. cd <project directory>

2. npm run tauri dev
```

### Notes

```
1. Environment var VITE_SERVER_PORT must be set to an available port.
The value will be read by the frontend and backend to ensure a
successful connection.
```
