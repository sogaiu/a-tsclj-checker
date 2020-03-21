# a-tsclj-checker

Parse .clj[cs]* files on the filesystem or within .jar files using tree-sitter-clojure, _looking for errors_.

Note that "error" here refers specifically to something tree-sitter concludes is an error based on its use of tree-sitter-clojure.

This project is based on borkdude's [analyze-reify](https://github.com/borkdude/analyze-reify).

## Rationale

Modifying tree-sitter-clojure's grammar has sometimes lead to unexpected breakage.  Testing using a variety of means is employed in order to have some degree of confidence that the grammar behaves reasonably after a change.  This project is used in the approach of testing across a collection of real-world code.

Originally, tree-sitter cli's parse subcommand was used for this purpose.  Although it can handle multiple paths for specific files, perhaps unsurprisingly, it does not handle parsing files stored within .jar files.  In the Clojure world, .clj[cs]* files often live inside .jar files.  This tool should make it a bit easier to _identify errors_ in the relevant content within .jar files without having to perform an explicit extraction.

Additionally, this tool can handle recursively traversing directories to process relevant files.

## Prerequisites

* rust
* npm

## Build

Clone the repository with its submodule:

```
$ git clone https://github.com/sogaiu/a-tsclj-checker --recursive
$ cd a-tsclj-checker
```

Build the `tree-sitter-clojure` source:

```
$ bash script/tree-sitter-clojure
```

Build with the Rust build tool `cargo`:

```
$ cargo build --release
```

or install the tool locally:

```
$ cargo install --path .
```

The binary is named `a-tsclj-checker`.

## Usage

Provide one or multiple paths (files, directories or .jar files):

```
$ a-tsclj-checker /tmp/clojars
/tmp/clojars/clj-mma/clj-mma/0.1.0/clj-mma-0.1.0.jar/mma/conversion.clj
/tmp/clojars/clj-mma/clj-mma/0.1.0/clj-mma-0.1.0.jar/mma/dictionary.clj
/tmp/clojars/clj-mma/clj-mma/0.1.0/clj-mma-0.1.0.jar/primes.clj
/tmp/clojars/ds-utils/ds-utils/0.3.1/ds-utils-0.3.1.jar/ds_utils/locality_sensitive_hashing.clj
/tmp/clojars/coast/coast/0.6.9/coast-0.6.9.jar/controller.clj
/tmp/clojars/coast/coast/0.6.9/coast-0.6.9.jar/model.clj
...
Processed 148373 files in 12426ms. 😎

```

## Thanks

Thanks to borkdude for analyze-reify, discussions, and more :)

## License

See analyze-reify.LICENSE for analyze-reify's license.
