(function() {
    const implementors = Object.fromEntries([["bitcoin",[]],["secp256k1",[]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":59,"fragment_lengths":[14,17]}