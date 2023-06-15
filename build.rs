fn main()
{
    cxx_build::bridge("src/lib.rs")
        .file("src/my_class.hpp")
        .compile("cpp_from_rust");
    
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/my_class.hpp");
}