use std::env;
use std::process;
use std::os::unix::process::CommandExt;

const USAGE_HINT: &'static str =
    "NOTICE ABOUT CAB:
  Cabal was invoked using cab, a wrapper script around
  Cabal. For cab-specific help, use --cab-help.\n";

const USAGE: &'static str =
    "Command line Cabal wrapper for new-style builds

  Usage: cab [GLOBAL_FLAGS] [COMMAND [FLAGS]]

cab for the most part has the exact same usage as Cabal, but it
deliberately rewrites certain commands so that they are understood as
the 'new-style project' commands.

Certain old-style commands cannot be invoked through cab (as running
`cab test' will always be treated as `cabal new-test` and not `cabal
test') but any command which does not have a new-style equivalent
(such as `check') will be run identically.

Commands which differ from Cabal:
    cab build     => cabal new-build
    cab configure => cabal new-configure
    cab conf      => cabal new-configure
    cab repl      => cabal new-repl
    cab run       => cabal new-run
    cab test      => cabal new-test
    cab bench     => cabal new-bench
    cab freeze    => cabal new-freeze
    cab haddock   => cabal new-haddock
    cab update    => cabal new-update
    cab install   => cabal new-install
    cab exec      => cabal new-exec
";

/// A function which knows all the replaceable arguments. This will
/// return None if it's not one of the new-style commands it knows
/// about.
fn replace(s: &str) -> Option<String> {
    Some(match s {
        "build" => "new-build".to_owned(),
        "configure" => "new-configure".to_owned(),
        "repl" => "new-repl".to_owned(),
        "run" => "new-run".to_owned(),
        "test" => "new-test".to_owned(),
        "bench" => "new-bench".to_owned(),
        "freeze" => "new-freeze".to_owned(),
        "haddock" => "new-haddock".to_owned(),
        _ => None?,
    })
}

/// This is a tiny stateful iterator that has some logic around
/// whether to rewrite arguments. (Not enough logic yet, I think, but
/// this is fine for a first pass.)
struct CabalArgs {
    seen_cmd: bool,
    args: env::Args,
}

impl CabalArgs {
    fn new() -> CabalArgs {
        let mut args = env::args();
        let _ = args.next();
        CabalArgs {
            seen_cmd: false,
            args: args,
        }
    }
}

impl Iterator for CabalArgs {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        // Once we've seen a proper command, then we don't want to do
        // any kind of rewriting of the rest of it, so we just pass
        // the arguments on
        if self.seen_cmd {
            self.args.next()
        } else {
            // if we haven't seen a proper command yet, then we can
            // keep intercepting arguments
            self.args.next().and_then( |arg| {
                if &arg == "--help" {
                    // if it's asking for help, then we should also
                    // add a short note about how the usage for `cab`
                    // is a bit different, but otherwise let `cabal`
                    // print whatever it wants
                    println!("{}", USAGE_HINT);
                    Some(arg)
                } else if &arg == "--cab-help" {
                    // We totally intercept --cab-help to display our
                    // own full help string and leave it at that.
                    println!("{}", USAGE);
                    process::exit(0);
                } else if let Some(r) = replace(&arg) {
                    // If there is a replacement for this fragment,
                    // then we've now seen a command (and shouldn't do
                    // anything else) but we should return the
                    // replacement string (i.e. new-build for build)
                    self.seen_cmd = true;
                    Some(r)
                } else {
                    // otherwise, keep going!
                    Some(arg)
                }
            })
        }
    }
}

fn main() {
    process::Command::new("cabal")
        .args(CabalArgs::new())
        .exec();
}
