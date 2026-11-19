fn main() {
    #[cfg(any(
        all(feature = "fdb-7_1", feature = "fdb-7_3"),
        all(feature = "fdb-7_1", feature = "fdb-7_4"),
        all(feature = "fdb-7_3", feature = "fdb-7_4"),
    ))]
    compile_error!(
        "Features 'fdb-7_1', 'fdb-7_3' and 'fdb-7_4' are mutually exclusive. \
         Please enable only one FoundationDB version feature, and disable the \
         default features to select a version other than 'fdb-7_3'."
    );

    #[cfg(all(
        feature = "fdb",
        not(any(feature = "fdb-7_1", feature = "fdb-7_3", feature = "fdb-7_4"))
    ))]
    {
        println!(
            "cargo:warning=No FoundationDB version feature enabled. \
             Consider enabling one of 'fdb-7_1', 'fdb-7_3' or 'fdb-7_4'. \
             Default is 'fdb-7_3'."
        );
    }
}
