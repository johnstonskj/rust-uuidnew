# crate uuidnew

This crate provides a simple cli for generating UUIDs, but with more options
than `uuidgen`.

<!-- markdownlint-disable MD013 -->
[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![crates.io](https://img.shields.io/crates/v/uuidnew.svg)](https://crates.io/crates/uuidnew)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-uuidnew.svg)](<https://github.com/johnstonskj/rust-uuidnew/stargazers>)
<!-- markdownlint-enable MD013 -->

## Examples

```bash
❯ uuidnew --help
Provides a simple cli for generating UUIDs, but with more options than uuidgen.

Usage: uuidnew [OPTIONS] [NAMESPACE_NAME]

Arguments:
  [NAMESPACE_NAME]  

Options:
  -v, --verbose...
          Increase logging verbosity by one level per occurance
  -q, --quiet...
          Decrease logging verbosity by one level per occurance
      --color <WHEN>
          Controls when to use color [default: auto] [possible values: auto, 
          always, never]
  -c, --count <COUNT>
          The number of UUIDs to generate [default: 1]
  -f, --format <FORMAT>
          The format to use for displaying UUIDs [default: dashes-only] [possible
          values: dashes-only, braces-only, dashes-and-braces, plain, compact, urn]
  -n, --version-number <VERSION_NUMBER>
          The UUID version to use [default: v4] [possible values: v1, v3, v4, v5,
          v6, v7, v8]
  -i, --namespace-id <NAMESPACE_ID>
          The namespace ID to use for name-based UUIDs (v3 and v5 only) [possible
          values: dns, url, oid, x500]
  -h, --help
          Print help
  -V, --version
          Print version
```

Calling the command with no additional arguments will return a single V4 random
UUID formatted with dashes only. This is `uuidgen` compatible.

```bash
❯ uuidnew
82e1be1e-1c4b-4152-b1b5-fcc7b19f5396
```

The `--count` or `-c` argument allows the generation of a batch of UUIDs, in
the example below these are again V4 random values.

```bash
❯ uuidnew --count 5
27dca2b1-85f4-4fa1-a042-fc5ad06f3b5a
f96dc75c-8ac1-43f9-b222-770e7c82b1b6
2feba478-7546-49a2-b26f-38d02e1570c5
5da95b28-1856-4a4c-a082-e870d7dfdf9a
c4cae394-28e3-4ed1-a277-ea1696f21d92
```

In the following example we override the default version to generate V7,
timestamped and sortable, values.

```bash
❯ uuidnew --count 5 --version-number v7
01a19ee6-c313-7072-95e9-469d2e1cd104
01a19ee6-c313-7072-95e9-469e905925a6
01a19ee6-c313-7072-95e9-469f39ada924
01a19ee6-c313-7072-95e9-46a058a370cc
01a19ee6-c313-7072-95e9-46a1b2cfb313
```

If you choose either V3 or V5 versions you **must** also provide a namespace ID
and name value, as shown in the following example.

```bash
❯ uuidnew --version-number v3 --namespace-id dns rust-lang.org`
c6db027c-615c-3b4d-959e-1a917747ca5a
```

### Formats

The command supports six different output formats, shown in the examples below.

```bash
❯ uuidnew --format dashes-only
386dd6bd-4c49-4059-8349-b69ee3128249

❯ uuidnew --format braces-only
{3348a4917ea84295990da84409aa10c0}

❯ uuidnew --format dashes-and-braces
{ae78aa50-3fde-41e1-a536-383e29783f0d}

❯ uuidnew --format plain
dd115b2f80b84d43b06219bdd2e551b4

❯ uuidnew --format compact
P35eZk-eTjuFZ_TaBjH6SA

❯ uuidnew --format urn
urn:uuid:1775083e-9fd2-472c-8f99-5f79aea38f87
```

### Inconsistency Errors

**If** the =urn= format is specified, the version **must** be either v4 (the
default) or v7. If any other value is specified the tool displays the following
error message.

```bash
❯ uuidnew --format urn --version-number v1

🛑 Error: Invalid UUID for URN format
   ├── 🔎 UUID has version `1` and variant `RFC4122`
   └── ℹ️  Help: only UUIDs of version 4 or 7 with variant RFC4122 are valid for
           URN format
```

**If** the version number is specified as v3 or v5 and the argument `-i` or
`--namespace-id` is not provided the following error message is displayed.

```bash
❯ uuidnew --version-number v3

🛑 Error: Missing namespace ID/value for name-based UUID
   ├── 🔎 requested UUID version `V3`
   └── ℹ️  Help: name-based UUIDs (v3 and v5) require a namespace ID and value; 
           add the `--namespace-id` and value arguments
```

## Licenses

### Apache-2.0

<!-- markdownlint-disable MD013 -->
> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Licensed under the Apache License, Version 2.0 (the "License");
> you may not use this file except in compliance with the License.
> You may obtain a copy of the License at
> 
>     http://www.apache.org/licenses/LICENSE-2.0
> 
> Unless required by applicable law or agreed to in writing, software
> distributed under the License is distributed on an "AS IS" BASIS,
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> See the License for the specific language governing permissions and
> limitations under the License.
> ```

See the enclosed file [LICENSE-Apache](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/LICENSE-Apache).
<!-- markdownlint-enable MD013 -->

### MIT

<!-- markdownlint-disable MD013 -->
> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the “Software”), to deal
> in the Software without restriction, including without limitation the rights to
> use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
> the Software, and to permit persons to whom the Software is furnished to do so,
> subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
> INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
> HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
> OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
> SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
> ```

See the enclosed file [LICENSE-MIT](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/LICENSE-MIT).
<!-- markdownlint-enable MD013 -->