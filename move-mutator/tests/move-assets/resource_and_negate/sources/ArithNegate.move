module TestAccount::ArithNegate {
    fun negate(x: u64): u64 {
        // This won't compile for unsigned, but tests the operator detection.
        // Move doesn't support unary minus on u64, so we use a wrapper pattern.
        x
    }

    fun apply_negation_bool(x: bool): bool {
        !x
    }
}
