# Manta Project

---

## TLDR;

```bash
# Clone the monorepo
git clone https://github.com/eth-cscs/manta-project.git
cd manta-project
# Clone the submodule ochami-rs
git submodule update --init --recursive
```
---
### About
This is the monorepo for the "manta" project containing the following crates:
+ [manta](https://github.com/eth-cscs/manta-project/tree/main/crates/manta)
+ [manta-ws](https://github.com/eth-cscs/manta-project/tree/main/crates/manta-ws)
+ [manta-backend-dispatcher](https://github.com/eth-cscs/manta-project/tree/main/crates/manta-backend-dispatcher)
+ [csm-rs](https://github.com/eth-cscs/manta-project/tree/main/crates/csm-rs)

It also includes [ochami-rs](https://github.com/OpenCHAMI/ochami-rs) as a git submodule.

### Contributing 


### Misc
---
#### How was this monorepo generated ?

Using [git-filter-repo](https://www.git-tower.com/learn/git/faq/git-filter-repo)

See the script at the bottom of this file.

#### How is the monorepo organized ?

Each previous repository is now a single directory under crates/.
If you want to check the history of that single crate, you can run the following:

```sh
git log --crates/csm-rs
```

The monorepo is a Cargo workspace. Ideally, all dependencies shared between the crates should be set
in the top level Cargo.toml and referenced like:

```toml
[dependencies]
base64 = { workspace = true }
```

The dependencies between the crates are be set as follows:

```toml
# manta-ws Cargo.toml
[dependencies]
csm-rs = { version = "0.1.7", path = "../csm-rs" }
```

Note that in that case the "version" field is informational; whatever is present at ../csm-rs will be used as is.

#### Was the history of the old repositories preserved ?

Yes, the following branches have been merged into the main branch of this monorepo:

```
csm-rs                    - origin/v0.5
manta                     - origin/1.5
manta-backend-dispatcher  - origin/main
manta-ws                  - origin/main
ochami-rs                 - origin/main
```
All those commits appear as a linear history when you run "git log". Alternatively if you want to
see only the history of the "csm-rs" crate you can run:
```sh
git log -- crates/csm-rs
```

#### Do the commit SHAs on the monorepo match the SHAs on the old repos ?

No they don't, take e.g. csm-rs, branch v0.5:

Monorepo (new commit)
```
commit 9062593c086a19adeaae89a8886c391c1e67e000 (csm-rs/v0.5)
Author: Manuel Sopena Ballesteros <masber@hotmail.com>
Date:   Wed May 14 11:52:56 2025 +0200

    chore: Release csm-rs version 0.5.0-beta.6
```

Standalone repository (old commit):
```
commit 4450bd86648d536e77264d39cb714f4c3492db48 (HEAD -> v0.5, origin/v0.5)
Author: Manuel Sopena Ballesteros <masber@hotmail.com>
Date:   Wed May 14 11:52:56 2025 +0200

    chore: Release csm-rs version 0.5.0-beta.6
```
#### Were tags preserved ?
Tags from the original repository are preserved but renamed to avoid collisions. I added a prefix
with the name of the repo they came from. 

```
v0.1.2 -> csm-rs-0.1.2
```

This was inspired by the setup on the ripgrep repository, see: https://github.com/BurntSushi/ripgrep/tags

#### How to use cargo release to update tags ?

You can set a release.toml in the root of the workspace like so:
```toml
[config]
tag-prefix = "{{crate_name}}-"  # e.g., manta-v1.0.4
publish = true
push = true
tag = true
allow-branch = ["monorepo-migration", "main"]
dependent-version = "upgrade"  # Update dependent crates' versions
sign-tag = false  # Set to true if you use GPG signing

# Per-crate configuration
[crates.manta]
[crates.manta-ws]
[crates.manta-backend-dispatcher]
[crates.ochami-rs]
[crates.csm-rs]
```

To release a single crate you can then run:
```sh
cargo release patch --package manta
```

#### How to manage tags in the future ?
We can continue using prefixed tags in the future for continuity's sake. That way we can manage the
release of each crate indenpendently. We also have the possibility to use "normal" tags (e.g.
v0.1.2) in the future if we think the releases of all crates should be more synchronized.

#### Are branches preserved ?
Branches from the original repository are preserved and renamed much like the tags

```
fix/update-dependencies -> csm-rs/fix/update-dependencies
```

#### How to build an old version ?
---

Simply checkout the old tag prefixed by the old repo name, e.g. for "csm-rs" tag "v0.9.1":

```sh
git checkout csm-rs-v0.9.1
```

