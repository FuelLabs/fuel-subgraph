## Firehose Integrators Tool Kit

This repository contains all the code that is required to maintain the Golang part of the Firehose stack for chain integrators. This repository can be seen as an Integrator Tool Kit for developers maintaining Firehose for a specific chain. It's essentially a chain-agnostic shared library that is used to avoid duplication across all projects and ease maintenance work for the teams. It contains **no** chain-specific code and everything that is chain-specific must be provided.

> **Note** This repository is **only** useful for maintainers of `firehose-<chain>` repositories and new integrators looking to integrate Firehose into a new chain. If you are a developer using Firehose or Substreams technology, this repository is not for you.

### Philosophy

Firehose maintenance cost comes from two sides. First, there is the chain integration that needs to be maintained. This is done within the chain's code directly by the chain's core developers. The second side of things is the maintenance of the Golang part of the Firehose stack.

Each chain creates its own Firehose Golang repository named `firehose-<chain>`. [Firehose-acme repository](https://github.com/streamingfast/firehose-core/firehose-acme) acts as an example of this. Firehose is composed of multiple smaller components that can be run independently and each of them has a set of CLI flags and other configuration parameters.

The initial "Acme" template we had contained a lot of boilerplate code to properly configure and run the Firehose Golang stack. This meant that if we needed to add a new feature that required a new flag or change a flag default value or any kind of improvements, chain integrators that were maintaining their `firehose-<chain>` repository were in the obligation of tracking changes made in `firehose-acme` and apply those back on their repository by hand.

This was true also for continuously tracking updates to the various small libraries that form the Firehose stack. With Firehose starting to get more and more streamlined across different chains, that was a recipe for a maintenance hell for every chain integration.

This repository aims at solving this maintenance burden by acting as a facade for all the Golang code required to have a functional and up-to-date Firehose stack. This way, we maintain the `firehose-core` project, adding/changing/removing flags, bumping dependencies, and adding new features, while you, as a maintainer of `firehose-<chain>` repository, simply need to track `firehose-core` for new releases and bump a single dependency to be up to date with the latest changes.

### Changelog

The [CHANGELOG.md](./CHANGELOG.md) of this project is written in such way so that you can copy-paste recent changes straight into your own release notes so that operators that are using your `firehose-<chain>` repository are made aware of deprecation notes, removal, changes and other important elements.

Maintainers, you should copy/paste content of this content straight to your project. It is written and meant to be copied over to your project. If you were at `firehose-core` version `1.0.0` and are bumping to `1.1.0`, you should copy the content between those 2 version to your own repository, replacing placeholder value `fire{chain}` with your chain's own binary.

The bash command `awk '/## v0.1.11/,/## v0.1.8/' CHANGELOG.md | grep -v '## v0.1.8'` to obtain the content between 2 versions. You can then merged the different `Added`, `Changed`, `Removed` and others into single merged section.

> [!NOTE]
> At some point will provide a tool to easily generate the change for you so you can simply copy it over.

### Update

When bumping `firehose-core` to a breaking version, details of such upgrade will be described in [UPDATE.md](./UPDATE.md). Breaking version can be be noticed currently if the minor version is bumped up, for example going from v0.1.11 to v0.2.0 introduces some breaking changes. Once we will release the very first major version 1, breaking changes will be when going from v0.y.z to v1.0.0.

### Build & CI

The build and CI files are maintained for now in https://github.com/streamingfast/firehose-acme directly and should be updated manually from time to time from there.
