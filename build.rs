fn main() {
    #[cfg(all(feature = "fdb-7_1", feature = "fdb-7_3"))]
    compile_error!(
        "Features 'fdb-7_1' and 'fdb-7_3' are mutually exclusive. \
         Please enable only one FoundationDB version feature."
    );

    #[cfg(all(feature = "fdb-7_1", feature = "fdb-7_4"))]
    compile_error!(
        "Features 'fdb-7_1' and 'fdb-7_4' are mutually exclusive. \
         Please enable only one FoundationDB version feature."
    );

    #[cfg(all(feature = "fdb-7_3", feature = "fdb-7_4"))]
    compile_error!(
        "Features 'fdb-7_3' and 'fdb-7_4' are mutually exclusive. \
         Please enable only one FoundationDB version feature."
    );

    #[cfg(not(any(feature = "fdb-7_1", feature = "fdb-7_3", feature = "fdb-7_4")))]
    {
        println!(
            "cargo:warning=No FoundationDB version feature enabled. \
             Consider enabling one of 'fdb-7_1', 'fdb-7_3', or 'fdb-7_4'. \
             Default is 'fdb-7_4'."
        );
    }
}
