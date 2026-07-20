# dorc-lang/v0.1
# An INCOHERENT wrapper (273 §5): predict consumes the verb before "$@" (tail depth 1); lend_map
# reaches "$@" immediately (tail depth 0). Their guests would start at different tokens ⇒ fail-fast.
w__predict() { verb=$1; shift; env "$@"; }
w__lend_map() { : lends user; : lends fs-view; : lends netns; "$@"; }
