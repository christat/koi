# Koi

**A GPU-rendered application bootstrapping kit.**

The project is a Cargo workspace with two member crates: **Kohaku** and **Ike**.

## Kohaku
Renderer library crate. It provides the essentials required to bootstrap a desktop application with a dedicated GPU renderer.
Currently the renderer only supports Vulkan, although DirectX12 support also wishlisted.

## Ike
Binary crate. Serves as an example application and testing ground for everything provided in the **Kohaku** crate.

### License
MIT