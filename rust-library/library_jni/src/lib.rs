///
/// [cfg(target_os = "android")]: Compiler flag ("cfg") which exposes
/// the JNI interface for targeting Android in this case
///
/// [allow(non_snake_case)]: Tells the compiler not to warn if
/// we are not using snake_case for a variable or function names.
/// For Android Development we want to be consistent with code style. 
///
/// Some useful links:
///  - https://doc.rust-lang.org/reference/items/external-blocks.html
///  - https://doc.rust-lang.org/reference/abi.html
///  - https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier
///  - https://doc.rust-lang.org/reference/items/external-blocks.html 
///
//#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {

    extern crate jni;

    // This is the interface to the JVM 
    // that we'll call the majority of our
    // methods on.
    // @See https://docs.rs/jni/latest/jni/
    use self::jni::JNIEnv;

    // These objects are what you should use as arguments to your 
    // native function. They carry extra lifetime information to 
    // prevent them escaping this context and getting used after 
    // being GC'd.
    use self::jni::objects::{JClass, JString};

    // This is just a pointer. We'll be returning it from our function. 
    // We can't return one of the objects with lifetime information 
    // because the lifetime checker won't let us.
    use self::jni::sys::jstring;

    use library::encrypt;
    use library::decrypt;


    // hello-world
    #[no_mangle]
    pub extern "C" fn Java_com_fernan_apps_LibraryKt_hello<'local>(
        // Notice that this `env` argument is mutable. Any `JNIEnv` API that may
        // allocate new object references will take a mutable reference to the
        // environment.
        mut env: JNIEnv<'local>,
        // this is the class that owns our static method. Not going to be used, but
        // still needs to have an argument slot
        _class: JClass<'local>,
        input: JString<'local>,
    ) -> JString<'local> {
        // First, we have to get the string out of java. Check out the `strings`
        // module for more info on how this works.
        let input: String = env
            .get_string(&input)
            .expect("Couldn't get java string!")
            .into();

        // Then we have to create a new java string to return. Again, more info
        // in the `strings` module.
        let output = env
            .new_string(format!("Hello, {}!", input))
            .expect("Couldn't create java string!");
        output
    }

    ///
    /// Encrypts a String.
    ///
    #[no_mangle] // This keeps Rust from "mangling" the name so it is unique (crate).
    pub extern "system" fn Java_com_fernan_apps_LibraryKt_encrypt<'local>(
        mut env: JNIEnv<'local>,
        // This is the class that owns our static method. It's not going to be used,
        // but still must be present to match the expected signature of a static
        // native method.
        _class: JClass<'local>,
        input: JString<'local>,
    ) -> jstring {

        // First, we have to get the string out of Java. Check out the `strings`
        // module for more info on how this works.
        let to_encrypt: String = env.get_string(&input).expect("Couldn't get java string!").into();

        // We encrypt our str calling the cryptor library
        let encrypted_str = encrypt(&to_encrypt);

        // Here we have to create a new Java string to return. Again, more info
        // in the `strings` module.
        let output = env.new_string(&encrypted_str).expect("Couldn't create Java String!");

        // Finally, extract the raw pointer to return.
        output.into_raw()
    }

    ///
    /// Decrypts a String.
    ///
    #[no_mangle] // This keeps Rust from "mangling" the name so it is unique (crate).
    pub extern "system" fn Java_com_fernan_apps_LibraryKt_decrypt<'local>(
        mut env: JNIEnv<'local>,
        // This is the class that owns our static method. It's not going to be used,
        // but still must be present to match the expected signature of a static
        // native method.        
        _class: JClass<'local>,
        input: JString<'local>,
    ) -> jstring {

        // First, we have to get the string out of Java. Check out the `strings`
        // module for more info on how this works.
        let to_decrypt: String = env.get_string(&input).expect("Couldn't get java string!").into();

        // We decrypt our str calling the cryptor library
        let decrypted_str = decrypt(&to_decrypt.to_owned());

        // Here we have to create a new Java string to return. Again, more info
        // in the `strings` module.
        let output = env.new_string(&decrypted_str).expect("Couldn't create Java String!");

        // Finally, extract the raw pointer to return.
        output.into_raw()
    }


}