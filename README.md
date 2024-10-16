# next-routes

`next-routes` is like a `rails routes` for Next.js

## Example

```shell
$ eza -T
.
├── src
│  ├── main.rs
│  └── pages
│     ├── account
│     │  ├── create.tsx
│     │  └── index.tsx
│     ├── api
│     │  └── create.ts
│     └── index.tsx

$ next-routes --src
/
/account
/account/create
/api/create

$ cd src
$ next-routes
/
/account
/account/create
/api/create
```

## How to install

```shell
cargo install next-routes --git https://github.com/funwarioisii/next-routes
```

## How to use

```shell
next-routes
```

if you want to specify the src directory, you can use the `--src` option.

```shell
next-routes --src
```
