# S3 Manager 

A graphical user interface to aws s3 service. 

This project is standalone application for connecting to your AWS S3 service.
Tested on Mac, Linux, Win 10. 

## Installation files
[releases](https://github.com/MassivDash/S3_Manager/releases)


## Features

![LightFiles](/screenshots/lightFiles.png?raw=true)
![DarkFiles](/screenshots/darkFiles.png?raw=true)

![LightImg](/screenshots/lightImg.png?raw=true)
![DarkImg](/screenshots/darkImg.png?raw=true)




- Dark and light mode dependant on system preferences  
- Files, Pictures, Movies and buckets gallery (grid style)
- Optimized for looking through the pictures / movies, lazy loading, images and movies streamed dritecly from the bucket, are loaded with presigned urls. 
- Adjustable grid gallery 
- Simple name search
- folders filtering in movies and images pages 
- Tagging of assets
- Folder creation
- Multi file upload (drop files)
- Galleries remember scroll positions while navigating the app. 




### AWS Account 
In order to use the application you need aws account with s3 service activated. S3 app uses standard aws config files used for aws cli connection [info here](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html).  

```
~/.aws/credentials
```

```
[default]
aws_access_key_id=AKIAIOSFODNN7EXAMPLE
aws_secret_access_key=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
```

```
~/.aws/config
```

```
[default]
region=us-west-2
output=json
```

### Alternate s3 endpoint 

Set env vars ```S3_CUSTOM_ENDPOINT``` to ```true``` or ```1```   at your machine to alter the s3 default endpoints
Set the alternative endpoint url with ```S3_CUSTOM_ENDPOINT_URL``` env

## Local install and development
Rust lang (cargo) and node.js (npm or yarn) and tauri-cli instalations are needed to run / build the project from source

### Development
### Installing the Prerequisites
Project is build thanks to tauri app which uses RUST lang for the system portion of the app and a custom html frontend, in order to start the project you need to install following. 


- Tauri app perquisites https://tauri.app/v1/guides/getting-started/prerequisites 
- Tauri app cli ```cargo install tauri-cli```


### Architecture

This application is based on [tauri.app](https://tauri.app/), RUST lang in the backend (core0 of the app and frontend things are handled by svelte + vite + ts combo. 

Simple division: Rust manages the data and the calls, front end displays the data. 

### Start 


Since rust is responsible for making the aws calls you will need the rust "backend" to be started, without it the app will not work.
You will also need need the aws credentials present on the system
```
cargo tauri dev
```

### Build 

```
cargo tauri build
```


## Backend development
Rust has its own docs on each package 


### Official Template docs:  Front end development
### Svelte + TS + Vite

This template should help get you started developing with Svelte and TypeScript in Vite.

#### Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode).

## Need an official Svelte framework?

Check out [SvelteKit](https://github.com/sveltejs/kit#readme), which is also powered by Vite. Deploy anywhere with its serverless-first approach and adapt to various platforms, with out of the box support for TypeScript, SCSS, and Less, and easily-added support for mdsvex, GraphQL, PostCSS, Tailwind CSS, and more.

## Technical considerations

**Why use this over SvelteKit?**

- It brings its own routing solution which might not be preferable for some users.
- It is first and foremost a framework that just happens to use Vite under the hood, not a Vite app.
  `vite dev` and `vite build` wouldn't work in a SvelteKit environment, for example.

This template contains as little as possible to get started with Vite + TypeScript + Svelte, while taking into account the developer experience with regards to HMR and intellisense. It demonstrates capabilities on par with the other `create-vite` templates and is a good starting point for beginners dipping their toes into a Vite + Svelte project.

Should you later need the extended capabilities and extensibility provided by SvelteKit, the template has been structured similarly to SvelteKit so that it is easy to migrate.

**Why `global.d.ts` instead of `compilerOptions.types` inside `jsconfig.json` or `tsconfig.json`?**

Setting `compilerOptions.types` shuts out all other types not explicitly listed in the configuration. Using triple-slash references keeps the default TypeScript setting of accepting type information from the entire workspace, while also adding `svelte` and `vite/client` type information.

**Why include `.vscode/extensions.json`?**

Other templates indirectly recommend extensions via the README, but this file allows VS Code to prompt the user to install the recommended extension upon opening the project.

**Why enable `allowJs` in the TS template?**

While `allowJs: false` would indeed prevent the use of `.js` files in the project, it does not prevent the use of JavaScript syntax in `.svelte` files. In addition, it would force `checkJs: false`, bringing the worst of both worlds: not being able to guarantee the entire codebase is TypeScript, and also having worse typechecking for the existing JavaScript. In addition, there are valid use cases in which a mixed codebase may be relevant.

**Why is HMR not preserving my local component state?**

HMR state preservation comes with a number of gotchas! It has been disabled by default in both `svelte-hmr` and `@sveltejs/vite-plugin-svelte` due to its often surprising behavior. You can read the details [here](https://github.com/rixo/svelte-hmr#svelte-hmr).

If you have state that's important to retain within a component, consider creating an external store which would not be replaced by HMR.

```ts
// store.ts
// An extremely simple external store
import { writable } from 'svelte/store'
export default writable(0)
```
