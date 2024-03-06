## *create-leptos-csr*
Generate a client-side rendered leptos application with one command.

![mesa](https://github.com/friendlymatthew/create-leptos-csr/assets/38759997/39c3c457-abd8-467b-a3b1-07aff61fd0ea)


### Getting Started
Install `create-leptos-csr` globally using:
```bash
cargo install create-leptos-csr-tw
```

This command installs the necessary binary, making it accessible from your command line. 

To initiate a new Leptos project, run:
```bash
create-leptos-csr-tw
```

### Template features:

- [x] TailwindCSS for styling
- [x] Serves image content from `/public` that trunk recognizes
- [x] Mobile viewport configuration 
- [x] An **optional** `vercel` config file to for deployment routing.
- [x] [`Leptos-use`](https://github.com/Synphonyte/leptos-use), a collection of Leptos utilities
- [x] Up to date crate dependencies

### Why I made this
Creating a client-side rendered Leptos application usually requires starting from scratch, involving multiple steps and potential hurdles.

Something like:
```bash
cargo init <project>
cargo add leptos --features=csr,nightly
cd <project>
touch index.html
... # more work here
touch input.css
npx tailwindcss init
... # setting up your tailwind...
mkdir public
(modify `index.html`)
... # adding the same boilerplate to your index.html
trunk serve --open
```

This crate aims to reduce all the hair pulling that a beginner would face when setting up a leptos application. For experienced developers, this crate saves you time by setting up the necessary stuff. 

If you're just starting out, this is great starting point since you can use this template while reading [the Leptos book](https://book.leptos.dev/).


### Recent Updates
- [x] Update wasm-bindgen to 0.2.89
- [x] Removed a lot of boilerplate in `app.rs`
- [x] Restructured project to extend to more templates

### Contributions
Contributions are warmly welcomed and greatly appreciated. I'd love to see other templates that use different styling libraries or deployment services.
