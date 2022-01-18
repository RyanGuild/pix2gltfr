# Pix2Gltfr
**a web app to turn web images into .gltf files** \
*by Ryan Guild*



## NodeJS Dependencies
- [Parcel Bundler](https://www.npmjs.com/package/parcel-bundler) for zero config modern js & ts
- [parcel-plugin-wasm.rs](https://www.npmjs.com/package/parcel-plugin-wasm.rs) for [rust](https://www.rust-lang.org/) and [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
- [typescript](https://www.typescriptlang.org/) for typechecking __included in parcel__
- [react](https://reactjs.org/) for dom interaction __included in parcel__

## Rust Dependcies
- [link](./src/pix2gltfr/Cargo.toml)

## Design
- ingest `Event<HTMLImageElement>` and extract `FileList` to `ReturnType<typeof URL.createURLObject>[]`
- create `<img/>` tags to load dataurl objects
- from image extract natural dimensions and draw to `<canvas/>` tags
- `ImageData` objects from canvas are passed to the [webassembly](https://webassembly.org/) module created by [rust](https://www.rust-lang.org/) and [wasm-bindgen](https://github.com/rustwasm/) located in [src/pix2gltfr](./src/pix2gltfr)
- `image_to_gltf` function returns a strinified gltf file constructed of the a cube buffer geometry translated and skined to the pixels of the image. 100% Transparent pixels are skipped.

## Commands
- `yarn start` run a dev server
- `yarn build` build the github pages statics

## Demo
- [github pages](https://ryanguild.github.io/pix2gltfr/)



