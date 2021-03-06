// ignore-tidy-linelength
// revisions: legacy v0
//[legacy]compile-flags: -Z symbol-mangling-version=legacy
    //[v0]compile-flags: -Z symbol-mangling-version=v0

#![feature(optin_builtin_traits, rustc_attrs)]
#![allow(dead_code)]

mod foo {
    pub struct Foo { x: u32 }

    impl Foo {
        #[rustc_symbol_name]
        //[legacy]~^ ERROR symbol-name(_ZN5impl13foo3Foo3bar
        //[legacy]~| ERROR demangling(impl1::foo::Foo::bar
        //[legacy]~| ERROR demangling-alt(impl1::foo::Foo::bar)
         //[v0]~^^^^ ERROR symbol-name(_RNvMNtCs4fqI2P2rA04_5impl13fooNtB2_3Foo3bar)
            //[v0]~| ERROR demangling(<impl1[317d481089b8c8fe]::foo::Foo>::bar)
            //[v0]~| ERROR demangling-alt(<impl1::foo::Foo>::bar)
        #[rustc_def_path]
        //[legacy]~^ ERROR def-path(foo::Foo::bar)
           //[v0]~^^ ERROR def-path(foo::Foo::bar)
        fn bar() { }
    }
}

mod bar {
    use foo::Foo;

    impl Foo {
        #[rustc_symbol_name]
        //[legacy]~^ ERROR symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz
        //[legacy]~| ERROR demangling(impl1::bar::<impl impl1::foo::Foo>::baz
        //[legacy]~| ERROR demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
         //[v0]~^^^^ ERROR symbol-name(_RNvMNtCs4fqI2P2rA04_5impl13barNtNtB4_3foo3Foo3baz)
            //[v0]~| ERROR demangling(<impl1[317d481089b8c8fe]::foo::Foo>::baz)
            //[v0]~| ERROR demangling-alt(<impl1::foo::Foo>::baz)
        #[rustc_def_path]
        //[legacy]~^ ERROR def-path(bar::<impl foo::Foo>::baz)
           //[v0]~^^ ERROR def-path(bar::<impl foo::Foo>::baz)
        fn baz() { }
    }
}

trait Foo {
    type Assoc;
}

auto trait AutoTrait {}

fn main() {
    // Test closure mangling, and disambiguators.
    || {};
    || {
        trait Bar {
            fn method(&self) {}
        }

        // Test type mangling, by putting them in an `impl` header.
        // FIXME(eddyb) test C varargs when `core::ffi::VaListImpl` stops leaking into the signature
        // (which is a problem because `core` has an unpredictable hash) - see also #44930.
        impl Bar for [&'_ (dyn Foo<Assoc = extern fn(&u8, /*...*/)> + AutoTrait); 3] {
            #[rustc_symbol_name]
            //[legacy]~^ ERROR symbol-name(_ZN198_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$RP$$u2b$impl1..AutoTrait$u3b$$u20$_$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method
            //[legacy]~| ERROR demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8)+impl1::AutoTrait; _] as impl1::main::{{closure}}::Bar>::method
            //[legacy]~| ERROR demangling-alt(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8)+impl1::AutoTrait; _] as impl1::main::{{closure}}::Bar>::method)
             //[v0]~^^^^ ERROR symbol-name(_RNvXNCNvCs4fqI2P2rA04_5impl14mains_0ARDNtB6_3Foop5AssocFG0_KCRL0_hEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)
                //[v0]~| ERROR demangling(<[&dyn impl1[317d481089b8c8fe]::Foo<Assoc = for<'a, 'b> extern "C" fn(&'b u8)> + impl1[317d481089b8c8fe]::AutoTrait; 3: usize] as impl1[317d481089b8c8fe]::main::{closure#1}::Bar>::method)
                //[v0]~| ERROR demangling-alt(<[&dyn impl1::Foo<Assoc = for<'a, 'b> extern "C" fn(&'b u8)> + impl1::AutoTrait; 3] as impl1::main::{closure#1}::Bar>::method)
            #[rustc_def_path]
            //[legacy]~^ ERROR def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8)> + AutoTrait; _] as main::{{closure}}#1::Bar>::method)
               //[v0]~^^ ERROR def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8)> + AutoTrait; _] as main::{{closure}}#1::Bar>::method)
            fn method(&self) {}
        }
    };
}
