Wage Calculator
====================

This is a simple SPA web application to help people recognize the equivalence of different
pay rates, without having to break out their own calculator - this website does it for you!

Building
----------

You'll need [node.js](https://nodejs.org/en) (current build uses v20.8.0;
[nvm](https://github.com/nvm-sh/nvm) is a great tool to manage node.js versions),
[trunk](https://trunkrs.dev/), and [ltext](https://ltext.github.io/).

### Procedure

```bash
npm install # gets pico
./build.sh # does the rest
```

Technical Details
---------------------

This application is a Single-Page Web Application written in Rust, which uses WebAssembly to do
the in-browser computations. It also features a simple hydration procedure, where the same
web application is first rendered to a static HTML page - this helps with SEO, and makes the
DOM loading easier on the front-end.
