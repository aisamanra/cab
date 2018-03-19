In short: write `cab build` instead of `cabal new-build`, and save six
keystrokes.

The `cab` tool is a tiny wrapper around Haskell's [`cabal`]()
package-building tool that rewrites the argument list so that the
"new-style" project commands can be invoked without the `new-`
prefix. Otherwise, it simply passes on all other arguments to `cabal`
unmodified, and thus should be mostly a drop-in convenience tool to
shorten some cabal invocations.

# Example Usages

```bash
# cab-specific usage help
$ cab --cab-help
# standard cabal help
$ cab --help
# building a project using a new-style build
$ cab build
# configuring a project using a new-style build
$ cab configure --enable-tests
# running an application
$ cab run
```

# Warning

As of the time of writing, `cab`'s logic is very simplistic: it
rewrites based on a set of known strings and tries to only rewrite
values once. Consequently, an invocation like `cab test run` will
rewrite to `cabal new-test run`, but it does mean that, in rare and
unusual cases, it might apply this logic incorrectly. In the future,
`cab` ought to properly replicate (a subset of) Cabal's command-line
parsing logic, but it does not yet!
