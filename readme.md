- [The Rust Programming Language Summary](#the-rust-programming-language-summary)
  - [Version](#version)
- [Foreword](#foreword)
- [Introduction](#introduction)
  - [Who Rust Is For](#who-rust-is-for)
    - [Teams of Developers](#teams-of-developers)
    - [Students](#students)
    - [Companies](#companies)
    - [Open Source Developers](#open-source-developers)
    - [People Who Value Speed and Stability](#people-who-value-speed-and-stability)
- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Hello, World!](#hello-world)
  - [Hello, Cargo](#hello-cargo)
- [Programming a Guessing Game](#programming-a-guessing-game)
  - [Setting Up a New Project](#setting-up-a-new-project)
- [Processing a Guess](#processing-a-guess)
- [Common Programming Concepts](#common-programming-concepts)
  - [Variables and Mutability](#variables-and-mutability)
  - [Differences Between Variable and Constants](#differences-between-variable-and-constants)
  - [Shadowing](#shadowing)
  - [Data Types](#data-types)
    - [Scalar Types](#scalar-types)
    - [Compound Types](#compound-types)
  - [Functions](#functions)
    - [Parameters](#parameters)
    - [Statements and Expressions](#statements-and-expressions)
    - [Functions with Return Values](#functions-with-return-values)
  - [Comments](#comments)
  - [Control Flow](#control-flow)
    - [if expression](#if-expression)
    - [Repeatition with Loops](#repeatition-with-loops)
- [Understanding Ownership](#understanding-ownership)
  - [What is Ownership?](#what-is-ownership)
    - [Ownership Rules](#ownership-rules)
    - [Variable Scope](#variable-scope)
    - [The String Type](#the-string-type)
    - [Memory and Allocation](#memory-and-allocation)
    - [Ways Variables and Data Interact: Move](#ways-variables-and-data-interact-move)
    - [Ways Variables and Data Interact: Clone](#ways-variables-and-data-interact-clone)
    - [Stack-Only Data: Copy](#stack-only-data-copy)
    - [Ownership and Function](#ownership-and-function)
    - [Return Values and Scope](#return-values-and-scope)
    - [Returning with Tuple](#returning-with-tuple)
  - [References and Borrowing](#references-and-borrowing)
    - [Mutable References](#mutable-references)
      - [Mutable refernces restriction](#mutable-refernces-restriction)
      - [Multiple mutable refernes by creating a new scope](#multiple-mutable-refernes-by-creating-a-new-scope)
      - [Immutable references](#immutable-references)
    - [Dangling References](#dangling-references)
  - [The Slice Type](#the-slice-type)
    - [String Slices](#string-slices)
      - [String literals are slices](#string-literals-are-slices)
      - [String Slices as Paramters](#string-slices-as-paramters)
      - [Other slices](#other-slices)
- [Using Structs to Structre Related Data](#using-structs-to-structre-related-data)
  - [Defining and Instantiation Structs](#defining-and-instantiation-structs)
    - [Using the Field Init Shorthand](#using-the-field-init-shorthand)
    - [Creating instances from other instnace with update value, or struct update syntax](#creating-instances-from-other-instnace-with-update-value-or-struct-update-syntax)
    - [Using Tuple Structs Without Named Fields to Create Different Types](#using-tuple-structs-without-named-fields-to-create-different-types)
    - [Unit-like Structs without Any Field](#unit-like-structs-without-any-field)
  - [An Example Program Using Struct](#an-example-program-using-struct)
    - [Refactoring with Tuples](#refactoring-with-tuples)
    - [Refactoring with Structs: Adding More Meaning](#refactoring-with-structs-adding-more-meaning)
    - [Adding Useful Functionality with Derived Traits](#adding-useful-functionality-with-derived-traits)
  - [Method Syntax](#method-syntax)
    - [Defining Methods](#defining-methods)
    - [Where’s the -\> Operator?](#wheres-the---operator)
    - [Methods with More Parameters](#methods-with-more-parameters)
    - [Associated Functions](#associated-functions)
    - [Multiple impl Blocks](#multiple-impl-blocks)
- [Enums and Pattern Matching](#enums-and-pattern-matching)
  - [Defining an Enum](#defining-an-enum)
  - [The `Option` Enum and Its Advantages Over Null Values](#the-option-enum-and-its-advantages-over-null-values)
    - [Use Some Value with various method](#use-some-value-with-various-method)
    - [Some Value usecases](#some-value-usecases)
  - [The `match` Control Flow Operator](#the-match-control-flow-operator)
    - [Patterns That Bind to Values](#patterns-that-bind-to-values)
    - [Matching with `Option<T>`](#matching-with-optiont)
    - [Matches Are Exhaustive](#matches-are-exhaustive)
    - [Catch-all Patterns and the \_ Placeholder](#catch-all-patterns-and-the-_-placeholder)
  - [Concise Control FLow with `if let`](#concise-control-flow-with-if-let)
    - [Match one pattern](#match-one-pattern)
    - [With else](#with-else)
- [Managing Growing Projects with Packages, Crates, and Modules](#managing-growing-projects-with-packages-crates-and-modules)
  - [Packages and Crates](#packages-and-crates)
  - [Defining Modules to Control Scope and Privacy](#defining-modules-to-control-scope-and-privacy)
    - [Modules Cheat Sheet](#modules-cheat-sheet)
    - [Grouping Related Code in Modules](#grouping-related-code-in-modules)
    - [Paths for Referring to an Item in the Module Tree](#paths-for-referring-to-an-item-in-the-module-tree)
    - [Starting Relative Paths with `super`](#starting-relative-paths-with-super)
    - [Making Structs and Enums Public](#making-structs-and-enums-public)
  - [Bringing Paths into Scope with the use Keyword](#bringing-paths-into-scope-with-the-use-keyword)
    - [Creating Idiomatic use Paths](#creating-idiomatic-use-paths)
    - [Providing New Names with the as Keyword](#providing-new-names-with-the-as-keyword)
    - [Re-exporting Names with pub use](#re-exporting-names-with-pub-use)
    - [Using External Packages](#using-external-packages)
    - [Using Nested Paths to Clean Up Large use Lists](#using-nested-paths-to-clean-up-large-use-lists)
    - [The Glob Operator](#the-glob-operator)
  - [Separating Modules into Different Files](#separating-modules-into-different-files)
- [Common Collections](#common-collections)
  - [Storing Lists of Values with Vectors](#storing-lists-of-values-with-vectors)
    - [Creating a New Vector](#creating-a-new-vector)
    - [Update a Vector](#update-a-vector)
    - [Reading Elements of Vectors](#reading-elements-of-vectors)
    - [Iterating over the values in a Vector](#iterating-over-the-values-in-a-vector)
    - [Using an enum to store multiple types](#using-an-enum-to-store-multiple-types)
    - [Dropping a Vector Drops Its Elements](#dropping-a-vector-drops-its-elements)
  - [Storing UTF-8 Encoded Text With Strings](#storing-utf-8-encoded-text-with-strings)
    - [What Is a String?](#what-is-a-string)
    - [Creating a New String](#creating-a-new-string)
    - [Updading a String](#updading-a-string)
      - [Appending to a String with push\_str and push](#appending-to-a-string-with-push_str-and-push)
      - [Concatenation with the + Operator or the format! Macro](#concatenation-with-the--operator-or-the-format-macro)
    - [Indexing into Strings](#indexing-into-strings)
      - [Internal Representation](#internal-representation)
      - [Bytes and Scalar Values and Grapheme Clusters! Oh My!](#bytes-and-scalar-values-and-grapheme-clusters-oh-my)
    - [Slicing Strings](#slicing-strings)
    - [Methods for Iterating Over Strings](#methods-for-iterating-over-strings)
    - [Strings Are Not So Simple](#strings-are-not-so-simple)
  - [Storing Keys with Assoicated Values in Hash Maps](#storing-keys-with-assoicated-values-in-hash-maps)
    - [Creating a New Hash Map](#creating-a-new-hash-map)
    - [Accessing Values in a Hash Map](#accessing-values-in-a-hash-map)
    - [Hash Maps and Ownership](#hash-maps-and-ownership)
    - [Updating a Hash Map](#updating-a-hash-map)
      - [Overwriting a Value](#overwriting-a-value)
      - [Adding a Key and Value Only If a Key Isn’t Present](#adding-a-key-and-value-only-if-a-key-isnt-present)
      - [Updating a Value Based on the Old Value](#updating-a-value-based-on-the-old-value)
    - [Hashing Functions](#hashing-functions)
- [Errror Handling](#errror-handling)
  - [Unrecoverable Erros With panic!](#unrecoverable-erros-with-panic)
  - [Using a panic! Backtrace](#using-a-panic-backtrace)
  - [Recoverable Error with Result](#recoverable-error-with-result)
    - [Matching on Different Errors](#matching-on-different-errors)
    - [Alternatives to Using match with Result\<T, E\>](#alternatives-to-using-match-with-resultt-e)
    - [Shortcuts for Panic on Error: unwrap and expect](#shortcuts-for-panic-on-error-unwrap-and-expect)
    - [Propagating Errors](#propagating-errors)
      - [A Shortcut for Propagating Errors: the ? Operator](#a-shortcut-for-propagating-errors-the--operator)
      - [Where The ? Operator Can Be Used](#where-the--operator-can-be-used)
  - [To panic! or Not ro panic!](#to-panic-or-not-ro-panic)
    - [Examples, Prototype Code, and Tests](#examples-prototype-code-and-tests)
    - [Cases in Which You Have More Information Than the Compiler](#cases-in-which-you-have-more-information-than-the-compiler)
    - [Guidelines for Error Handling](#guidelines-for-error-handling)
    - [Creating Custom Types for Validation](#creating-custom-types-for-validation)
- [Generic Types, Traits, and Lifetimes](#generic-types-traits-and-lifetimes)
  - [Removing Duplication by Extracting a Function](#removing-duplication-by-extracting-a-function)
  - [Generic Data Types](#generic-data-types)
    - [In Function Definitions](#in-function-definitions)
    - [In Struct Definitions](#in-struct-definitions)
    - [In Enum Definition](#in-enum-definition)
    - [In Method Definition](#in-method-definition)
    - [Performance of Code Using Generics](#performance-of-code-using-generics)
  - [Trais: Defining Shared Behavior](#trais-defining-shared-behavior)
    - [Defining a Trait](#defining-a-trait)
    - [Implementing a Trait on a Type](#implementing-a-trait-on-a-type)
    - [Default Implementations](#default-implementations)
    - [Traits as Parameters](#traits-as-parameters)
      - [Trait Bound Syntax](#trait-bound-syntax)
      - [Specifying Multiple Trait Bounds with the + Syntax](#specifying-multiple-trait-bounds-with-the--syntax)
      - [Clearer Trait Bounds with where Clauses](#clearer-trait-bounds-with-where-clauses)
    - [Returning Types that Implement Traits](#returning-types-that-implement-traits)
    - [Using Trait Bounds to Conditionally Implement Methods](#using-trait-bounds-to-conditionally-implement-methods)
  - [Validating References with Lifetimes](#validating-references-with-lifetimes)
    - [Preventing Dangling References with Lifetimes](#preventing-dangling-references-with-lifetimes)
    - [The Borrow Checker](#the-borrow-checker)
    - [Generic Lifetimes in Functions](#generic-lifetimes-in-functions)
      - [Lifetime Annotation Syntax](#lifetime-annotation-syntax)
    - [Lifetime Annotations in Function Signatures](#lifetime-annotations-in-function-signatures)
    - [Thinking in Terms of Lifetimes](#thinking-in-terms-of-lifetimes)
    - [Lifetime Annotations in Struct Definitions](#lifetime-annotations-in-struct-definitions)
    - [Lifetime Elision](#lifetime-elision)
    - [Lifetime Annotations in Method Definitions](#lifetime-annotations-in-method-definitions)
    - [The Static Lifetime](#the-static-lifetime)
    - [Generic Type Parameters, Trait Bounds, and Lifetimes Together](#generic-type-parameters-trait-bounds-and-lifetimes-together)
- [Writing Automated Tests](#writing-automated-tests)
  - [How to Write Tests](#how-to-write-tests)
    - [The Anatomy of a Test Function](#the-anatomy-of-a-test-function)
    - [Checking Results with the assert! Macro](#checking-results-with-the-assert-macro)
    - [Testing Equality with the assert\_eq! and assert\_ne! Macros](#testing-equality-with-the-assert_eq-and-assert_ne-macros)
    - [Adding Custom Failure Messages](#adding-custom-failure-messages)
    - [Checking for Panics with should\_panic](#checking-for-panics-with-should_panic)
    - [Using Result\<T, E\> in Tests](#using-resultt-e-in-tests)
  - [Controlling How Tests Are Run](#controlling-how-tests-are-run)
    - [Running Tests in Parallel or Consecutively](#running-tests-in-parallel-or-consecutively)
    - [Showing Function Output](#showing-function-output)
    - [Running a Subset of Tests by Name](#running-a-subset-of-tests-by-name)
    - [Ignoring Some Tests Unless Specifically Requested](#ignoring-some-tests-unless-specifically-requested)
  - [Test Organinzation](#test-organinzation)
    - [Unit Tests](#unit-tests)
      - [Testing Private Functions](#testing-private-functions)
    - [Integration Tests](#integration-tests)
      - [The tests Directory](#the-tests-directory)
      - [Submodules in Integration Tests](#submodules-in-integration-tests)
      - [Integration Tests for Binary Crates](#integration-tests-for-binary-crates)
- [An I/O Project: Building a Command Line Program](#an-io-project-building-a-command-line-program)
  - [Accepting Command Line Arguments](#accepting-command-line-arguments)
    - [Reading the Argument Values](#reading-the-argument-values)
    - [Saving the Argument Values in Variables](#saving-the-argument-values-in-variables)
  - [Reading a File](#reading-a-file)
  - [Refactoring to Improve Modularity and Error Handling](#refactoring-to-improve-modularity-and-error-handling)
    - [Separation of Concerns for Binary Projects](#separation-of-concerns-for-binary-projects)
      - [Extracting the Argument Parser](#extracting-the-argument-parser)
      - [Grouping Configuration Values](#grouping-configuration-values)
      - [Creating a Constructor for Config](#creating-a-constructor-for-config)
    - [Fixing the Error Handling](#fixing-the-error-handling)
      - [Improving the Error Message](#improving-the-error-message)
      - [Returning a Result Instead of Calling panic!](#returning-a-result-instead-of-calling-panic)
      - [Calling Config::build and Handling Errors](#calling-configbuild-and-handling-errors)
    - [Extracting Logic from main](#extracting-logic-from-main)
      - [Returning Errors from the run Function](#returning-errors-from-the-run-function)
      - [Handling Errors Returned from run in main](#handling-errors-returned-from-run-in-main)
    - [Splitting Code into a Library Crate](#splitting-code-into-a-library-crate)
  - [Developing the Library’s Functionality with Test-Driven Development](#developing-the-librarys-functionality-with-test-driven-development)
    - [Writing a Failing Test](#writing-a-failing-test)
    - [Writing Code to Pass the Test](#writing-code-to-pass-the-test)
      - [Using the search Function in the run Function](#using-the-search-function-in-the-run-function)
  - [Working with Environment Variables](#working-with-environment-variables)
    - [Writing a Failing Test for the Case-Insensitive search Function](#writing-a-failing-test-for-the-case-insensitive-search-function)
    - [Implementing the search\_case\_insensitive Function](#implementing-the-search_case_insensitive-function)
  - [Writing Error Messages to Standard Error Instead of Standard Output](#writing-error-messages-to-standard-error-instead-of-standard-output)
    - [Checking Where Errors Are Written](#checking-where-errors-are-written)
    - [Printing Errors to Standard Error](#printing-errors-to-standard-error)
- [Functional Language Features: Iterators and Closures](#functional-language-features-iterators-and-closures)
  - [Closures: Anonymous Functions that Can Capture Their Environment](#closures-anonymous-functions-that-can-capture-their-environment)
    - [Capturing the Environment with Closures](#capturing-the-environment-with-closures)
    - [Closure Type Inference and Annotation](#closure-type-inference-and-annotation)
    - [Capturing References or Moving Ownership](#capturing-references-or-moving-ownership)
    - [Moving Captured Values Out of Closures and the Fn Traits](#moving-captured-values-out-of-closures-and-the-fn-traits)
  - [Processing a Series of Items with Iterators](#processing-a-series-of-items-with-iterators)
    - [The Iterator Trait and the next Method](#the-iterator-trait-and-the-next-method)
    - [Methods that Consume the Iterator](#methods-that-consume-the-iterator)
    - [Methods that Produce Other Iterators](#methods-that-produce-other-iterators)
    - [Using Closures that Capture Their Environment](#using-closures-that-capture-their-environment)
  - [Improving Our I/O Project](#improving-our-io-project)
    - [Removing a clone Using an Iterator](#removing-a-clone-using-an-iterator)
      - [Using the Returned Iterator Directly](#using-the-returned-iterator-directly)
      - [Using Iterator Trait Methods Instead of Indexing](#using-iterator-trait-methods-instead-of-indexing)
    - [Making Code Clearer with Iterator Adaptors](#making-code-clearer-with-iterator-adaptors)
    - [Choosing Between Loops or Iterators](#choosing-between-loops-or-iterators)
  - [Comparing Performance: Loops vs. Iterators](#comparing-performance-loops-vs-iterators)
- [More about Cargo and Crates.io](#more-about-cargo-and-cratesio)
  - [Customizing Builds with Release Profiles](#customizing-builds-with-release-profiles)
  - [Publishing a Crate to Crates.io](#publishing-a-crate-to-cratesio)
    - [Making Useful Documentation Comments](#making-useful-documentation-comments)
      - [Commonly Used Sections](#commonly-used-sections)
      - [Documentation Comments as Tests](#documentation-comments-as-tests)
      - [Commenting Contained Items](#commenting-contained-items)
    - [Exporting a Convenient Public API with pub use](#exporting-a-convenient-public-api-with-pub-use)
    - [Setting Up a Crates.io Account](#setting-up-a-cratesio-account)
    - [Adding Metadata to a New Crate](#adding-metadata-to-a-new-crate)
    - [Publishing to Crates.io](#publishing-to-cratesio)
    - [Publishing a New Version of an Existing Crate](#publishing-a-new-version-of-an-existing-crate)
    - [Deprecating Versions from Crates.io with cargo yank](#deprecating-versions-from-cratesio-with-cargo-yank)
  - [Cargo Workspaces](#cargo-workspaces)
    - [Creating a Workspace](#creating-a-workspace)
    - [Creating the Second Package in the Workspace](#creating-the-second-package-in-the-workspace)
      - [Depending on an External Package in a Workspace](#depending-on-an-external-package-in-a-workspace)
      - [Adding a Test to a Workspace](#adding-a-test-to-a-workspace)
  - [Installing Binaries from Crates.io with cargo install](#installing-binaries-from-cratesio-with-cargo-install)
  - [Extending Cargo with Custom Commands](#extending-cargo-with-custom-commands)
- [Smart Pointers](#smart-pointers)
  - [Using \`Box to Point to Data on he Heap](#using-box-to-point-to-data-on-he-heap)
    - [Using a Box to Store Data on the Heap](#using-a-box-to-store-data-on-the-heap)
    - [Enabling Recursive Types with Boxes](#enabling-recursive-types-with-boxes)
      - [More Information About the Cons List](#more-information-about-the-cons-list)
      - [Computing the Size of a Non-Recursive Type](#computing-the-size-of-a-non-recursive-type)
      - [Using Box to Get a Recursive Type with a Known Size](#using-box-to-get-a-recursive-type-with-a-known-size)
  - [Treating Smart Pointers Like Regular References with the Deref Trait](#treating-smart-pointers-like-regular-references-with-the-deref-trait)
    - [Following the Pointer to the Value](#following-the-pointer-to-the-value)
    - [Using Box Like a Reference](#using-box-like-a-reference)
    - [Defining Our Own Smart Pointer](#defining-our-own-smart-pointer)
    - [Treating a Type Like a Reference by Implementing the Deref Trait](#treating-a-type-like-a-reference-by-implementing-the-deref-trait)
    - [Implicit Deref Coercions with Functions and Methods](#implicit-deref-coercions-with-functions-and-methods)
    - [How Deref Coercion Interacts with Mutability](#how-deref-coercion-interacts-with-mutability)
  - [Running Code on Cleanup with the `Drop` Trait](#running-code-on-cleanup-with-the-drop-trait)
    - [Dropping a Value Early with std::mem::drop](#dropping-a-value-early-with-stdmemdrop)
  - [Rc, the Reference Counted Smart Pointer](#rc-the-reference-counted-smart-pointer)
  - [Rc, the Reference Counted Smart Pointer](#rc-the-reference-counted-smart-pointer-1)
    - [Using Rc to Share Data](#using-rc-to-share-data)
    - [Cloning an Rc Increases the Reference Count](#cloning-an-rc-increases-the-reference-count)
  - [RefCell and the Interior Mutability Pattern](#refcell-and-the-interior-mutability-pattern)
  - [RefCell and the Interior Mutability Pattern](#refcell-and-the-interior-mutability-pattern-1)
    - [Enforcing Borrowing Rules at Runtime with RefCell](#enforcing-borrowing-rules-at-runtime-with-refcell)
    - [Interior Mutability: A Mutable Borrow to an Immutable Value](#interior-mutability-a-mutable-borrow-to-an-immutable-value)
      - [A Use Case for Interior Mutability: Mock Objects](#a-use-case-for-interior-mutability-mock-objects)
      - [Keeping Track of Borrows at Runtime with RefCell](#keeping-track-of-borrows-at-runtime-with-refcell)
    - [Having Multiple Owners of Mutable Data by Combining Rc and RefCell](#having-multiple-owners-of-mutable-data-by-combining-rc-and-refcell)
  - [Reference Cycles Can Leak Memory](#reference-cycles-can-leak-memory)
    - [Creating a Reference Cycle](#creating-a-reference-cycle)
    - [Preventing Reference Cycles: Turning an Rc into a Weak](#preventing-reference-cycles-turning-an-rc-into-a-weak)
      - [Creating a Tree Data Structure: a Node with Child Nodes](#creating-a-tree-data-structure-a-node-with-child-nodes)
      - [Adding a Reference from a Child to Its Parent](#adding-a-reference-from-a-child-to-its-parent)
      - [Visualizing Changes to strong\_count and weak\_count](#visualizing-changes-to-strong_count-and-weak_count)
- [Fearless Concurrency](#fearless-concurrency)
  - [Using Threads to Run Code Simultaneously](#using-threads-to-run-code-simultaneously)
    - [Creating a New Thread with spawn](#creating-a-new-thread-with-spawn)
    - [Waiting for All Threads to Finish Using join Handles](#waiting-for-all-threads-to-finish-using-join-handles)
    - [Using move Closures with Threads](#using-move-closures-with-threads)
    - [Using Message Passing to Transfer Data Between Threads](#using-message-passing-to-transfer-data-between-threads)
    - [Channels and Ownership Transference](#channels-and-ownership-transference)
    - [Sending Multiple Values and Seeing the Receiver Waiting](#sending-multiple-values-and-seeing-the-receiver-waiting)
    - [Creating Multiple Producers by Cloning the Transmitter](#creating-multiple-producers-by-cloning-the-transmitter)
  - [Shared-State Concurrency](#shared-state-concurrency)
    - [Using Mutexes to Allow Access to Data from One Thread at a Time](#using-mutexes-to-allow-access-to-data-from-one-thread-at-a-time)
      - [The API of Mutex](#the-api-of-mutex)
      - [Sharing a Mutex Between Multiple Threads](#sharing-a-mutex-between-multiple-threads)
      - [Multiple Ownership with Multiple Threads](#multiple-ownership-with-multiple-threads)
      - [Atomic Reference Counting with Arc](#atomic-reference-counting-with-arc)
    - [Similarities Between RefCell/Rc and Mutex/Arc](#similarities-between-refcellrc-and-mutexarc)
  - [Extensible Concurrency with the Sync and Send Traits](#extensible-concurrency-with-the-sync-and-send-traits)
    - [Allowing Transference of Ownership Between Threads with Send](#allowing-transference-of-ownership-between-threads-with-send)
    - [Allowing Access from Multiple Threads with Sync](#allowing-access-from-multiple-threads-with-sync)
    - [Implementing Send and Sync Manually Is Unsafe](#implementing-send-and-sync-manually-is-unsafe)
- [Object Oriented Programming Features of Rust](#object-oriented-programming-features-of-rust)
  - [Characteristics of Object-Oriented Languages](#characteristics-of-object-oriented-languages)
    - [Objects Contain Data and Behavior](#objects-contain-data-and-behavior)
    - [Encapsulation that Hides Implementation Details](#encapsulation-that-hides-implementation-details)
    - [Inheritance as a Type System and as Code Sharing](#inheritance-as-a-type-system-and-as-code-sharing)
  - [Using Trait Objects that Allow for Values of Different Types](#using-trait-objects-that-allow-for-values-of-different-types)
    - [Defining a Trait for Common Behavior](#defining-a-trait-for-common-behavior)
    - [Implementing the Trait](#implementing-the-trait)
    - [Trait Objects Perform Dynamic Dispatch](#trait-objects-perform-dynamic-dispatch)
  - [Implementing an Object-Oriented Design Pattern](#implementing-an-object-oriented-design-pattern)
    - [Defining Post and Creating a New Instance in the Draft State](#defining-post-and-creating-a-new-instance-in-the-draft-state)
    - [Storing the Text of the Post Content](#storing-the-text-of-the-post-content)
    - [Ensuring the Content of a Draft Post Is Empty](#ensuring-the-content-of-a-draft-post-is-empty)
    - [Requesting a Review of the Post Changes Its State](#requesting-a-review-of-the-post-changes-its-state)
    - [Adding approve to Change the Behavior of content](#adding-approve-to-change-the-behavior-of-content)
    - [Trade-offs of the State Pattern](#trade-offs-of-the-state-pattern)
    - [Encoding States and Behavior as Types](#encoding-states-and-behavior-as-types)
    - [Implementing Transitions as Transformations into Different Types](#implementing-transitions-as-transformations-into-different-types)
- [Patterns and Matching](#patterns-and-matching)
  - [All the Places Patterns Can Be Used](#all-the-places-patterns-can-be-used)
    - [match Arms](#match-arms)
  - [All the Places Patterns Can be Used](#all-the-places-patterns-can-be-used-1)
    - [Conditional `if let` Expressions](#conditional-if-let-expressions)
    - [`while let` Conditional Loops](#while-let-conditional-loops)
    - [`for` Loops](#for-loops)
    - [`let` Statements](#let-statements)
    - [Function Parameters](#function-parameters)
  - [Refutability: Whether a Pattern Might Fail to Match](#refutability-whether-a-pattern-might-fail-to-match)
  - [Pattern Syntax](#pattern-syntax)
    - [Matching Literals](#matching-literals)
    - [Matching Named Variables](#matching-named-variables)
    - [Multiple Patterns](#multiple-patterns)
    - [Matching Ranges of Values with ..=](#matching-ranges-of-values-with-)
    - [Destructuring to Break Apart Values](#destructuring-to-break-apart-values)
      - [Destructuring Structs](#destructuring-structs)
      - [Destructuring Enums](#destructuring-enums)
      - [Destructuring Nested Structs and Enums](#destructuring-nested-structs-and-enums)
      - [Destructuring Structs and Tuples](#destructuring-structs-and-tuples)
    - [Ignoring Values in a Pattern](#ignoring-values-in-a-pattern)
      - [Ignoring an Entire Value with \_](#ignoring-an-entire-value-with-_)
      - [Ignoring Parts of a Value with a Nested \_](#ignoring-parts-of-a-value-with-a-nested-_)
      - [Ignoring an Unused Variable by Starting Its Name with \_](#ignoring-an-unused-variable-by-starting-its-name-with-_)
      - [Ignoring Remaining Parts of a Value with ..](#ignoring-remaining-parts-of-a-value-with-)
    - [Extra Conditionals with Match Guards](#extra-conditionals-with-match-guards)
    - [@ Bindings](#-bindings)
- [Advanced Features](#advanced-features)
  - [Unsafe Rust](#unsafe-rust)
    - [Unsafe Superpowers](#unsafe-superpowers)
    - [Dereferencing a Raw Pointer](#dereferencing-a-raw-pointer)
    - [Calling an Unsafe Function or Method](#calling-an-unsafe-function-or-method)
    - [Creating a Safe Abstraction over Unsafe Code](#creating-a-safe-abstraction-over-unsafe-code)
      - [Using extern Functions to Call External Code](#using-extern-functions-to-call-external-code)
      - [Calling Rust Functions from Other Languages](#calling-rust-functions-from-other-languages)
    - [Accessing or Modifying a Mutable Static Variable](#accessing-or-modifying-a-mutable-static-variable)
    - [Implementing an Unsafe Trait](#implementing-an-unsafe-trait)
    - [Accessing Fields of a Union](#accessing-fields-of-a-union)
    - [When to Use Unsafe Code](#when-to-use-unsafe-code)
  - [Advanced Traits](#advanced-traits)
    - [Specifying Placeholder Types in Trait Definitions with Associated Types](#specifying-placeholder-types-in-trait-definitions-with-associated-types)
    - [Default Generic Type Parameters and Operator Overloading](#default-generic-type-parameters-and-operator-overloading)
    - [Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name](#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name)
    - [Using Supertraits to Require One Trait’s Functionality Within Another Trait](#using-supertraits-to-require-one-traits-functionality-within-another-trait)
    - [Using the Newtype Pattern to Implement External Traits on External Types](#using-the-newtype-pattern-to-implement-external-traits-on-external-types)
  - [Advanced Types](#advanced-types)
    - [Using the Newtype Pattern for Type Safety and Abstraction](#using-the-newtype-pattern-for-type-safety-and-abstraction)
    - [Creating Type Synonyms with Type Aliases](#creating-type-synonyms-with-type-aliases)
    - [The Never Type that Never Returns](#the-never-type-that-never-returns)
    - [Dynamically Sized Types(DST) and the Sized Trait](#dynamically-sized-typesdst-and-the-sized-trait)
  - [Advanced Functions \& Closures](#advanced-functions--closures)
    - [Function Pointers](#function-pointers)
    - [Returning Closures](#returning-closures)
  - [Macros](#macros)
    - [The Difference Between Macros and Functions](#the-difference-between-macros-and-functions)
    - [Declarative Macros with macro\_rules! for General Metaprogramming](#declarative-macros-with-macro_rules-for-general-metaprogramming)
    - [Procedural Macros for Generating Code from Attributes](#procedural-macros-for-generating-code-from-attributes)
    - [How to Write a Custom derive Macro](#how-to-write-a-custom-derive-macro)
    - [Attribute-like macros](#attribute-like-macros)
    - [Function-like macros](#function-like-macros)
- [Final Project: Building a Multithreaded Web Server](#final-project-building-a-multithreaded-web-server)
  - [Building a Single-Threaded Web Server](#building-a-single-threaded-web-server)
    - [Listening to the TCP Connection](#listening-to-the-tcp-connection)
    - [Reading the Request](#reading-the-request)
    - [A Closer Look at an HTTP Request](#a-closer-look-at-an-http-request)
    - [Writing a Response](#writing-a-response)
    - [Returning Real HTML](#returning-real-html)
    - [Validating the Request and Selectively Responding](#validating-the-request-and-selectively-responding)
  - [A Touch of Refactoring](#a-touch-of-refactoring)
  - [Turning Our Single-Threaded Server into a Multithreaded Server](#turning-our-single-threaded-server-into-a-multithreaded-server)
    - [Simulating a Slow Request in the Current Server Implementation](#simulating-a-slow-request-in-the-current-server-implementation)
    - [Improving Throughput with a Thread Pool](#improving-throughput-with-a-thread-pool)
      - [Spawning a Thread for Each Request](#spawning-a-thread-for-each-request)
      - [Creating a Finite Number of Threads](#creating-a-finite-number-of-threads)
      - [Building ThreadPool Using Compiler Driven Development](#building-threadpool-using-compiler-driven-development)
      - [Validating the Number of Threads in new](#validating-the-number-of-threads-in-new)
      - [Creating Space to Store the Threads](#creating-space-to-store-the-threads)
      - [A Worker Struct Responsible for Sending Code from the ThreadPool to a Thread](#a-worker-struct-responsible-for-sending-code-from-the-threadpool-to-a-thread)
      - [Sending Requests to Threads via Channels](#sending-requests-to-threads-via-channels)
      - [Implementing the execute Method](#implementing-the-execute-method)
  - [Graceful Shutdown and Cleanup](#graceful-shutdown-and-cleanup)
    - [Implementing the Drop Trait on ThreadPool](#implementing-the-drop-trait-on-threadpool)
    - [Signaling to the Threads to Stop Listening for Jobs](#signaling-to-the-threads-to-stop-listening-for-jobs)

# [The Rust Programming Language](https://doc.rust-lang.org/book/) Summary

> Summary, Short version of The Rust Programming Language.

## Version

- Updated on 2023-02 with Rust 1.65 over version aka 2021 edition

# Foreword

- Rust empowers you to reach farther, to program with confidence in a wider variety of domains then did before

- Not only low-level systems programming, ergonomic enough to make CLI apps, web servers, web app, Raspberry Pi, and many other kinds of code quite pleasant to write

# Introduction

## Who Rust Is For

### Teams of Developers

Rust is proving to be a productive tools for collaborating among large teams of developer

- Cargo, dependency manager and build tool
- Rustfmt, formatting tools to ensure a consistent coding style
- The Rust Language Server for IDE integrations for code completion and iline error message

### Students

- For students who are interested in learning about system concepts

- The community is very welcoming and happy to answer student questions

### Companies

- Hundreds of companies, large and small, use Rust in production for a variety of tasks

- CLI tools, web services, DevOps tooling, embedded devices, audio and video analysis and transcoding, cryptocurrencies, bioinformatics, search engines, Internet of Things applications, machine learning, and even major parts of the Firefox web browser

### Open Source Developers

- For people who want to build the Rust programming language, community, developer tools, and libraries

### People Who Value Speed and Stability

- For people who crave speed and stability in a language

- The Rust compiler’s checks ensure stability through feature additions and refactoring

- By striving for zero-cost abstractions, higher-level features that compile to lower-level code as fast as code written manually

- Rust’s greatest ambition is to eliminate the trade-offs that programmers have accepted for decades by providing safety and productivity, speed and ergonomics

# Getting Started

## Installation

- Run `curl https://sh.rustup.rs -sSf | sh` to download `rustup`

  - `rustup`: command line tool for managing Rust version and tools

- Add the following line to your env `export PATH="$HOME/.cargo/bin:$PATH"`

- If installation is already done, run `rustup update`

  - To uninstall, `rustup self uninstall`

- On macOS, we need xcode command line tools by using `xcode-select --install`

## Hello, World!

- Create a project directory

```sh
mkdir hello_world
cd hello_world
```

- New open the main.rs file then save with belows:

```rust
// main.rs
fn main() {
  println!("Hello, world!"); // `!` is macro. end with ';'
}
```

- Compile main.rs with `rustc`

```sh
$ rustc main.rs
$ ./main
```

## Hello, Cargo

- Cargo is Rust's build system and package manager

- `cargo new $PROJECT_NAME` to create a project

  - `-—bin` extra option for executable binary
  - `-—lib` extra option for library

- carg ogenerate Cargo.toml in TOML format

```rust
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

- Building and running a Cargo projects

```sh
# building
$ cargo build

# running
$ ./target/debug/hello_cargo
or
$ cargo run

# checking before build
cargo check

# update crates
cargo update
```

# Programming a Guessing Game

Hand-on project, you will lean about let, match, methods, asscicated function, external crates and more

## Setting Up a New Project

- Create a new project and run

```sh
$ cargo new guessing_game --bin
$ cargo ru
```

# Processing a Guess

```rust
use std::io;
use std::cmp::Ordering;
// use external crate rand, see Cargo.toml
use rand::Rng;

fn main() {
    // print message
    println!("Guess the number!");

    // gen_range is de
    let secret_number = rand::thread_rng().gen_range(1..=101);

    println!("the serect number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // immutable variable to save user input
        let mut guess = String::new();

        io::stdin()
            // read stdio buffer and write it to guess
            .read_line(&mut guess)
            .expect("Failed to read line");

        // handling invalid input
        let guess: u32 = match guess.trim().parse() {
            // convert to u32
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // match expression to compare guess and secret_number
        match guess.cmp(&secret_number) {
            // Ordering type is another enum, Less, Greater, Equal
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // quite the loop
                break;
            }
        }
    }
}
```

# Common Programming Concepts

> This chapter covers concepts that appear in almost every programming language and how they work in Rust.

## Variables and Mutability

- By default, variables are immutable
- Using `mut` to make a variable as mutable

## Differences Between Variable and Constants

- No `mut` for constants
- Constants can be declared in any scope
- Set with constant expression

## Shadowing

- Without `let` we will get compile error

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    // (5 + 1) * 2 = 12
    println!("The value of x is: {}", x);
}
```

- We can change the type, reuse the same name

```rust
// reuse the variable with the another type
let spaces = "   "; // string type
let spaces = spaces.len(); // bind new type value

// compile error with using mut
let mut spaces = "   ";
spaces = spaces.len();
```

## Data Types

### Scalar Types

Single value: integers, floating-point numbers, Booleans, and characters

- Integers Types: i/u8, i/u16, i/u32, i/u64, i/u128, i/usize
- Intergers literals:
  - Decimal: `98_222` // 98.222
  - Hex: 0xff
  - Octal: 0o77
  - Binary?: `0b1111_0000` // 11110000
  - Byte(u8):` b'A'`
  - u8: `57u8`
- Overflow, in debug, it panic, in production, wrapping
- Floating-Point: f64, default, same speeed as f32, double precision. f32, single precision
- Numberic Operations: `+, -, *, / , %`
- The Boolean Type: `true, false`
- The Character Type: unicode scalar values range from U+0000 to u+D7FFF and U+E000 to U+10FFFF
  - `char`
  - "string literal"

### Compound Types

- Group multiple values into one type, tuple and arrays
- The Tuple Type, fixed length, once declared, cannot grow or shirnk in size

```rust
// decalration
let tup: (i32, f64, u8) = (500, 6.4, 1);

// access with index
let five_hundred = tup.0; // start with 0
let six_point_four = tup.1;
let one = tup.2;

// destructure
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
```

- The Array Type, collection of multiple values with the same type, fixed length unlike other languages.

```rust
// declaration
let a = [1, 2, 3, 4, 5];
let b: [i32; 5] = [1, 2, 3, 4, 5];

// accessing
let first = a[0];

// out of indexing, rust protects this kind of error from memory corruption
let index = 10;
let run_time_error = a[index];
```

## Functions

- Using snake case
- Defining function anywhere

### Parameters

```rust
fn another_function(x) {
  println!("The value of x is: {}", x);
}

fn another_function(y: i32) {
  println!("The value of y is: {}", y);
}
```

### Statements and Expressions

```rust

fn statement_fn() {
  // Statements are instructions that perform some action and do not return a value
  println!("function!");
}

fn main() {
  let x = 5;
  let y = { // Expressions evaluate to a resultant value
    let x = 3;
    x + 1 // no semicolun at the end of the expression
  }

  println!("y is {}", y); // y is 4
}
```

### Functions with Return Values

```rust
fn five() -> i32 {
  5 // end of expression without return
}

fn plus_one(x: i32) -> i32 {
  x + 1
}
```

## Comments

- Commenting with two slashes and continue until the end of line

## Control Flow

### if expression

```rust
let number = 3;

if number < 5 {
  println!("number is under 5");
} else {
  println!("number isnot under 5");
}

// error, missmatch type, expected `bool`, found integer
// must be `number != 0`
if number {
  ...
}

// handling multiple if
if number % 4 == 0 {
  println!("number is divisible by 4");
} else if number % 3 == 0 {
  println!("number is divisible by 3");
} else if number % 2 == 0 {
  println!("number is divisible by 2");
} else {
  println!("number is not divisible by 4, 3, or 2");
}

// if in a let statement
let condition = true;
// must be return same type variable
let number = if confition { 5 } else { 6 };
```

### Repeatition with Loops

- Returning Values from Loops

```rust
fn main() {
  let mut counter = 0;
  let result = loop { // loop
    counter += 1;

    println!("again!");

    if counter == 10 {
      // return value from loop
      break counter * 2 // return 10 * 2
    }
  }

   println!("The result is {result}"); // The result is 20
}
```

- Loop Labels to Disambiguate Between Multiple Loops

```rust
fn main() {
  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        // break parent loop, 
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");
}
```

- Contitional Loops with while

```rust
fn main() {
  let mut number = 3;
  while number != 0 {
    println!("again!");
    number = number - 1;
  }
}
```

- Looping Through a Collection with for

```rust
fn main() {
  let a = [10, 20, 30, 40, 50];
  for element in a {
    println!("val is {}", element);
  }

  for number in (1..4).rev() {
    println!("val is {}", number);
  }
}
```

# Understanding Ownership

- Enables Rust to make memory safety guarantees without needing a `garbage collector`.
- In Ruet, memory is managed through a system of ownership `with a set of rules that compiler checks at compile time`

## What is Ownership?

- Ownership is a set of rules that govern how a Rust program manages memory
- Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks

### Ownership Rules

- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dopped

### Variable Scope

```rust
{                     // s is not valid here, it's not yet declared
  let s = "hello";    // s is valid from this point forward
  // do stuff with s
}                     // this scope is over, and s is no longer valid
```

### The String Type

> Rust has a second string type, String. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time

```rust
let s = String::from("hello"); // string is allocated on the heap, String:: is namespace

let mut s2 = String::from("hello");
s2.push_str(", world!");
```

### Memory and Allocation

- Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope
- When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`
- Rust calls `drop` automatically at the closing curly bracket

```rust
{
  let s = String::from("hello"); // s is valid from this point forward
  ...
} // this scope is now over, s goes out of scope, and s is no longer valid
```

### Ways Variables and Data Interact: Move

```rust
// these two 5 values are pused to onto the stack
let x = 5;
let y = x;

// two of containers for the string data (ptr, len, and capacity)
let s1 = String::from("hello");
let s2 = s1; // s1 string data is moved to s2, Rust invalidates the first variable, instead of being called a shallow copy

println!("{}, world!", s1); // error, string data has been moved to s2
```

- Rust will never automatically create “deep” copies of your data. Therefore, `any automatic copying` can be assumed to be inexpensive in terms of runtime performance.

### Ways Variables and Data Interact: Clone

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // using deep copy to call clone()

println!("s1 = {}, s2 = {}", s1, s2);
```

### Stack-Only Data: Copy

- Types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.

```rust
let x = 5;
let y = x; // x is stil valid after x moved to y

println!("x = {}, y = {}", x, y)
```

- Rust has a `special annotation called the Copy` trait that we can place on types that are stored on the stack, as integers are
- Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the `Drop trait`.
- Any group of simple scalar values can implement `Copy`, some of the types that implement Copy:
  - All the integer types, such as `u32`
  - The Boolean type `bool`, with values `true` and `false`
  - All the floating point type, such as `f64`
  - The character type `char`
  - Tuples, if they only contain types that are also `Copy`. For example, `(i32, i32)` is `Copy` but `(i32, string)` is not

### Ownership and Function

- Passing a variable to a function will `move or copy`, just as assgiment does

```rust
fn main() {
  let s = String::from("hello");    // s comes into scope
  takes_ownership(s);               // s's value moves into the function
                                    // s is no longer valid

  let x = 5;                        // x comes into scope

  makes_copy(x);                    // x would move into the function
                                    // but i32 is Copy, so it's ok to use x afterward
}

fn takes_ownership(some_string: String) { // some_string comes into scope
  println!("{}", some_string);
} // some_string goes out of scope and `drop`, `the backing memory is freed`

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("{}", some_integer);
} // some_integer goes out of scope. `Nothing special happens.`
```

### Return Values and Scope

- Returning values can also transfer ownership

```rust
fn main() {
  let s1 = gives_ownership();               // gives_ownership moves its return value into s1

  let s2 = String::from("hello");           // s2 comes into scope

  let s3 = takes_and_gives_back(s2);        // s2 is moved into takes_and_gives_back
                                            // takes_and_gives_back moves its return value into s3
} // s3 goes out of scope, and is dropped
  // s2 goes out of scope, but was moved, so nothing happens
  // s1 goes out of scope, and it dropped

fn gives_ownership() -> String {            // gives_ownership will move its return value into the function that calls it
  let some_string = String::from("hello");  // some_string comes into scope

  some_string                               // some_string is returned and moves out to the calling function
}

fn takes_and_givs_back(a_string: String) -> String { // a_string comes into scope
  a_string                                           // a_string is returned and moves out to the calling function
}
```

- The ownership of a variable follows the same pattern every time
  - Assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope
  - The value will be cleaned up by drop unless ownership of the data has been moved to another variable.

### Returning with Tuple

```rust
fn main() {
  let s1 = String::from("hello");

  let (s2, len) = calculate_length(s1);

  println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();

  (s, length)
}
```

## References and Borrowing

```rust
fn main() {
  let s1 = String::from("hello");         // s1 will not be dropped when the reference goes out of scope

  let len = calculate_length(&s1);        // s1 still have ownership because provide its as reference

  println!("{}, {}", s1, len);            // the tuple is stil valid
}

fn calulate_length(s: &String) -> usize { // s is a reference
  s.len()
} // goes out of scope, it refer to, it is not dropped because s doesn't have ownership
```

### Mutable References

```rust
fn main() {
  let mut s = String::from("hello");

  change(&mut s);                        // updatable reference
  cannot_change(&s);
}

fn canot_change(some_string: &String) {  // cannot borrow immutable borrowed content
  some_string.push_ptr(", world"); // ERROR!
}

fn change(some_string: &mut String) {
  some_string.push_ptr(", world");
}
```

#### Mutable refernces restriction

you can only have `one mutable reference` to a particular piece of data in particular scope. The benefit of having this restriction is that Rust can prevent data races at compile time. This code will fail:

```rust
let mut s = String:from("hello");

let r1 = &mut s;
let r2 = &mut s;                        // s cannot be borrowed more than once at a time

println!("{}, {}", r1, r2);             // first borrow later used here
```

#### Multiple mutable refernes by creating a new scope

```rust
let mut s = String:from("hello");

{
  let r1 = &mut s;
} // r1 goes out of scope here, now we can make a new reference

let r2 = &mut s;
```

#### Immutable references

Cannot have a mutable reference while we have an immutable one.

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem, immutable borrow occurs here
let r2 = &s; // no problem
let r3 = &mut s; // ERROR, mutable borrow occurs here
```

These scopes don’t overlap, so this code is allowed.

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```
### Dangling References

> dangling pointer—a pointer that references a location in memory that may have been given to someone else

In Rust, the compiler guarantees that references will never be dangling reference

```rust
fn main() {
  let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
  let s = String::from("hello"); // s is a new String in dangle

  &s // ERROR! missing lifetime specifier
} // s goes out of scope, and is dropped. It's memory goes away

fn no_dangle() -> String {
  let s = String::from("hello");

  s
}
```

## The Slice Type

- Slice let you reference a contiguous sequence of elements in a colleaction rather than the whole collection
- Slice doesn't have ownership

```rust
fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  // use reference
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  s.len()
}

fn main() {
  let mut s = String::from("hello world");

  let word = first_word(&s); // word will get the value 5

  s.clear(); // empties the String, making it euqal to ""

  // word still has the value 5 here, but there's no more string that
  // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

### String Slices

- String slice is a `reference` to part of a String

```rust
let s = String::from("hello");

// with ..
let slice = &s[0..2];  // => hel
let slice = &s[..2];   // => hel

// drop the trailing number
let let = s.len();

let slice = &s[3..len]; // => o
let slice = &s[3..];    // => o

// both drop
let slice = &s[0..len]; // => hello
let slice = &s[..];     // => hello
```

- Rewrite of `first_word`

```rust
fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if (item == b' ') {
      return &s[0..i];
    }
  }

  &s[..]
}

// If we have an immutable reference to something, we `cannot` also take a mutable reference
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // immutable borrow occurs here

    s.clear(); // error!, mutable borrow occrus here

    println!("the first word is: {}", word); // word is still used here
}
```

#### String literals are slices

String literals being stored inside the binary.

```rust
let s = "Hello, world!"; // s is &str, slice pointing, immutable reference
```

#### String Slices as Paramters

We can pass string slice directly or slice of the entire String

```rust
fn first_word(s: &str) -> &str { // support &String and &str (deref)
  ...
}

fn main() {
  let my_string = String::from("hello world");

  // `first_word` works on slices of `String`s, whether partial or whole
  let word = first_word(&my_string[0..6]);
  let word = first_word(&my_string[..]);
  // `first_word` also works on references to `String`s, which are equivalent
  // to whole slices of `String`s
  let word = first_word(&my_string);

  let my_string_literal = "hello world";

  // `first_word` works on slices of string literals, whether partial or whole
  let word = first_word(&my_string_literal[0..6]);
  let word = first_word(&my_string_literal[..]);

  // Because string literals *are* string slices already,
  // this works too, without the slice syntax!
  let word = first_word(my_string_literal);
}
```

#### Other slices

The slice has the type `&[i32]`, works the same way as string slice do

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

# Using Structs to Structre Related Data

> Structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group

## Defining and Instantiation Structs

- The pieces of a struct can be different type, can be named

```rust
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}
```

- Create an instance by specifying concreat values

```rust
let user1 = User {
  email: String::from("someone@example.com"),
  username: String::from("someone"),
  active: true,
  sign_in_count: 1,
}
```

- Get value with `dot notation`. it the instance is mutable, we can change a value by using the dot notation
- Certain field as mutable is not allowed

```rust
let mut user1 = User {
  email: String::from("someone@example.com"),
  username: String::from("someone"),
  active: true,
  sign_in_count: 1,
}

user1.email = String::from("someone@example.com");
```

- Using builder function

```rust
// implicity return that new instance

fn build_user(email: String, username: String) -> User {
  User {
    email: email,
    username: username,
    active: true,
    sign_in_count: 1,
  }
}
```

### Using the Field Init Shorthand

```rust
fn build_user(email: String, username: String) -> User {
  email, // no repetition
  username,
  active: true,
  sign_in_count: 1,
}
```

### Creating instances from other instnace with update value, or struct update syntax

```rust
// without update syntax
let user2 = User {
  email: String::from("another@example.com"),
  username: String::from("another"),
  active: user1.active,
  sign_in_count: user1.sign_in_count
};

// using struct update syntax `..`
let user2 = User {
  email: String::from("another@example.com"),
  username: String::from("another"),
  ..user1
}
```

### Using Tuple Structs Without Named Fields to Create Different Types

```rust
struct Color(i32, i32, i32); // Tuple structs
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

### Unit-like Structs without Any Field

```rust
struct AlwaysEqual;

fn main() {
  let subject = AlwaysEqual;
}
```

## An Example Program Using Struct

```rust
// sample code with multiple arguments
fn area(width: u32, height: u32) -> u32 {
  width * height
}

area(width, height);
```

### Refactoring with Tuples

```rust
fn area(dimensions: (u32, u32)) -> u32 {
  return dimensions.0 * dimensions.1
}

area((30, 50));
```

### Refactoring with Structs: Adding More Meaning

```rust
// with struct
struct Rectangle {
  width: u32,
  height: u32
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

let rect = Rectangle { width: 30, height: 50 };

area(&rect);
```

### Adding Useful Functionality with Derived Traits

```rust
#[derive(Debug)] // define outer attribute
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rect1 = Rectangle { width: 30, height: 50 };
  println!("rect 1 is {:?}", rect1); // {} doesn't work because of lack of Display implement


  let rect2 = Rectangle {
    width: dbg!(30 * scale),  // dbg! macro to prints the stderr
    height: 50
  };

  dbg!(&rect2);
}
```

## Method Syntax

> Methods are similar to functions, methods are different from functions in that they're defined within the context of a struct

### Defining Methods

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

// starts with impl
impl Rectangle {
  // &self instead of rectangle: &Rectangle
  // `&mut self`, to take an ownership
  fn area(&self) -> u32 {
    self.width * self.height
  }

  // same name as one of the struct's fields
  fn width(&self) -> bool {
    self.width > 0
  }
}

fn main() {
  let rect1 = Rectangle { width: 30, height: 50 };
  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  )
}
```

### Where’s the -> Operator?

```rust
// Rust has a feature called automatic referencing and dereferencing, automatically adds in `&, &mut or *` instead of using `->`. following are same
p1.distance(&p2);
(&p1).distance(&p2);
```

### Methods with More Parameters

```rust
impl Rectangle {
  fn area(&self) -> u32 {...}
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

...

rect1.can_hold(&rect2);
```

### Associated Functions

```rust
impl Rectangle {
  // Assoociated functions, without `self` parameter, still functions, not methods
  fn square(size: u32) -> Rectangle {
    // Often used for constructor that will return a new instance of the struct
    Rectangle { width: size, height: size }
  }
}

let sq = Rectangle::square(3);
```

### Multiple impl Blocks

```rust
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}
```

# Enums and Pattern Matching

## Defining an Enum

````rust
// define enum without type
enum IpAddrKind {
  V4,
  V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

fn route(ip_type: IpAddrKind) {...}

struct IpAddr {
  kind: IpADdrKind,
  address: String,
}

let home = IpAddr {
  kind: IpAddrKind::V4,
  address: String::from("127.0.0.1"),
}

let loopback = IpAddr {
  kind: IpAddrKind::V6,
  address: String::from("::1"),
}

// define enum value associated String
enum IpAddr {
  V4(String),
  V6(String),
}

let home = IpAddr::V4(String::From("127.0.0.1"));
let loopback = IpAddr::V6(String::From("::1"));

// define enum with different types and amounts of associated data
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

let home = IpAddr::V4(String::From(127, 0, 0, 1));
let loopback = IpAddr::V6(String::From("::1"));

// define enum embedded struct as associated type
struct Ipv4Addr {
  ...
}

struct Ipv6Addr {
  ...
}

enum IpAddr {
  V4(Ipv4Addr),
  V6(Ipv6Addr)
}

// define enum with variant
enum Message {
  Quit,
  Move { x:i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32)
}

// define method using impl
impl Message {
  fn call(&self) {
    ...
  }
}

let m = Message.Write(String::from("hello"));
m.call();
````

## The `Option` Enum and Its Advantages Over Null Values

- Rust does not have nulls, but it does have an enum that being present of absent
- Another enum, compiler can check whether you've handled all the cases
- Option<T> is defined by the standard library
- Option value, `Some`, contains a value, or `None`, does not

```rust
// Option is defined by the standard libaray, included in the prelude
enum Option<T> {
  None,
  Some(T),
}

// hold number type
let some_number = Some(5);

// hold string type
let some_string = Some("a string")

// hold none value
let absent_number: Option<i32> = None;

let x: i8 = 5;
let y: Options<i8> = Some(5);

let sum = x + y; // error, they are different type, i8 + std::option:Option<i8>
```

### [Use Some Value with various method](https://doc.rust-lang.org/std/option/enum.Option.html)

```rust
let mut x = Some(2);

match x.as_mut() {
    Some(v) => *v = 42, // matched
    None => {},
}

assert_eq!(x, Some(42)); // valid

let x = Some("value");
assert_eq!(x.expect("the world is ending"), "value"); // valid, expect return some value, but if some value is None? panics with "the world is ending"

let x: Option<u32> = Some(2);
assert_eq!(x.is_some(), true); // valid

let x: Option<u32> = None;
assert_eq!(x.is_some(), false); // valid

let x: Option<u32> = Some(2);
assert_eq!(x.is_none(), false); // valid

let x: Option<u32> = None;
assert_eq!(x.is_none(), true); // valid
```

### Some Value usecases

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// The return value of the function is an option
let result = divide(2.0, 3.0);

// Pattern match to retrieve the value
match result {
    // The division was valid
    Some(x) => println!("Result: {}", x), // 0.6666666666666666
    // The division was invalid
    None    => println!("Cannot divide by 0"),
}
```

## The `match` Control Flow Operator

- Basic usese

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

- Run multiple lines of code in a match arm

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
           println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Patterns That Bind to Values

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
  Alabama,
  Alaska,
  // --snip--
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    // match enum and return function
    Coin::Peny => {
      println!("Lucky penny!");
      1
    }
    // match enum and return value
    Coin::Nickel => 5,
    Coin::Dime => 10,
    // patterns that bind to values, value_in_cents(Coin::Quarter(UsState::Alaska))
    Coin::Quater(state) => {
      println("State quater from{:?}!", state);
      25
    },
  }
}
```

### Matching with `Option<T>`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

let five = Some(5);
let six = plus_one(five); // returns 6
let none = plus_one(None); // No value to add to, stops and return `None`
```

### Matches Are Exhaustive

The arms’ patterns must cover all possibilities

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    Some(i) => Some(i + 1), // Error, didn't cover every possible case, `Pattern 'None` not covered
  }
}
```

### Catch-all Patterns and the _ Placeholder

Use this when we don't want to list all possible values. `_` pattern will match any value.

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),cases
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {} 
}
```

We can express that by using the empty tuple type as the code that goes with the _ arm

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

## Concise Control FLow with `if let`

- `if let` syntax lets you combine `if` and `let` into a less verbos way to handle value that match one pattern while ignoring the rest

### Match one pattern

`if let` is `syntax sugar` for a match that runs code when the value matches one pattern

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
  Some(3) => println!("three"),
  _ => (),
}

// only work woth Some(3), using if let instead of

if let Some(3) = some_u8_value {
  println!("three");
}
```

### With else

```rust
fn main() {
  let mut count = 0;
  if let Coin::Quater(state) = coin {
    println!("State quater from {:?}!", state);
  } else {
    count + 1;
  }
}
```

# Managing Growing Projects with Packages, Crates, and Modules

As you write large programs, organizing your code will become increasingly important. By grouping related functionality and separating code with distinct features, you’ll clarify where to find code that implements a particular feature and where to go to change how a feature works.

- Packages: A Cargo feature that let you build, test, and share crates
- Crates: A tree of modules that produce a library or executable
- Modules and the use: Let you control the organization, scope, and privacy of path
- Paths: A way of naming an item such as a struct function, or module

## Packages and Crates

- crate: smallest amount of code that Rust compiler consider at a time
  - binary crate:
  - programs you can compile to an executable that you can run
  - library crate:
  - don't have main
  - don't compile to an executable
  - shared with multiple pojects
  - Rustaceans says `crate`, they mean library crate
- package: a bundle of one or more crates that provides a set of functionality
  - contains a Cargo.toml that describe how to build one or more crates
  - Cargo, is binary crate for build, also contgains a library crate
  - can contain as many binary crate
  - must contain at least one crate

```rust
// package
.
├── Cargo.toml
├── bin // package can have multiple binary crates
└── src
    ├── lib.rs // crate root, it can be library or binary
    └── main.rs // crate root, it can be binary
```

## Defining Modules to Control Scope and Privacy

### Modules Cheat Sheet

- How module work
  - Start from the crate root: when compiling a crate, the compiler first looks in the crate root file `src/main.rs` or `src/lib.rs`
  - Declaring modules: as `mod garden` declared, the compiler will look for module's code in these places: Inline > `src/garden.rs` > `src/garden/mod.rs`
  - Declaring submodules: as `mod vegetables` decalred in `src/garden.rs`, the compiler will look for the submodule's code within the directory named for the parent module in these places: Inline > `src/garden/vegetables.rs` > `src/garden/vegetables/mod.rs`
  - Plath to code in modules: Once a module is part of your crate, you can refer to code. For example, `Asparagus` type in the garden vegetable module would be found at `crate::garden::vegetables::Asparagus`
  - Private vs Public: Private from its parent modules by default, use `pub mod` to make a module public
  - The use keyword: Within a scope, the `use` keyword creates shortcuts to item to reduce repetion of long paths. `use crate::garden::vegetables::Asparagus;` make you only need to write just `Asparagus`
- The crate’s directory, also named backyard, contains these files and directories:

```rust
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

- The crate root file in this case is src/main.rs, and it contains:

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

- The `pub mod garden;` line tells the compiler to include the code it finds in `src/garden.rs`


```rust
pub mod vegetables;
```

- `pub mod vegetables;` means the code in `src/garden/vegetables.rs` is included too

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

### Grouping Related Code in Modules

```rust
mod front_of_house {
  mod hosting {
    fn add_to_waitlist() {}
    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}
    fn server_order() {}
    fn take_payment() {}
  }
}
```

here is `module tree`. `src/main.rs` and `src/lib.rs` are called crate roots

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### Paths for Referring to an Item in the Module Tree

- `An absolute path` is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
- `A relative path` starts from the current module and uses self, super, or an identifier in the current module.
- Both absolute and relative paths are followed by one or more identifiers separated by double colons `(::)`

```rust
mod front_of_house {
  mod hosting {
      fn add_to_waitlist() {}
  }
}

pub fn eat_at_restaurant() {
  // Absolute path
  crate::front_of_house::hosting::add_to_waitlist(); // error, module `hosting` is private

  // Relative path
  front_of_house::hosting::add_to_waitlist();
}
```

- Module are the privacy boundary in Rust
- All items (functions, methods, structs, enums, modules and constants) are private by default
- `pub` keyword to make an item public
- Items in a parent module `can’t use` the private items `inside child modules`, but items in child modules `can us`e the items in `their ancestor modules`.

```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

pub fn eat_at_restaurant() {
  // Absolute path
  crate::front_of_house::hosting::add_to_waitlist(); 

  // Relative path
  front_of_house::hosting::add_to_waitlist();
}
```

### Starting Relative Paths with `super`

- `super`, is like starting a file system path with `...`
- in case of `super` is root

```rust
fn deliver_order() {}

mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
  }

  fn cook_order() {}
}
```

### Making Structs and Enums Public

- `pub` before a struct definition, we make the struct public

```rust
mod back_of_house {
  // pub to make public struct
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    // If Breakfast didn’t have such a function, we couldn’t create an instance of
    // Breakfast in eat_at_restaurant because we couldn’t set the value of the private
    // seasonal_fruit field in eat_at_restaurant.
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }
}

pub fn eat_at_restaurant() {
  // Order a breakfast in the summer with Rye toast
  let mut meal = back_of_house::Breakfast::summer("Rye");
  // Change our mind about what bread we'd like
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // The next line won't compile if we uncomment it; we're not allowed
  // to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");
}
```

- if we make an enum public, all of its variants are then public

```rust
mod menu {
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

fn main() {
  let order1 = menu::Appetizer::Soup;
  let order2 = menu::Appetizer::Salad;
}
```

## Bringing Paths into Scope with the use Keyword

- Create a shortcut to a path with the `use` keyword once, and then use the shorter name everywhere else in the scope
- Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope

```rust
mod front_of_house {
  pub mod hosting {
      pub fn add_to_waitlist() {}
  }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
}
```

- `use` only creates the shortcut for the particular scope in which the `use` occurs

```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

use crate::front_of_house::hosting;

mod customer {
  pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // error, failed to resolve: use of undeclared crate or module `hosting`
  }
}
```

### Creating Idiomatic use Paths

- Specifying the parent module when calling the function makes it clear that the function isn’t locally defined

```rust
mod front_of_house {
  pub mod hosting {
      pub fn add_to_waitlist() {}
  }
}

use crate::front_of_house::hosting::add_to_waitlist; // make it confusing between local defined function

pub fn eat_at_restaurant() {
  add_to_waitlist();
}
```

- On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

- There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.
- If we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

### Providing New Names with the as Keyword

- Same name into the same scope with use: after the path, we can specify as and a new local name, or alias, for the type

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### Re-exporting Names with pub use

- Bring a name into scope with the use keyword, the name available in the new scope is private.
- To `re-exporting`, combine `pub` and `use`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

### Using External Packages

- Add package to `Cargo.toml` we want to use. for example `rand`. 
- Cargo will down load `rand` package
- Any dependencies from crates.io and make rand available to our project.

```toml
rand = "0.8.5"
```

- Use line starting with the name of the crate, rand, and listed the items we wanted to bring into scope

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

- Standard std library is also a crate that’s external to our package.
- Because the standard library is shipped with the Rust language, we don’t need to change Cargo.toml to include std
- But we do need to refer to it with `use` to bring items from there into our package’s scope

```rust
use std::collections::HashMap;
```

### Using Nested Paths to Clean Up Large use Lists

- Use curly brackets after two colons to bring the same items into scope in one line. 

```rust
use std::cmp::Ordering;
use std::io;

// use ::{,} to bring itmes
use std::{cmp::Ordering, io};
```

- We can use a nested path at any level in a path, which is useful when combining two use statements that share a subpath

```rust
use std::io;
use std::io::Write;

// use `self` in the nested path
use std::{self, Write}
```

### The Glob Operator

- To bring all public items defined in a path into scope

```rust
use std::collections::*;
```

## Separating Modules into Different Files

- We’ll extract modules into files instead of having all the modules defined in the crate root file. see below

```
src
 └── lib.rs
 └── front_of_house
     └── hosting.rs
```

- `src/lib.rs` decalre `mod front_of_house` at the begin of the file

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
}
```

- `src/front_of_house.rs` decalre `mod hositing` and makes it as public

```rust
pub mod hosting;
```

- `src/front_of_house/hosting.rs` contains the definitions

```rust
pub fn add_to_waitlist() {}
```

- alternate File Paths
  - src/front_of_house.rs (what we covered)
  - src/front_of_house/mod.rs (`older style`, still supported path)

# Common Collections

Rust’s standard library includes a number of very useful data structures called collections. Collections can contain multiple values. Unlike the built-in array and tuple types

- vector: allows you to store a variable number of values next to each other
- string: collection of characters
- hash map: allows you to associate a value with a particular key

## Storing Lists of Values with Vectors

### Creating a New Vector

> [samples](./5-collections/src/main.rs)

```rust
let v: Vec<i32> = Vec::new(); // added a type annotation

let v = vec![1, 2, 3]; // use vec! macro
```

### Update a Vector

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

### Reading Elements of Vectors

- Two ways to reference a value. indexing[] and `get`
- Out of index: [] will cause the panic, `get` method returns `None` without panicking

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&32> = v.get(2);
match third {
  Some(third) => println!("The third element is {third}");
  None => println!("There is no thiid element.");
}
```

- Cannot have mutable and immutable references in the same scope

```rust
let mut v = vec![1, 2, 3, 4];
let first = &v[0]; // immutable borrow
v.push(6); // mutable borrow, vector might require allocating new memory and copy the old elms
println!("fist is {}", first) // immutable borrow used here
```

### Iterating over the values in a Vector

> [samples](./5-collections/src/main.rs)

```rust
let v = vec![100, 32, 57];
for i in &v {
  println!("{i}");
}

// with mutable references
let mut v = vec![100, 32, 57];
for i in &mut v {
  *i + 50;  // use * dereference operator to get to the value
}
```

### Using an enum to store multiple types

- Vector can only store values that are the sample type. This can be inconvenient. To store different use enum
- If you don’t know the exhaustive set of types a program, enum doesn't work, use Trait instead of

```rust
// this enum has multiple types
enum SpreadsheetCell {
  Int(32),
  Float(f64),
  Text(String)
}

// holds different types
let row = vec![
  SpreadsheetCell::Int(3),
  SpreadsheetCell::Text(String::from("blue"))
  SpreadsheetCell::Float(10.12)
]
```

### Dropping a Vector Drops Its Elements

- Vector is freed when it goes out of scope

```rust
{
  let v = vec![1, 2, 3, 4];
  // do stuff with v
} // <- v goes out of scope and is free here
```

## Storing UTF-8 Encoded Text With Strings

### What Is a String?

- String is only one type in the core language in Rust, `str`
  - which is the string slice, usually seen in its borrowed
- String are implemented as a collection of bytes
- String, gowable, mutable, owned, UTF-8 encoded type

### Creating a New String

- Many of some operations available with Vec<T>
- String is actually impemented as a wrapper around a vector of bytes with some extra gurantees restrictions and capbilities

```rust
// create a new empty string with the new function
let mut s = String::new();

// create a string containing `string`
let data = "initial contents"
let s = data.to_string();

// create a string from literal directly
let s = "initial contents".to_string();

// create a string from a string literal same as to_string
let s = String::from("initial content");
```

- Strings are UTF-8 encoded

```rust
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

### Updading a String

> String can grow in size and its contents can change same as Vec<T>

#### Appending to a String with push_str and push

```rust
let mut s = String::from("foo");
s.push_str("bar"); // method takes a string slice

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2); 
println!("s2 is {s2}"); // s2 ownership still alive

// single character
s.push('l');
```

#### Concatenation with the + Operator or the format! Macro

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

// note s1 has been moved here and can no longer be used
let s3 = s1 + &s2;  // 1. s1 will call `add(self, s:&str) -> String`
                    // 2. &s2 will be `deref coercion`, &s2 -> &s2[..]
                    // 3. `add` keep `s`'s ownership. s2  is still valid
                    // 4. `add` takes ownership of self, s1 will be moved into the `add`
                    // 5. s1 will no longer be valid after this operation
                    // 6. append a copy of the content of s2
                    // 7. return ownership of the result

// format! for multiple concatenate
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

### Indexing into Strings

- Rust doesn’t allow us to index into a String to get a character is that indexing

```rust
//  operations are expected to always take constant time (O(1)).
let s1 = String::from
let h = s1[0]; // `String` error cannot be indexed by {integer}
```

#### Internal Representation

```rust
let hello = String::From("hola").len(); // 4 byte, take 1 byte for UTF-8
let hello = String::from("Здравствуйте").len(); // 24 byte, take 2 byte for UTF-8
let hello = "Здравствуйте";
let answer = &hello[0]; // return 208 not '3', which is first byte
```

#### Bytes and Scalar Values and Grapheme Clusters! Oh My!

- UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as `bytes`, `scalar values`, and `grapheme clusters` (letters)
- How “नमस्ते” (Devanagari scriptis) show in Rust's perspectives

```rust
// bytes (u8)
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

// unicode scalar values (char, but 4 and 6 are not letters)
['न', 'म', 'स', '्', 'त', 'े']

// grapheme clusters, we'd get what a person would call the four `letters`
["न", "म", "स्", "ते"]
```

- Rust priovidess different ways of interpreting the raw string data which program can choose the interpretation it needs
- Rust doesn't allow us to index into a string in constant time o(1) because Rust would  have to walk through the content from the begnning to the index to determine how many valid charaters there where

### Slicing Strings

- Rust asks you to be more specific via [] with a range to creat strin slice

```rust
let hello = "Здравствуйте";
let s = &hello[0..4]; // 4byte, will be Зд
let s = &hello[0..1]; // panic, index is not a char boundary
```

### Methods for Iterating Over Strings

- Use `chars` for individual Unicode scalar values

```rust
for c in "Зд".chars() {
  println!("{c}");
}

// result
З
д

for c in "Зд".bytes() {
  println!("{}", c);
}

// result
208
151
208
180
```

### Strings Are Not So Simple

- To Summarize, string are complicated. Different programming languages make different choices about how to present this complexity to the programmer
- Rust has chosen to make the correct handling of String data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data upfront

## Storing Keys with Assoicated Values in Hash Maps

- `HashMap<K, V>` type stores a mapping of keys of type K to values of types V using `hashing function`
- `Key` can be of any type

### Creating a New Hash Map

- Vector, Hash map store data on the heap
- `insert` to add elements

```rust
use std::collection::Hashmap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

### Accessing Values in a Hash Map

- `get` a value with `key`

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores
  .get(&team_name) // reutrn `Option(&V)`, If no value?, will return `None`
  .copied() // call `copied` to get Option<i32> rather than an Option<&i32>
  .unwrap_or(0); // set score to zero if scores doesn't have an entry
```

- using `for` loop

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.inseret(String::from("Blue"), 10);
scores.inseret(String::from("Yellow"), 50);

for (key, value) in &scores {
  println!("{key}: {value}");
}
```

### Hash Maps and Ownership

- The values are copied into the hash map in case of types implemented the `Copy` trait, like i32
- A string value, like String, will be moved and hash map will be the owner

```rust
let field_name = String::from("Favorite color");
let field_value = String::from("Blue":);

let mut map = HashMap::new();
map.insert(field_name, filed_value); // values are copied into the hash
                                     // field_name and field_value are invalid at this point, try using them and
                                     // see what compiler error you get!
```

### Updating a Hash Map

> When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned

#### Overwriting a Value

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores); 

// result
{"Blue": 25}
```

#### Adding a Key and Value Only If a Key Isn’t Present

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// `or_insert` method on Entry is defined to return a mutable reference
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);

// result
{"Yellow": 50, "Blue": 10}
```

#### Updating a Value Based on the Old Value

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
  let count = map.entry(word).or_insert(0);
  *count += 1; // update old value
}

println!("{:?}", scores);

// result
 {"world": 2, "hello": 1, "wonderful": 1}
```

### Hashing Functions

- Uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it
- You can switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher trait

# Errror Handling

- Rust requires you to acknowledge the possibility of an error and take some action before your code will compile.
- Rust group errors into two types of categories:
  - recoverable: file not found error, Result<T, E>
  - unrecoverable: symptoms of bugs, accessing end of an array, panic!
- Rust doesn't have exceptions, manage Result<T, E> or get panic!

## Unrecoverable Erros With panic!

- When a panic occurs(or `panic!`), the program starts `unwinding` but it cleanup is a lot of work so Rust allow you to choose the alrernative of immediately aborting

```
[profile.release]
panic = 'abort'
```

## Using a panic! Backtrace

- Unlike C, Rust will stop execution and refuse to contitue in case of invalid index to protect your program from this sort of vulnerablity like `buffer overread`
- To show backtrace, `RUST_BACKTRACE=1 cargo run`
- To enable cargo symbol `cargo build, cargo run` without `--release`

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

## Recoverable Error with Result

- Opening file failed is that no need to terminate the process, we can create the file
- Function returl `Result` because function could fail

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

- When ssucceeds return Ok(T), failed, return Err(E)

```rust
use std:fs::File;

fn main() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => panic!("There was a problem opening file: {:?}", error),
  };
}
```

### Matching on Different Errors

- Take different actions for different failure reasons

```rust
use std:fs::File;

fn main() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) = fc,
        Err(e) = panic!("Tried to create file but there was a problem {:?}", e),
      },
      other_error => panic!("There was a problem opening file: {:?}", error)
    },
  };
}
```

### Alternatives to Using match with Result<T, E>

- Using closures and the `unwrap_or_else` without `match`

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let f = File.open("hello.txt").map_err(|error| {
    if (error.kind()) == ErrorKind.NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Tried to create file but there was a problem {:?}", e)
      })
    } else {
      panic!("There was a problem opening file: {:?}", error)
    }
  })
}
```

### Shortcuts for Panic on Error: unwrap and expect

- Using `match` works well but verbose
- `Result<T, E>` type has many helper methods defined
- `Ok` variant in `Result` reutn the value inside the `Ok`
- `Err` variant in `Result`, `unwrap` will call `panic!`

```rust
use std::fs::File;

fn main() {
 // panic! call without hello.txt
  let greeting_file = File::open("hello.txt").unwrap();
}
```

- `expect` lets us choose the `panic!` error message
- Using `expect` instead of `unwrap` and providing `good error message`

```rust
use std:fs:File;

fn main() {
  // unwarp, shortcut method of match, return Ok or Err
  let f = File::open("hello.txt").unwrap()

  // expect, let us choos the panic, return the fild handle or call panic! macro
  let f = File::open("hello.txt").expect("Failed to open hello.txt)
}
```

### Propagating Errors

- Return the error to the calling code instead of handling the error within the function itself

```rust
fn read_username_from_file() -> Rsult<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,        // return String
    Err(e) => return Err(e), // return error to the code that called the function
                             // use the return keyword to return early out of the
                             // function entirely and pass the error value from File::open
  }

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e), // return error
  }
}
```

#### A Shortcut for Propagating Errors: the ? Operator

- Uses the ? operator, placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

- Shorten the code futher by chaining mehtod call immediately after the `?`

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}
```

- More shorten the code by using `fs::read_to_string`

```rust
fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
```

#### Where The ? Operator Can Be Used

- `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on

```rust
fn main() { // main is incompatible with `Result` or `Option`
  let f = File::open("hello.txt"); // ^ cannot use the `?` operator in a function that returns `()`
}
```

- `?` can be used with `Option<T>` values
- The behavior of the ? operator when called on an Option<T> is similar to its behavior when called on a Result<T, E>
- If the value is None, the None will be returned early from the function at that point
- If the value is Some, the value inside the Some is the resulting value of the expression and the function continues

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

- Because it’s the entry and exit point of executable programs, and `there are restrictions` on what its return type can be for the programs to behave as expected.
  - Main can also return a Result<(), E>
  - You can read Box<dyn Error> to mean “any kind of error.” 
- Main function returns a Result<(), E>, the executable will exit with a value of 0 if main returns Ok(())
- Main function will exit with a nonzero value if main returns an Err value

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

## To panic! or Not ro panic!

- You could call panic! for any error situation, whether there’s a possible way to recover or not, but then you’re making the decision that a situation is unrecoverable on behalf of the calling code. When you choose to return a Result value, you give the calling code options

### Examples, Prototype Code, and Tests

When you’re writing an example to illustrate some concept, also including robust error-handling code can make the example less clear. In examples, it’s understood that a call to a method like unwrap that could panic is meant as a placeholder for the way you’d want your application to handle errors, which can differ based on what the rest of your code is doing.

### Cases in Which You Have More Information Than the Compiler

if you can ensure by manually inspecting the code that you’ll never have an Err variant, it’s perfectly acceptable to call unwrap, and even better to document the reason you think you’ll never have an Err variant in the expect text.

```rust
  use std::net::IpAddr;

  let home: IpAddr = "127.0.0.1"
      .parse()
      .expect("Hardcoded IP address should be valid");
```

### Guidelines for Error Handling

- Panic when it’s possible that your code could end up in a `bad state`
- The `bad state` is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong forma
- Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
- There’s not a good way to encode this information in the types you use

> If someone calls your code and passes in values that don’t make sense, it’s best to return an error if you can so the user of the library can decide what they want to do in that case. However, in cases where continuing could be insecure or harmful, the best choice might be to call panic! and alert the person using your library to the bug in their code so they can fix it during development. However, when failure is expected, it’s more appropriate to return a Result than to make a panic! call

### Creating Custom Types for Validation

- Parse the guess as an i32 instread of only a u32 to allow potentially negative numbers, and then add a check for the number being in range
- However, this is not an ideal solution: if it was absolutely critical that the program only operated on values between 1 and 100, and it had many functions with this requirement, having a check like this in every function would be tedious

```rust
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
    // --snip--
}

- If value doesn’t pass this test, we make a panic! call, which will alert the programmer who is writing the calling code that they have a bug they need to fix, because creating a Guess with a value outside this range would violate the contract that Guess::new is relying on
- Function that has a parameter or returns only numbers between 1 and 100 could then declare in its signature that it takes or returns a Guess rather than an i32 and wouldn’t need to do any additional checks in its body.

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

# Generic Types, Traits, and Lifetimes

- `generic` is tool for effectively handling the duplication of concepts in Rust
- Functions can take parameter of some generic type
- `traits` define behavior in a generic way which can combine with generic types to constrain a generic type to accept only those types that a particular behavior as opposed to just any type

## Removing Duplication by Extracting a Function

- We've now been tasked with finding the largest number in two different lists of numbers

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

- Identify duplicate code.
- Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
- Update the two instances of duplicated code to call the function instead.

```rust
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```

## Generic Data Types

### In Function Definitions

- Place the generics in the signature of the function where we would usually specify the data types of the parameters and return value

```rust
fn largest<T>(list: &[T]) -> &T {}
```
- We read this definition as: the function largest is `generic over some type T`. This function has one `parameter named list, which is a slice of values of type T`. The largest function will `return a reference to a value of the same type T`.

```rust
fn largest<T>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
    // but it would be error because of a lack of a trait in type T
    if item > largest {
      largest = item;
    }
  }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

### In Struct Definitions

- We can also define structs to use a generic type parameter in one or more fields using the <> syntax
- We can use multiple generic type parameters

```rust
struct Point<T, U> {
  x: T,
  y: U,
}

let work = Point { x: 5, y: 6 }
let wont_work = Point { x: 5, y: 6.0 }

struct Point<T. U> {
  x: T,
  y: U
}

let both_integer = Point { x: 5, y: 20 }
let both_float = Point { x: 5.0, y: 20.0 }
let integer_and_float = Point { x: 5, y: 20.0 }
```

### In Enum Definition

- We can define enums to hold generic data types in their variants
- Enums can use multiple generic types as well

```rust
enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

### In Method Definition

- We can implement methods on structs and enums and use generic types in their definitions, too
- We have to declare T just after impl so we can use T to specify that we’re implementing methods on the type Point<T>. By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type

```rust
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

//. implemention with f32
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

let p = Point{ x: 5, y: 10 };
println("p.x = {}", p.x())
```

- Generic type parameters in a struct definition `aren’t always the same as` those you use in that same struct’s method signatures

```rust
struct Point<X1, Y1> {
  x: X1,
  y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
  // mixup method dhas different generic type to Point
  fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

fn main() {
  let p1 = Point { x: 5, y: 10.4 };
  let p2 = Point { x: "Hello", y: 'c' };

  let p3 = p1.mixup(p2);

  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

### Performance of Code Using Generics

- Using generic types won't make your program run any slower than it would with concrete types
- Rust accomplishes this by performing monomorphization of the code using generics at compile time
  - Monomorphization is the process of turning generic ode into specific code by filling the concrete types that are used when compiled

```rust
// expand and replace the generic definition with the spedific ones
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

## Trais: Defining Shared Behavior

- `trait` defines functionality a particular type has and can `share with other types`
- We can use traits to define shared behavior in an abstract way
- We can use `trait bounds` to specify that a generic type can be any type that has certain behavior

### Defining a Trait

- Type’s behavior consists of the methods we can call on that type
- Different types share the same behavior
- Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose
- Decalare a trait with `trait` keyword, `pub` and decalre method signature in to currly brakets with a semicolon

```rust
pub trait Summary {
  fn summerize(&self) -> String;
}
```

### Implementing a Trait on a Type

- Implementing a trait on a type is similar to implementing regular methods. The difference is that after `impl`, we put the trait name we want to implement, then use the `for` keyword

```rust
// src/lib.rs
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}
```

- Users of the crate can call the trait methods on instances of NewsArticle and Tweet in the same way we call regular methods. The `only difference is that the user must bring the trait into scope` as well as the types

```rust
use aggregator::{Summary, Tweet}; // User must bring Summary trait into scope

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize()); // 1 new tweet: horse_ebooks: of course, as you probably already know, people
```

- Other crates that depend on the aggregator crate can also bring the Summary trait into scope to implement Summary on their own types
- One restriction to note is that we `can implement a trait on a type only if at least one of the trait or the type is local` to our crate
  - We can implement Display(std libary) on a custom type like Tweet as part of our aggregator crate functionality, because `the type Tweet is local` to our aggregator crate
  - We can also implement Summary on Vec<T> in our aggregator crate, because `the trait Summary is local` to our aggregator crate
   - we can’t implement `external traits on external types`, the Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are `both defined in the standard library and aren’t local` to our aggregator crate

### Default Implementations

- Sometimes it’s `useful to have default behavior for some or all of the methods` in a trait instead of requiring implementations for all methods on every type

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

let article = NewsArticle {
  headline: String::from("Penguins win the Stanley Cup Championship!"),
  location: String::from("Pittsburgh, PA, USA"),
  author: String::from("Iceburgh"),
  content: String::from(
    "The Pittsburgh Penguins once again are the best \
      hockey team in the NHL.",
  ),
};

println!("New article available! {}", article.summarize()); // New article available! (Read more...).
```

- Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation

```rust
pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
      format!("@{}", self.username)
  }
}

let tweet = Tweet {
  username: String::from("horse_ebooks"),
  content: String::from(
    "of course, as you probably already know, people",
  ),
  reply: false,
  retweet: false,
};

println!("1 new tweet: {}", tweet.summarize()); // 1 new tweet: (Read more from @horse_ebooks...)
```

### Traits as Parameters

- Calls the summarize method on its item parameter, which is of some type that implements the Summary trait. To do this, we use the `impl` Trait syntax

```rust
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}
```

#### Trait Bound Syntax

- `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form known as a `trait bound`

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

- The `impl Trait` syntax is convenient and makes for more concise code in simple cases, while the fuller trait bound syntax can express more complexity in other cases
- Using `impl Trait` is appropriate if we want this function to `allow item1 and item2 to have different types`

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
```

- If we `want to force both parameters to have the same type`, we must use a `trait bound`

```rust
pub fn notify<T: Summary>(item: T, item2: T) {}
```

#### Specifying Multiple Trait Bounds with the + Syntax

- We can also specify more than one trait bound. we can do so using the + syntax

```rust
// we specify in the notify definition that item must implement both Display and Summary 
pub fn notify(item: impl Summary + Display) {}
```

- The + syntax is also valid with trait bounds on generic types

```rust
pub fn notify<T: impl Summary + Display>(item: T) {}
```

#### Clearer Trait Bounds with where Clauses

- Using too many trait bounds has its downsides. Each generic has its own trait bounds,
  - Functions with multiple generic type parameters `can contain lots of trait bound information between the function’s name and its parameter list`
  - Making the function signature hard to read

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```

- Rust has alternate syntax for specifying trait bounds inside a `where` clause after the function signature
- This function’s signature is less cluttered: the function name, parameter list, and return type are close together, similar to a function without lots of trait bounds

```rust
fn some_function<T, U>(t:T, t:U) -> i32
  where T: Display + Clone,
        U: Clone + Debug {}
}
```

### Returning Types that Implement Traits

- We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait
- Using impl Summary for the return type, we specify that the returns_summarizable function `returns some type that implements the Summary trait`
- The ability to specify a return type only by the trait it implements is especially useful in the context of closures and iterators

```rust
fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  }
}
```

- However, you can only use impl Trait if you’re returning a single type. For example, this code that returns either a NewsArticle or a Tweet with the return type specified as impl Summary wouldn’t work
- Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how the impl Trait syntax is implemented in the compiler

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
  if switch {
    NewsArticle {
      headline: String::from(
        "Penguins win the Stanley Cup Championship!",
      ),
      location: String::from("Pittsburgh, PA, USA"),
      author: String::from("Iceburgh"),
      content: String::from(
        "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
      ),
    }
  } else {
    Tweet {
      username: String::from("horse_ebooks"),
      content: String::from(
        "of course, as you probably already know, people",
      ),
      reply: false,
      retweet: false,
    }
  }
}
```

### Using Trait Bounds to Conditionally Implement Methods

- Using a `trait bound with an impl block` that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits

```rust
use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

// trait bound with an impl block to implement method conditionally
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}
```

- We can also conditionally implement a trait for any type that implements another trait
- Implementations of a trait on any type that `satisfies the trait bounds` are called `blanket implementations` and are extensively used in the Rust standard library
```rust
impl<T: Display> ToString for T {}
```

- We can call the `to_string method defined by the ToString trait` on any type that implements the Display trait

```rust
// integers, 3 into their corresponding String values like this because integers implement Display
let s = 3.to_string();
```

## Validating References with Lifetimes

- Lifetimes are another kind of generic that we’ve already been using
- Lifetimes ensure that references are valid as long as we need them to be
- Lifetimes are the scope for which that reference is valid
- Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid

### Preventing Dangling References with Lifetimes

- The main aim of lifetimes is to prevent dangling references

```rust
{
  let r // with no ititial value
  {
    let x = 5;
    r = &x; // borrowed value does not live long enough
  }
  // - `x` dropped here while still borrowed
  println!("{}", r);
}
```

### The Borrow Checker

- The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid
- Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b. The program is rejected because 'b is shorter than 'a

```rust
{
  let r;                // ---------+-- 'a
                        //          |
  {                     //          |
    let x = 5;          // -+-- 'b  |
    r = &x;             //  |       |
  }                     // -+       |
                        //          |
  println!("r: {}", r); //          |
}                       // ---------+
```

- Fixes the code so it doesn’t have a dangling reference and compiles without any errors.

```rust
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
                          // ----------+
}
```

### Generic Lifetimes in Functions

- Return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to x or y
-  To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis

```rust
fn longest(x: &str, y: &str) -> &str { // expected named lifetime parameter
  if x.len() > y.len() {
    x
  } else {
      y
  }
}

let string1 = String::from("abcd");
let string2 = "xyz";

let result = longest(string1.as_str(), string2); // longest take a ownership
println!("The longest string is {}", result);
```

#### Lifetime Annotation Syntax

- The annotations are meant to tell Rust `how generic lifetime parameters of multiple references relate to each other`
- Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short

```rust
&i32           // a refernce
&'a i32        // a refernce with an explicit lifetime
&'a mut i32    // a mutable reference with an explicit lifetime
```

### Lifetime Annotations in Function Signatures

- To use lifetime annotations in function signatures, we need to declare the generic lifetime parameters inside angle brackets between the function name and the parameter list
- The function signature now tells Rust that for some lifetime 'a, the function `takes two parameters`, both of which are string slices that `live at least as long as lifetime 'a` 
- The function signature also tells Rust that the `string slice returned from the function` will `live at least as long as lifetime 'a`
- Specifying that the borrow checker should reject any values that `don’t adhere to these constraints`

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

}
```

- The generic lifetime 'a will `get the concrete lifetime that is equal to the smaller of the lifetimes` of x and y
- `result` references that is valid until the end of the inner scope

```rust
fn main() {
  let string1 = String::from("long string is long");
  {
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
  }
}
```

- The lifetime of the reference in `result` must be the smaller lifetime of the two arguments
- Rust knows this because we annotated the lifetimes of the function parameters and return values using the same lifetime parameter 'a

```rust
fn main() {
  let string1 = String::from("long string is long");
  let result;
  {
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
                                       ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
  }
  println!("The longest string is {}", result);
                                      ------ borrow later used here
}
```

### Thinking in Terms of Lifetimes

- We’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.


```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
  x
}
```

- If the reference returned does not refer to one of the parameters, it must refer to a value created within this function. However, this would be a dangling reference because the value will go out of scope at the end of the function

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
  let result = String::from("really long string");
  result.as_str()
  ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function
}
```

### Lifetime Annotations in Struct Definitions

- We can define structs to hold references, but in that case we `would need to add a lifetime annotation on every reference` in the struct’s definition

```rust
struct ImportantExcerpt<'a> {
  part: &'a str, // holds a string slice, which is a reference. As with generic data types
}

fn main() {
  let novel = String::from("call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.')
    .next()
    .expect("Cound not find a '.'")
  let i = ImportantExcerpt { part: first_sentence }; // can’t outlive the reference it holds in its part field.
}
```

### Lifetime Elision

> Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

- The patterns programmed into Rust’s analysis of references are called the lifetime elision rules. These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

```rust
fn first_word(s: &str) -> &str {}
```

- The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter

```rust
fn first_word<'a>(s: &'a str) -> &str {}
```

- The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters

```rust
fn first_word<'a>(s: &'a str) -> &'a str {}
```

- The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters

```rust
// You can see that the second rule doesn’t apply because there is more than one input lifetime. The third rule doesn’t apply either, because longest is a function rather than a method
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
```

### Lifetime Annotations in Method Definitions

- Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name
- We’re not required to annotate the lifetime of the reference to self because of the first elision rule.

```rust
impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {
    3
  }
}
```

- Example where the third lifetime elision rule applies
- There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for

```rust
impl<'a> ImportantExcerpt<'a> {
  fn announce_and_return_part(&self, announcement: &str) -> &str {
      println!("Attention please: {}", announcement);
      self.part
  }
}
```

### The Static Lifetime

- `'static`, which denotes that the affected reference can live for the entire duration of the program
- All string literals have the 'static lifetime, which we can annotate as follows:
- Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is fixing those problems, not specifying the 'static lifetime.

```rust
let s: &'static str = "I have a static lifetime.";
```

### Generic Type Parameters, Trait Bounds, and Lifetimes Together

- Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!
  - It has an extra parameter named ann of the generic type T, which can be filled in by any type that implements the Display trait as specified by the where clause. 
  - This extra parameter will be printed using {}, which is why the Display trait bound is necessary. 
  - Lifetimes are a type of generic, the declarations of the lifetime parameter 'a
  - The generic type parameter T go in the same list inside the angle brackets after the function name.

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
  x: &'a str,
  y: &'a str,
  ann: T,
) -> &'a str
where
  T: Display,
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
```

# Writing Automated Tests

## How to Write Tests

- The bodies of test functions typically perform these three actions:
    1. Set up any needed data or state.
    2. Run the code you want to test.
    3. Assert the results are what you expect.

### The Anatomy of a Test Function

- At its simplest, a test in Rust is a function that’s annotated with the `test` attribute

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

### Checking Results with the assert! Macro

- The assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true
- `use super::*;`, we use a glob here so anything we define in the outer module is available to this tests module.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

### Testing Equality with the assert_eq! and assert_ne! Macros

- A common way to verify functionality is to test for equality between the result of the code under test and the value you expect the code to return
- Standard library provides a pair of macros `assert_eq!` and `assert_ne!`

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

### Adding Custom Failure Messages

- Add a custom message to be printed with the failure message as optional arguments to the assert!, assert_eq!, and assert_ne!
- You can pass a format string that contains {} placeholders and values to go in those placeholders

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
```

### Checking for Panics with should_panic

- `should_panic`, the test passes if the code inside the function panics; the `test fails` if the code inside the function `doesn’t panic`

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

- To make should_panic tests more precise, we can add an optional `expected` parameter to the should_panic attribute

```rust
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

### Using Result<T, E> in Tests

- Our tests so far all panic when they fail. We can also write tests that use `Result<T, E>`
- We return Ok(()) when the test passes and an Err with a String inside when the test fails

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

## Controlling How Tests Are Run

- The default behavior of the binary produced by cargo test is to `run all the tests in parallel` and capture output generated during test runs
- Preventing the output from being displayed and making it easier to read the output related to the test results. You can, however, specify command line options to change this default behavior
- Running `cargo test --help` displays the options you can use with cargo test, and running `cargo test -- --help` displays the options you can use after the separator.

### Running Tests in Parallel or Consecutively

- If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the `--test-threads`

```sh
$ cargo test -- --test-threads=1
```

### Showing Function Output

- If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests with `--show-output`

```sh
$ cargo test -- --show-output
```

### Running a Subset of Tests by Name

- You can choose which tests to run passing cargo test the name or names of the test(s) you want to run as an argument

```sh
$ cargo test one_hundred
```

- We can specify part of a test name, and any test whose name matches that value will be run

```sh
$ cargo test add
```

### Ignoring Some Tests Unless Specifically Requested

- You can instead annotate the time-consuming tests using the ignore attribute to exclude them

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

```sh
cargo test -- --ignored
```

## Test Organinzation

- Testing is a complex discipline, and different people use different terminology and organization
- The Rust community thinks about tests in terms of two main categories: `unit tests` and `integration tests`
  - `Unit tests` are `small and more focused`, testing one module in isolation at a time, and can test private interfaces
  - `Integration tests` are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

### Unit Tests

- The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected.
- You’ll put unit tests in the src directory in each file with the code that they’re testing
- The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run`cargo test`

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

#### Testing Private Functions

- Rust’s privacy rules do allow you to test private functions

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

### Integration Tests

- In Rust, integration tests are entirely external to your library
- The test can only call functions that are part of your library’s public API
- Their purpose is to test whether many parts of your library work together correctly

#### The tests Directory

- We create a tests directory at the top level of our project directory, next to src
- Cargo knows to look for integration test files in this directory
- We can then make as many test files as we want, and Cargo will compile each of the files as an individual crate

```sh
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

- Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope. For that reason we add `use adder` at the top of the code, which we didn’t need in the unit tests.

```rust
// tests/integration_test.rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

- We can still run a particular integration test function by specifying the test function’s name as an argument to cargo test. To run all the tests in a particular integration test file, use the --test argument of cargo test

```sh
$ cargo test --test integration_test
```

#### Submodules in Integration Tests

- You can group the test functions by the functionality they’re testing
- Each file in the tests directory is compiled as its own separate crate
- Files in the tests directory don’t share the same behavior as files in src
- We want to call from multiple test functions in multiple test files, to avoid having common appear in the test output, instead of creating tests/common.rs, we’ll create tests/common/mod.rs

```sh
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

- After we’ve created tests/common/mod.rs, we can use it from any of the integration test files as a module

```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

#### Integration Tests for Binary Crates

- If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement
- Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

# An I/O Project: Building a Command Line Program

- This chapter is a recap of the many skills you’ve learned so far and an exploration of a few more standard library features
  - Organizing code (using what you learned about modules in Chapter 7)
  - Using vectors and strings (collections, Chapter 8)
  - Handling errors (Chapter 9)
  - Using traits and lifetimes where appropriate (Chapter 10)
  - Writing tests (Chapter 11)

## Accepting Command Line Arguments

- Create a new project with, as always, `cargo new`

```sh
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

### Reading the Argument Values

- To enable minigrep to read the values of command line arguments we pass to it, we’ll need the std::env::args function provided in Rust’s standard library
- Bring the `std::env` module into scope with a use statement 
  - In cases where the desired function is nested in more than one module, we’ve chosen to bring the parent module into scope rather than the function
  - `std::env::args` will panic if any argument contains `invalid Unicode`. If your program needs to accept arguments containing invalid Unicode, use `std::env::args_os` instead
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```

- Print the vector using the debug macro

```sh
$ cargo run -- needle haystack
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/minigrep needle haystack`
[src/main.rs:5] args = [
    "target/debug/minigrep",
    "needle",
    "haystack",
]
```

### Saving the Argument Values in Variables

- As we saw when we printed the vector, the program’s name takes up the first value in the vector at args[0], so we’re starting arguments at index 1
- Run `cargo run -- test sample.txt`

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
```

## Reading a File

- First, we bring in a relevant part of the standard library with a use statement: we need `std::fs` to handle files.
- The new statement fs::read_to_string takes the file_path, opens that file, and returns a `std::io::Result<String>` of the file’s contents.
- Run `cargo run -- the poem.txt`

```rust
use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
```

## Refactoring to Improve Modularity and Error Handling

### Separation of Concerns for Binary Projects

- the Rust community has developed guidelines for splitting the separate concerns of a binary program when main starts getting large.
  - Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
  - As long as your command line parsing logic is small, it can remain in main.rs.
  - When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

#### Extracting the Argument Parser

- Extract the functionality for parsing arguments into a function that main will call to prepare for moving the command line parsing logic to src/lib.rs
- This rework may seem like overkill for our small program, but we’re refactoring in small, incremental steps

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
```

#### Grouping Configuration Values

- At the moment, we’re returning a tuple, but then we immediately break that tuple into individual parts again. This is a sign that perhaps we don’t have the right abstraction yet
- We’ll instead put the two values into one struct and give each of the struct fields a meaningful name
- There are a number of ways we could manage the String data; the easiest, though somewhat inefficient, route is to call the clone method on the values. This will make a full copy of the data for the Config instance to own, which takes more time and memory than storing a reference to the string data. However, cloning the data also makes our code very straightforward because we don’t have to manage the lifetimes of the references
- The Trade-Offs of Using clone: it’s okay to copy a few strings to continue making progress because you’ll make these copies only once and your file path and query string are very small

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // --snip--
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
```

#### Creating a Constructor for Config

- We can create instances of types in the standard library, such as String, by calling String::new. Similarly, by changing parse_config into a new function associated with Config, we’ll be able to create instances of Config by calling Config::new

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

// --snip--

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
```

### Fixing the Error Handling

#### Improving the Error Message

```rust
// --snip--
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // --snip--
```

#### Returning a Result Instead of Calling panic!

- We’re also going to change the function name from new to build because many programmers expect new functions to never fail.
- When Config::build is communicating to main, we can use the Result type to signal there was a problem. Then we can change main to convert an Err variant into a more practical error for our users without the surrounding text about thread 'main' and RUST_BACKTRACE that a call to panic! causes
- Our build function returns a Result with a Config instance in the success case and a `&'static str` in the error case. Our error values `will always be string literals that have the 'static lifetime`

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

#### Calling Config::build and Handling Errors

- We’ve used a method we haven’t covered in detail yet: unwrap_or_else, which is defined on Result<T, E> by the standard library. Using unwrap_or_else allows us to define some custom, non-panic! error handling
- We’ve added a new use line to bring process from the standard library into scope. The code in the closure that will be run in the error case is only two lines: we print the err value and then call process::exit

```rust
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
```

### Extracting Logic from main

- Extract a function named run that will hold all the logic currently in the main function that isn’t involved with setting up configuration or handling errors

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

// --snip--
```

#### Returning Errors from the run Function

- Instead of allowing the program to panic by calling expect, the run function will return a Result<T, E> when something goes wrong
- First, we changed the return type of the run function to `Result<(), Box<dyn Error>>`
- We used the trait object `Box<dyn Error>` (and we’ve brought std::error::Error into scope with a use statement at the top)
  - `Box<dyn Error>` means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be. This gives us flexibility to return error values that may be of different types in different error cases. `The dyn keyword is short for “dynamic.”`
- Second, we’ve removed the call to expect in favor of the ? operator. Rather than panic! on an error, `? will return the error value from the current function` for the caller to handle.

#### Handling Errors Returned from run in main

- Use `if let` rather than `unwrap_or_else` to check whether run returns an `Err value` and call `process::exit(1)`

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
```

### Splitting Code into a Library Crate

Let’s move all the code that isn’t the main function from src/main.rs to src/lib.rs:

- The run function definition
- The relevant use statements
- The definition of Config
- The Config::build function definition

## Developing the Library’s Functionality with Test-Driven Development

We’ll add the searching logic to the minigrep program using the test-driven development (TDD) process with the following steps:

  1. Write a test that fails and run it to make sure it fails for the reason you expect.
  2. Write or modify just enough code to make the new test pass.
  3. Refactor the code you just added or changed and make sure the tests continue to pass.
  4. Repeat from step 1!

### Writing a Failing Test

- Let’s remove the println! statements from src/lib.rs and src/main.rs that we used to check the program’s behavior.

```rust
// src/lib.rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

- Run `cargo test`

### Writing Code to Pass the Test

Our program needs to follow these steps:

- Iterate through each line of the contents
- Check whether the line contains our query string
- If it does, add it to the list of values we’re returning
- If it doesn’t, do nothing
- Return the list of results that match

```rust
// src/lib.rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // Iterating Through Lines with the lines Method
    for line in contents.lines() {
        // Check whether the line contains our query string
        if line.contains(query) {
            // If it does, add it to the list of values we’re returning
            results.push(line);
        } // If it doesn’t, do nothing
    }

    results // Return the list of results that match
}
```

- Run `cargo test`

#### Using the search Function in the run Function

```rust
// src/lib.rs
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
```

## Working with Environment Variables

Improve minigrep by adding an extra feature: an option for case-insensitive searching that the user can turn on via an environment variable

### Writing a Failing Test for the Case-Insensitive search Function

```rust
// src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

### Implementing the search_case_insensitive Function

- The only difference is that we’ll lowercase the query and each line so whatever the case of the input arguments

```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

- Run `cargo test`
- We’ll add a configuration option to the Config struct to switch between case-sensitive and case-insensitive search

```rust
// src/lib.rs
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
```

- Finally, we need to check for the environment variable. The functions for working with environment variables are in the env module in the standard library

```rust
// src/lib.rs
use std::env;
// --snip--

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

- Run `cargo run -- to poem.txt` or `IGNORE_CASE=1 cargo run -- to poem.txt`

## Writing Error Messages to Standard Error Instead of Standard Output

We’re writing all of our output to the terminal using the println! macro. In most terminals, there are two kinds of output: standard output (stdout) for general information and standard error (stderr) for error messages.

### Checking Where Errors Are Written

Command line programs are expected to send error messages to the standard error stream so we can still see error messages on the screen even if we redirect the standard output stream to a file. Our program is not currently well-behaved: we’re about to see that it saves the error message output to a file instead!

### Printing Errors to Standard Error

- The standard library provides the eprintln! macro that prints to the standard error stream, so let’s change the two places we were calling println! to print errors to use eprintln! instead.

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```

- Run `cargo run > output.txt` or `cargo run -- to poem.txt > output.txt`

# Functional Language Features: Iterators and Closures

Rust’s design has taken inspiration from many existing languages and techniques, and one significant influence is functional programming. Programming in a functional style often includes using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution, and so forth

More specifically, we’ll cover:

- Closures, a function-like construct you can store in a variable
- Iterators, a way of processing a series of elements
- How to use closures and iterators to improve the I/O project in Chapter 12
- The performance of closures and iterators (Spoiler alert: they’re faster than you might think!)

## Closures: Anonymous Functions that Can Capture Their Environment

- Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions
- You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context
- Unlike functions, closures can capture values from the scope in which they’re defined

### Capturing the Environment with Closures

- Body of the `giveaway` method that uses a closure
- In the `giveaway` method, we get the user preference as a parameter of type `Option<ShirtColor>` and call the `unwrap_or_else` method on `user_preference`
-  If the `Option<T>` is the `Some` variant, `unwrap_or_else returns the value from within the Some`. If the `Option<T>` is the `None` variant, `unwrap_or_else calls the closure and returns the value returned by the closure`

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

- Run `cargo run`

```sh
The user with preference Some(Red) gets Red
The user with preference None gets Blue
```

### Closure Type Inference and Annotation

- Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do. Type annotations are required on functions because the types are part of an explicit interface exposed to your users
- Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables
- As with variables, we can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose than is strictly necessary


```rust
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

- With type annotations added, the syntax of closures looks more similar to the syntax of functions

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

- For closure definitions, the compiler will infer one concrete type for each of their parameters and for their return value
- Types are locked in to the closure, getting a type error if we try to use a different type with the same closure

```rust
let example_closure = |x| x;


let s = example_closure(String::from("hello"));
let n = example_closure(5);
        --------------- ^- help: try using a conversion method: `.to_string()`
        |               |
        |               expected struct `String`, found integer
        arguments to this function are incorrect
```

### Capturing References or Moving Ownership

- Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: `borrowing immutably`, `borrowing mutably`, and `taking ownership`
- Captures an immutable reference to the vector named list

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}
```

- We change the closure body so that it adds an element to the list vector. The closure now captures a mutable reference

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}
```

- If you want to force the closure to take ownership of the values it uses in the environment even though the body of the closure doesn’t strictly need ownership, you can use the `move` keyword before the parameter list
- We spawn a new thread, giving the thread a closure to run as an argument. The closure body prints out the list
- The closure only captured list using an immutable reference because that's the least amount of access to list needed to print it. In this example, even though the closure body still only needs an immutable reference, we need to specify that list should be moved into the closure by putting the move keyword at the beginning of the closure definition

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
```

### Moving Captured Values Out of Closures and the Fn Traits

- The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use
- Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:
  1. `FnOnce` applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
  2. `FnMut` applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
  3. `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
- Let’s look at the definition of the unwrap_or_else method on `Option<T>`
  - `unwrap_or_else` function has the additional generic type parameter F. The F type is the type of the parameter named f, which is the closure we provide when calling unwrap_or_else
  - The trait bound specified on the generic type F is FnOnce() -> T, which means F must be able to be called once, take no arguments, and return a T. Using FnOnce in the trait bound expresses the constraint that unwrap_or_else is only going to call f at most one time
  - `f` the Option is `Some`, f won’t be called. If the Option is None, `f` will be called `once`

> Note: Functions can implement all three of the Fn traits too. If what we want to do doesn’t require capturing a value from the environment, we can use the name of a function rather than a closure where we need something that implements one of the Fn traits. For example, on an Option<Vec<T>> value, we could call unwrap_or_else(Vec::new) to get a new, empty vector if the value is None.

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

- `sort_by_key` uses FnMut instead of FnOnce for the trait bound. The closure gets one argument in the form of a reference to the current item in the slice being considered, and returns a value of type K that can be ordered
- The reason sort_by_key is defined to take an FnMut closure is that it calls the closure multiple times: once for each item in the slice. The closure |r| r.width doesn’t capture, mutate, or move out anything from its environment, so it meets the trait bound requirements.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
```

- The closure captures value then moves value out of the closure by transferring ownership of value to the sort_operations vector. This closure can be called once (FnOnce trait); trying to call it a second time wouldn’t work because value would no longer be in the environment to be pushed into sort_operations again!

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");
        ----- captured outer variable
    list.sort_by_key(|r| {
                     --- captured by this `FnMut` closure
        sort_operations.push(value);
                              ^^^^ move occurs because `value` has type `String`, which does
                                   not implement the `Copy` trait
        r.width
    });
    println!("{:#?}", list);
}
```

- To fix this, we need to change the closure body so that it doesn’t move values out of the environment
- The Fn traits are important when defining or using functions or types that make use of closures

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
```

```rust
fn main() {
  let x = 4;

  let equal_to_x = |z| z == x; // capure x

  let y = 4

  assert!(equal_to_x(y));

  let x = 4;

  // can't capture, use FnOnce, fnMut, Fn
  // FnOnce, capture from enclosing scope, take ownership, can't take ownership more than once
  // FnMut, can change the environment
  // Fn, borrows values from environment immutably
  fn euqal_to_x(z: i32) -> bool { z == x }

  // move
  let x = vec![1, 2, 3];
  let equal_to_x = move |z| z == x; // x moved in to closure

  println!("can't use x here: {:?}", x); // panic! value used after move

  let y = vec![1, 2, 3];

  assert!(equal_to_x(y));
}
```

## Processing a Series of Items with Iterators

- The iterator pattern allows you to perform some task on a sequence of items in turn
- In Rust, iterators are lazy, `have no effect until you call methods` that consume the iterator to use it up
- his code by itself doesn’t do anything useful

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
```

- When the for loop is called using the iterator in v1_iter, each element in the iterator is used in one iteration of the loop, which prints out each value.

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

### The Iterator Trait and the next Method

- All iterators implement a trait named Iterator that is defined in the standard library
- Notice this definition uses some new syntax: type Item and Self::Item, which are defining an associated type with this trait
  - The Iterator trait requires that you also define an Item type, and this Item type is used in the return type of the next method

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

- We can call the next method on iterators directly;
- If we want to iterate over mutable references, we can call iter_mut instead of Iiter

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

### Methods that Consume the Iterator

- `The Iterator trait` has a `number of different methods` with default implementations provided by the standard library
- Methods that call next are called `consuming adaptors` eg: sum, collect
- We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

### Methods that Produce Other Iterators

- `Iterator adaptors` are methods defined on the Iterator trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator
- The warning reminds us why: iterator adaptors are lazy, and we need to consume the iterator here

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
^^^^^^^^^^^^^^^^^^^^^^^^^
= note: `#[warn(unused_must_use)]` on by default
= note: iterators are lazy and do nothing unless consumed
```

- To fix this warning and consume the iterator, we’ll use the collect method
- We collect the results of iterating over the iterator that’s returned from the call to map into a vector
- You can chain multiple calls to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

### Using Closures that Capture Their Environment

- Many iterator adapters `take closures as arguments`, and commonly the closures we’ll specify as arguments to iterator adapters will be closures that capture their environment

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// The shoes_in_size function takes ownership of a vector of shoes and a shoe size as parameters. It returns a vector containing only shoes of the specified size
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // we call `into_iter` to create an iterator that takes ownership of the vector
    shoes.into_iter()
      // Then we call filter to adapt that iterator into a new iterator that only contains elements for which the closure returns true
      // The closure captures the shoe_size parameter from the environment and compares the value with each shoe’s size
      .filter(|s| s.size == shoe_size)
      // Finally, calling collect gathers the values returned by the adapted iterator into a vector that’s returned by the function
      .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```

## Improving Our I/O Project

### Removing a clone Using an Iterator

#### Using the Returned Iterator Directly

- We’ll first change the start of the main function to use iterator

```rust
// src/main.rs
fn main() {
    // The env::args function returns an iterator, now we’re passing ownership of the iterator returned from env::args to Config::build directly
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
}

//src/lib.rs
impl Config {
    pub fn build(
        // parameter args has a generic type with the trait bounds impl Iterator<Item = String> instead of &[String]
        // This usage of the impl Trait syntax, “Traits as Parameters”
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
// --snip--
    }
}
```

#### Using Iterator Trait Methods Instead of Indexing

-  Because args implements the Iterator trait, we know we can call the next method on it

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // Remember that the first value in the return value of env::args is the name of the program
        args.next();

        // Call next to get the value we want to put in the query field of Config. If next returns a Some, we use a match to extract the value. If it returns None, it means not enough arguments were given and we return early with an Err value. We do the same thing for the file_path value.
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

### Making Code Clearer with Iterator Adaptors

- We can write this code in a more concise way using iterator adaptor methods. Doing so also lets us avoid having a mutable intermediate results vector. The functional programming style prefers to minimize the amount of mutable state to make code clearer

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

### Choosing Between Loops or Iterators

- Most Rust programmers prefer to use the iterator style. It’s a bit tougher to get the hang of at first, but once you get a feel for the various iterator adaptors and what they do, iterators can be easier to understand
- But are the two implementations truly equivalent? The intuitive assumption might be that the more low-level loop will be faster. Let’s talk about performance

## Comparing Performance: Loops vs. Iterators

- The point is this: iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself. Iterators are one of `Rust’s zero-cost abstractions`, by which we mean using the abstraction imposes no additional runtime overhead
- The iterator version was slightly faster!
- Rust knows that how many iterations, so it `unrolls` the loop, compiles down to same assembly, `Unrolling` is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop.

# More about Cargo and Crates.io

 In this chapter, we’ll discuss some of its other, more advanced features to show you how to do the following:

- Customize your build through release profiles
- Publish libraries on crates.io
- Organize large projects with workspaces
- Install binaries from crates.io
- Extend Cargo using custom commands

## Customizing Builds with Release Profiles

- In Rust, release profiles are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code. Each profile is configured independently of the others
- Cargo has two main profiles: the dev profile Cargo uses when you run cargo build and the release profile Cargo uses when you run `cargo build --release`. The dev profile is defined with good defaults for development, and the release profile has good defaults for release builds

```sh
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```

- Cargo has default settings for each of the profiles that apply when you haven't explicitly added any [profile.*] sections in the project’s Cargo.toml file
- By adding [profile.*] sections for any profile you want to customize, you override any subset of the default settings
- The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3

```toml
// Cargo.toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

- You can override a default setting by adding a different value for it in Cargo.toml

```toml
[profile.dev]
opt-level = 1
```

## Publishing a Crate to Crates.io

- We’ve used packages from crates.io as dependencies of our project, but you can also share your code with other people by publishing your own packages

### Making Useful Documentation Comments

- Rust also has a particular kind of comment for documentation, known conveniently as a documentation comment, that will generate HTML documentation
- Documentation comments use three slashes, `///`, instead of two and `support Markdown notation` for formatting the text

````rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
````

- Run `cargo doc --open` will build the HTML

![html](https://doc.rust-lang.org/stable/book/img/trpl14-01.png)

#### Commonly Used Sections

- We used the `# Examples Markdown heading` to create a section in the HTML with the title `“Examples.”`
- Here are some other sections that crate authors commonly use in their documentation:
  - Panics: The scenarios in which the function being documented could panic. Callers of the function who don’t want their programs to panic should make sure they don’t call the function in these situations.
  - Errors: If the function returns a Result, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so they can write code to handle the different kinds of errors in different ways.
  - Safety: If the function is unsafe to call (we discuss unsafety in Chapter 19), there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.

#### Documentation Comments as Tests

- Adding example code blocks in your documentation comments can help demonstrate how to use your library, and doing so has an additional bonus: running `cargo tes`t will `run the code examples in your documentation` as tests!

#### Commenting Contained Items

- The style of doc comment `//!` adds documentation to the item that contains the comments rather than to the items following the comments
- We typically use these doc comments inside the crate root file (src/lib.rs by convention) or inside a module to document the crate or the module as a whole
- Documentation comments within items are useful for describing crates and modules especially

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

![html](https://doc.rust-lang.org/stable/book/img/trpl14-02.png)

### Exporting a Convenient Public API with pub use

- The structure of your public API is a major consideration when publishing a crate.

```rust
// src/lib.rs

//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}

// src/main.rs
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

- Run `cargo doc`

![](https://doc.rust-lang.org/stable/book/img/trpl14-03.png)

- The good news is that if the structure isn’t convenient for others to use from another library, you `don’t have to rearrange your internal organization`: instead, `you can re-export items to make a public structure` that’s different from your private structure by using `pub use`

```rust
// src/lib.rs

//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}

// src/main.rs
use art::mix;
use art::PrimaryColor;

fn main() {
    // --snip--
}
```

- Run `cargo doc`

![](https://doc.rust-lang.org/stable/book/img/trpl14-03.png)


### Setting Up a Crates.io Account

- Before you can publish any crates, you need to create an account on crates.io and get an API token
- Visit your account settings at https://crates.io/me/ and retrieve your API key. Then run the cargo login command with your API key, like this:

```sh
$ cargo login abcdefghijklmnopqrstuvwxyz012345
```

### Adding Metadata to a New Crate

- Before publishing, you’ll need to add some metadata in the [package] section of the crate’s Cargo.toml file.
  - Your crate will need a unique name. Crate names on crates.io are allocated on a first-come, first-served basis. Once a crate name is taken, no one else can publish a crate with that name
  - This errors because you’re missing some crucial information: a description and license are required so people will know what your crate does and under what terms they can use it
- With a unique name, the version, your description, and a license added, the Cargo.toml file for a project that is ready to publish might look like this:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

### Publishing to Crates.io

- Now that you’ve created an account, saved your API token, chosen a name for your crate, and specified the required metadata, you’re ready to publish! Publishing a crate uploads a specific version to crates.io for others to use.
- Be careful, because a publish is permanent. The version can never be overwritten, and the code cannot be deleted

- `cargo login` for setting up a crates.io account
- adding metadata

```
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

- Run the cargo publish command to publish the crate to crates.io

### Publishing a New Version of an Existing Crate

- When you’ve made changes to your crate and are ready to release a new version, you change the version value specified in your Cargo.toml file and republish
- Use the Semantic Versioning rules to decide what an appropriate next version number is based on the kinds of changes you’ve made

### Deprecating Versions from Crates.io with cargo yank

- This is useful when a crate version is broken for one reason or another. In such situations, Cargo supports yanking a crate version
- Yanking a version prevents new projects from depending on that version while allowing all existing projects that depend on it to continue. Essentially, a yank means that all projects with a Cargo.lock will not break, and `any future Cargo.lock files generated will not use the yanked version`
- `yank` does not delete any code. It cannot, for example, delete accidentally uploaded secrets. If that happens, you must reset those secrets immediately
```sh
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1

# By adding --undo to the command, you can also undo a yank and allow projects to start depending on a version again:

$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1
```

## Cargo Workspaces

- As your project develops, you might find that the library crate continues to get bigger and you want to split your package further into multiple library crates

### Creating a Workspace

- Workspace is a set of packages that share the same Cargo.lock and output directory

```sh
$ mkdir add
$ cd add
```

- Next, in the add directory, we create the Cargo.toml file that will configure the entire workspace
- It will start with a [workspace] section that will allow us to add members to the workspace by specifying the path to the package with our binary crate

```sh
// Cargo.toml
[workspace]

members = [
    "adder",
]
```

- Next, we’ll create the adder binary crate by running cargo new within the add directory:

```sh
$ cargo new adder
     Created binary (application) `adder` package
```

- At this point, we can build the workspace by running cargo build. The files in your add directory should look like this:

```
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

### Creating the Second Package in the Workspace

- Next, let’s create another member package in the workspace and call it add_one. Change the top-level Cargo.toml to specify the add_one path in the members list:

```toml
[workspace]

members = [
    "adder",
    "add_one",
]
```

- Then generate a new library crate named add_one:

```sh
$ cargo new add_one --lib
     Created library `add_one` package
```

- Your add directory should now have these directories and files:

```
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

- In the add_one/src/lib.rs file, let’s add an add_one function:

```rust
// add_one/src/lib.rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

- Now we can have the adder package with our binary depend on the add_one package that has our library. First, we’ll need to add a path dependency on add_one to adder/Cargo.toml.
- Cargo doesn’t assume that crates in a workspace will depend on each other, so we need to be explicit about the dependency relationships

```toml
// adder/Cargo.toml
[dependencies]
add_one = { path = "../add_one" }
```

- Next, let’s use the add_one function (from the add_one crate) in the adder crate

```rust
// adder/src/main.rs

use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
```

- Let’s build the workspace by running cargo build in the top-level add directory

```sh
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
```

- To run the binary crate from the add directory, we can specify which package in the workspace we want to run using the -p argument and the package name with cargo run:

```sh
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

#### Depending on an External Package in a Workspace

- Notice that the workspace has `only one Cargo.lock file at the top level`, rather than having a Cargo.lock in each crate’s directory
- This ensures that `all crates are using the same version of all dependencies`. If we add the rand package to the adder/Cargo.toml and add_one/Cargo.toml files, `Cargo will resolve both of those to one version of rand and record` that in the one Cargo.lock
- We can now add use rand; to the add_one/src/lib.rs file, and building the whole workspace by running cargo build in the add directory will bring in and compile the rand crate
  - We will get one warning because we aren’t referring to the rand we brought into scope

```toml
// add_one/Cargo.toml

[dependencies]
rand = "0.8.5"
```

- The top-level Cargo.lock now contains information about the dependency of add_one on rand. However, even though rand is used somewhere in the workspace, we can’t use it in other crates in the workspace unless we add rand to their Cargo.toml files as well
- For example, if we add use rand; to the adder/src/main.rs file for the adder package, we’ll get an error

```sh
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

- To fix this, `edit the Cargo.toml file for the adder package` and indicate that rand is a dependency for it as well. `Building the adder package will add rand to the list of dependencies for adder in Cargo.lock`, but no additional copies of rand will be downloaded

#### Adding a Test to a Workspace

- For another enhancement, let’s add a test of the add_one::add_one function within the add_one crate:

```rust
// add_one/src/lib.rs

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

- Run `cargo test`or `cargo test -o add-one` for testing specific test

## Installing Binaries from Crates.io with cargo install

- `cargo install` command allows you to install and use binary crates locally
- This isn’t intended to replace system packages
- All binaries installed with cargo install are stored in the installation `root’s bin folder`. If you installed Rust using rustup.rs and don’t have any custom configurations, this directory will be `$HOME/.cargo/bin`

## Extending Cargo with Custom Commands

- Cargo is designed so you can extend it with new subcommands without having to modify Cargo.
- If a binary in your $PATH is named cargo-something, you can run it as if it was a Cargo subcommand by running `cargo something`
- Custom commands like this are also listed when you run `cargo --list`

# Smart Pointers

- Pointer is a general concept for a variable that contains an address in memory
- Smart pointers, on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities
- Rust, with its concept of ownership and borrowing, has an additional difference between references and smart pointers: `while references only borrow data`
- Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the Deref and Drop traits
  - The Deref trait allows an instance of the smart pointer struct to `behave like a reference so you can write your code to work with either references or smart pointers`
  - The Drop trait allows you to customize the code that’s` run when an instance of the smart pointer goes out of scope`
- We’ll cover the most common smart pointers in the standard library:
  - Box<T> for allocating values on the heap
  - Rc<T>, a reference counting type that enables multiple ownership
  - Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

## Using `Box<T> to Point to Data on he Heap

- The most straightforward smart pointer is a box, whose type is written `Box<T>`
- Boxes allow you to `store data on the heap` rather than the stack
- Boxes `don’t have performance overhead`, other than storing their data on the heap instead of on the stack
- You’ll use them most often in these situations:
  - When you have a type whose `size can’t be known at compile time` and you want to use a value of that type in a context that `requires an exact size`
  - When you have a `large amount of data` and `you want to transfer ownership` but ensure the data ` won’t be copied when you do` so
  - When you `want to own a value and you care only that it’s a type` that implements a particular trait rather than being of a specific type

### Using a Box<T> to Store Data on the Heap

- We define the variable b to have the value of a Box that points to the value 5, which is allocated on the heap
-  Just like any owned value, when a box goes out of scope, as b does at the end of main, it will be deallocated

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

### Enabling Recursive Types with Boxes

- Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up
- However, the nesting of values of recursive types could theoretically continue infinitely, so Rust can’t know how much space the value needs
- Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

#### More Information About the Cons List

- A cons list is a data structure that comes from the Lisp programming language and its dialects and is made up of nested pairs, and is the Lisp version of a linked list. Its name comes from the cons function (short for “construct function”) in Lisp
- Each item in a cons list contains two elements: the value of the current item and the next item
- The last item in the list contains only a value called Nil without a next item
- `The cons list isn’t a commonly used data structure in Rust`. Most of the time when you have a list of items in Rust, `Vec<T> is a better choice to use`
- If we try to compile the code below, we will see the error shows this type `“has infinite size.”` The reason is that we’ve defined List with a variant that is recursive: it holds another value of itself directly. As a result, `Rust can’t figure out how much space it needs to store a List value`

```rust
enum List {
^^^^^^^^^ recursive type has infinite size
  Cons(i32, Box<List>),
|               ---- recursive without indirection
  Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

- An infinite List consisting of infinite Cons variants

![](https://doc.rust-lang.org/stable/book/img/trpl15-01.svg)

#### Computing the Size of a Non-Recursive Type

- To determine how much space to allocate for a Message value, Rust goes through each of the variants to see which variant needs the most space. Rust sees that Message::Quit doesn’t need any space, Message::Move needs enough space to store two i32 values, `and so forth`. Because only one variant will be used, the most space a Message value will need is the space it would take to store `the largest of its variants.`

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

#### Using Box<T> to Get a Recursive Type with a Known Size

- Because Rust can’t figure out how much space to allocate for recursively defined types, the compiler gives an error with this helpful suggestion:

```
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

- In this suggestion, `“indirection” means` that instead of storing a value directly, `we should change the data structure to store the value indirectly by storing a pointer to the value instead`
- `Because a Box<T> is a pointer`, Rust always `knows how much space a Box<T> needs`

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

- A List that is not infinitely sized because Cons holds a Box
- Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like those we’ll see with the other smart pointer types

![](https://doc.rust-lang.org/stable/book/img/trpl15-02.svg)

## Treating Smart Pointers Like Regular References with the Deref Trait

- Implementing the `Deref trait` allows you to customize the behavior of the `dereference operator *`
- By implementing Deref in such a way that a smart pointer `can be treated like a regular reference`

### Following the Pointer to the Value

- We create a reference to an i32 value and then use the dereference operator to follow the reference to the value:
- However, if we want to make an assertion about the value in y, we have to use *y to follow the reference to the value it’s pointing to (hence dereference) 

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    ^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
}
```

### Using Box<T> Like a Reference

- Use a `Box<T>` instead of a reference; the dereference operator used on the Box<T>
- Functions in the same way as the dereference operator used on the reference 
- We set y to be an instance of a Box<T> pointing to a `copied value of x rather` than a reference pointing to the value of x

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

### Defining Our Own Smart Pointer

- We define a struct named MyBox and declare a generic parameter T, because we want our type to hold values of any type
- The MyBox type is a tuple struct with one element of type T
- The MyBox::new function takes one parameter of type T and returns a MyBox instance that holds the value passed in.

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

n main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

- Run `cargo run` and the error will show because Rust doesn’t know how to dereference MyBox

```rust
$ cargo run
   Compiling deref-example v0.1.0 (file:///projects/deref-example)
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^

For more information about this error, try `rustc --explain E0614`.
error: could not compile `deref-example` due to previous error
```

### Treating a Type Like a Reference by Implementing the Deref Trait

- The Deref trait, provided by the standard library, requires us to implement one method named deref that borrows self and returns a reference to the inner data

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    // The syntax below defines an associated type for the Deref trait to use
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // We fill in the body of the deref method with &self.0 so deref returns a reference to the value we want to access with the * operator
        &self.0
    }
}
```

- When we entered *y in Listing 15-9, behind the scenes Rust actually ran this code:
- Rust substitutes the * operator with a call to the deref method and then a plain dereference so we don’t have to think about whether or not we need to call the deref method

```rust
*(y.deref())
```

### Implicit Deref Coercions with Functions and Methods

- Deref coercion `converts a reference` to a type that implements the Deref trait `into a reference to another type`
- Deref coercion `can convert &String` to `&str` because `String implements the Deref trait` such that `it returns &str`
- Deref coercion is a convenience Rust performs on `arguments to functions`
- Works `only on types that implement the Deref trait`
- It happens `automatically` when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition
- We can call the hello function with a string slice as an argument, such as hello("Rust");

```rust
fn hello(name: &str) {
  println!("Hello, {}!", name);
}
```

- Rust can turn &MyBox<String> into &String by calling deref
- The standard library provides an implementation of Deref on String that returns a string slice, and this is in the API documentation for Deref
- Rust calls deref again to turn the &String into &str, which matches the hello function’s definition

```rust
fn main() {
  let m = MyBox::new(String::from("rust"));
  hello(&m);
```

- If Rust didn’t implement deref coercion, we would have to write the code below instead of the code above to call hello with a value of type &MyBox<String>.
- The (*m) dereferences the MyBox<String> into a String. Then the & and [..] take a string slice of the String

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

### How Deref Coercion Interacts with Mutability

- You can use the DerefMut trait to override the * operator on mutable references
- Rust does deref coercion when it finds types and trati implementations in three cases:
  - From `&T` to `&U` when `T: Deref<Target=U>`
  - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
  - From `&mut T` to `&U` when `T: Deref<Target=U>`

## Running Code on Cleanup with the `Drop` Trait

- The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope
- You can provide an implementation for the Drop trait on any type, and that code can be used to release resources like files or network connections
- In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically
- For example, when a Box<T> is dropped it will deallocate the space on the heap that the box points to
- The Drop trait is included in the prelude, so we don’t need to bring it into scope

```rust
// src/main.rs
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

- Run `cargo run` to print logs
- Rust automatically called drop for us when our instances went out of scope, calling the code we specified

```sh
$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.60s
     Running `target/debug/drop-example`
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

### Dropping a Value Early with std::mem::drop

- Rust doesn’t let you call the Drop trait’s drop method manually

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
|     --^^^^--
|     | |
|     | explicit destructor calls not allowed
|     help: consider using `drop` function: `drop(c)`
    };
    println!("CustomSmartPointer created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}
```

- The std::mem::drop function is different from the drop method in the Drop trait. We call it by passing as an argument the value we want to force drop

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```


## Rc, the Reference Counted Smart Pointer

- to enable multiple ownership, Rust has a type called `RC<T>`

```rust
fn main() {
  let a = Rc::new(Cons(5, Rc:new(Cons(10, Rc::new(Nil)))));
  println!("count", Rc::strong_coint(&a)); // 1

  let b = Cons(3, Rc::clone(&a))
  println!("count", Rc::strong_coint(&a)); // 2

  {
    let c = Cons(3, Rc::clone(&a))
    println!("count", Rc::strong_coint(&a)); // 3
  }

  println!("count", Rc::strong_coint(&a)); // 4
}
```

## Rc<T>, the Reference Counted Smart Pointer

- You have to enable multiple ownership explicitly by using the Rust type `Rc<T>`, which is an abbreviation for reference counting
- The `Rc<T> type keeps track of the number of references` to a value to determine whether or not the value is still in use
- Note that Rc<T> is only for use in single-threaded scenarios

### Using Rc<T> to Share Data

- Trying to implement this scenario using our definition of List with Box<T> won’t work
- The Cons variants own the data they hold, so when we create the b list, a is moved into b and b owns a. Then, when we try to use a again when creating c, we’re not allowed to because a has been moved

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        - move occurs because `a` has type `List`, which does not implement the `Copy` trait
    let b = Cons(3, Box::new(a));
                             - value moved here
    let c = Cons(4, Box::new(a));
                             ^ value used here after move
}
```

- We’ll change our definition of List to use Rc<T> in place of Box<T>
- Each Cons variant will now hold a value and an Rc<T> pointing to a List. When we create b, instead of taking ownership of a, we’ll clone the Rc<List> that a is holding, thereby `increasing the number of references` from one to two and letting a and b share ownership of the data in that Rc<List>

```rust
num List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use Rc::clone in this case
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

### Cloning an Rc<T> Increases the Reference Count

- `Rc::clone` to `increase the reference count`: the implementation of the `Drop trait decreases the reference count` automatically when an Rc<T> value goes out of scope
- Via `immutable references, Rc<T>` allows you to share data between multiple parts of your program for reading only

```rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // count after creating a = 1
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    // count after creating b = 2
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        // count after creating c = 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // count after c goes out of scope = 2
}
```

## RefCell<T> and the Interior Mutability Pattern

- Interior mutability is a design pattern in Rust that `allows you to mutate data even when there are immutable references` to that data
- To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing
- We can use types that use the interior mutability pattern only `when we can ensure that the borrowing rules will be followed at runtime`

## RefCell and the Interior Mutability Pattern

### Enforcing Borrowing Rules at Runtime with RefCell<T>

- Unlike Rc<T>, the `RefCell<T>` type `represents single ownership` over the data it holds
- What makes RefCell<T> different from a type like Box<T>?
  - At any given time, `you can have either (but not both) one mutable reference` or `any number of immutable references`
  - References must always be valid
- With references and Box<T>, the borrowing rules’ `invariants are enforced at "compile time"`. With RefCell<T>, these `invariants are enforced at "runtime"`, `With references`, if you break these rules, you’ll `get a compiler error`. `With RefCell<T>`, if you break these rules, `your program will panic and exit`
  - `The advantages of checking the borrowing rules at compile time` are that` errors will be caught soone`r in the development process, and there is `no impact on runtime performance` because all the analysis is completed beforehand
  - `The advantage of checking the borrowing rules at runtime` instead is that certain memory-safe scenarios are then allowed, where they would’ve been disallowed by the compile-time checks
- Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.
- Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
  - `Rc<T>` enables `multiple owners of the same data`; `Box<T> and RefCell<T>` have `single owners`
  - `Box<T>` allows `immutable or mutable borrows checked at compile time`; `Rc<T>` allows `only immutable borrows` checked `at compile time`; `RefCell<T>` allows i`mmutable or mutable borrows` checked `at runtime`
  - Because `RefCell<T> allows mutable borrows` checked `at runtime`, you `can mutate the value inside the RefCell<T>` even when the RefCell<T> is immutable

### Interior Mutability: A Mutable Borrow to an Immutable Value

- Using RefCell<T> is one way to `get the ability to have interior mutability`, but RefCell<T> doesn’t get around the borrowing rules completely: the borrow checker `in the compiler` allows this `interior mutability`, and `the borrowing rules` are checked `at runtime instead`
- If you violate the rules, you’ll get a panic! instead of a compiler error.

#### A Use Case for Interior Mutability: Mock Objects

- Sometimes during testing a programmer will use a type in place of another type, in order to observe particular behavior and assert it’s implemented correctly. This placeholder type is called a `test double`
- Rust `doesn’t have objects in the same sense as other languages have objects`, and Rust doesn’t have mock object functionality built into the standard library as some other languages do. However, you can definitely create a struct that will serve the same purposes as a mock object.

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
    ----- help: consider changing that to be a mutable reference: `&mut self`
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

- We need a mock object that, instead of sending an email or text message when we call send, will only keep track of the messages it’s told to send
- We can create a new instance of the mock object, create a LimitTracker that uses the mock object, call the set_value method on LimitTracker, and then check that the mock object has the messages we expect

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // RefCell<Vec<String>> instead of Vec<String>
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // The first parameter is still an immutable borrow of self, which matches the trait definition. We call borrow_mut on the RefCell<Vec<String>>
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
        // We call `borrow` on the RefCell<Vec<String>> to get an immutable reference to the vector
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

#### Keeping Track of Borrows at Runtime with RefCell<T>

- With `RefCell<T>`, we use the `borrow` and `borrow_mut` methods
- `borrow method` returns the smart pointer type `Ref<T>`
- `borrow_mut method` returns the smart pointer type `RefMut<T>`
- Both types implement Deref, so we can treat them like regular references.
- The RefCell<T> `keeps track` of how many Ref<T> and RefMut<T> smart pointers are currently active
- Every time we call borrow, the RefCell<T> `increases its count of how many immutable borrows are active`
- When a Ref<T> value goes out of scope, the count of immutable borrows goes down by one
- RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time
- If we try to violate these rules, rather than getting a compiler error as we would with references, the implementation of RefCell<T> will `panic at runtime`
- This makes `two mutable references in the same scope, which isn’t allowed`

```rust
 impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();
        --- thread 'main' panicked at 'already borrowed: BorrowMutError'

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}
```

### Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

- A common way to use RefCell<T> is in `combination with Rc<T>`
- Recall that `Rc<T> lets you have multiple owners of some data`, but it `only gives immutable access` to that data
- If you have an `Rc<T> that holds a RefCell<T>`, you can get a `value that can have multiple owners` and that you can mutate

```rust
#[derive(Debug)]
enum List {
    // Rc<T> is used to allow multiple lists to share ownership of another list
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // We create a value that is an instance of Rc<RefCell<i32>> and store it in a variable named value
    // We can use the methods on RefCell<T> that provide access to its interior mutability so we can modify our data when we need to
    let value = Rc::new(RefCell::new(5));

    // We create a List in a with a Cons variant that holds value
    // We need to clone value so both a and value have ownership of the inner 5 value
    // We wrap the list a in an Rc<T> so when we create lists b and c, they can both refer to a
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // We create lists b and c, they can both refer to a
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Add 10 to the value in value. We do this by calling borrow_mut on value
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    // a after = Cons(RefCell { value: 15 }, Nil)
    println!("b after = {:?}", b);
    // b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    println!("c after = {:?}", c);
    // c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
}
```

## Reference Cycles Can Leak Memory

- Preventing memory leaks entirely is not one of Rust’s guarantees, meaning memory leaks are memory safe in Rust
- We can see that Rust `allows memory leaks` by using Rc<T> and RefCell<T>

### Creating a Reference Cycle

- Creates a list in a and a list in b that points to the list in a. Then it modifies the list in a to point to b, creating a reference cycle

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // We create an Rc<List> instance holding a List value in the variable a with an initial list of 5, Nil
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    //  We then create an Rc<List> instance holding another List value in the variable b that contains the value 10 and points to the list in a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // We modify a so it points to b instead of Nil, creating a cycle
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

- This instance’s memory can’t be dropped either, because the other Rc<List> instance still refers to it. The memory allocated to the list will remain uncollected forever
- If you have RefCell<T> values that contain Rc<T> values or similar nested combinations of types with interior mutability and reference counting, you must ensure that you don’t create cycles; ; you can’t rely on Rust to catch them
- Creating a reference cycle would be a logic bug in your program that you should use automated tests, code reviews, and other software development practices to minimize

![](https://doc.rust-lang.org/stable/book/img/trpl15-04.svg)

### Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>

- You can also create a weak reference to the value within an Rc<T> instance by calling Rc::downgrade and passing a reference to the Rc<T>
- Strong references are how you can share ownership of an Rc<T> instance. `Weak references don’t express an ownership relationship`, and `their count doesn’t affect when an Rc<T> instance is cleaned up`
- When you call Rc::downgrade, you get a smart pointer of type Weak<T>. Instead of increasing the strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the weak_count by 1
- The Rc<T> type `uses weak_count to keep track of how many Weak<T> references exist`, similar to strong_count. The difference is the `weak_count doesn’t need to be 0` for the Rc<T> instance to `be cleaned up`

#### Creating a Tree Data Structure: a Node with Child Nodes

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    // Vec<T> items to be values of type Rc<Node>, we also want to modify which nodes are children of another node, so we have a RefCell<T> in children around the Vec<Rc<Node>>
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        // We clone the Rc<Node> in leaf and store that in branch, meaning the Node in leaf now has two owners: leaf and branch
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // We can get from branch to leaf through branch.children, but there’s no way to get from leaf to branch. The reason is that leaf has no reference to branch and doesn’t know they’re related
}
```

#### Adding a Reference from a Child to Its Parent

- Thinking about the relationships another way, a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well. However, a child should not own its parent: if we drop a child node, the parent should still exist. This is a case for weak references
- So instead of Rc<T>, we’ll make the type of parent use Weak<T>, specifically a `RefCell<Weak<Node>>`

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    // We create a new, empty Weak<Node> reference instance.
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // Starts out without a parent, so we create a new, empty Weak<Node> reference instance
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = None

    let branch = Rc::new(Node {
        value: 5,
        // new Weak<Node> reference in the parent field, because branch doesn’t have a parent node
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Modify leaf to give it a Weak<Node> reference to its parent
    // We use the borrow_mut method on the RefCell<Weak<Node>> in the parent field of leaf, and then we use the Rc::downgrade function to create a Weak<Node> reference to branch from the Rc<Node> in branch.
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
    // children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
    // children: RefCell { value: [] } }] } })
}
```

#### Visualizing Changes to strong_count and weak_count

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // After leaf is created, its Rc<Node> has a strong count of 1 and a weak count of 0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // branch will have a strong count of 1 and a weak count of 1
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        // leaf, we’ll see it will have a strong count of 2, because branch now has a clone of the Rc<Node> of leaf stored in branch.children, but will still have a weak count of 0.
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    } // branch goes out of scope and the strong count of the Rc<Node> decreases to 0, so its Node is dropped

    // The weak count of 1 from leaf.parent has no bearing on whether or not Node is dropped, so we don’t get any memory leaks!
    // get None again
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // the Rc<Node> in leaf has a strong count of 1 and a weak count of 0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

# Fearless Concurrency

- Handling concurrent programming safely and efficiently is another of Rust’s major goals
- `Concurrent programming`, where different parts of a program execute independently, and `parallel programming`, where different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their multiple processors
- Over time, the team discovered that the ownership and type systems are a powerful set of tools to help manage memory safety and concurrency problems
- Here are the topics we’ll cover in this chapter:
  - How to `create threads to run multiple pieces of code at the same time`
  - `Message-passing concurrency`, where channels send messages between threads
  - `Shared-state concurrency`, where multiple threads have access to some piece of data
  - `The Sync and Send traits`, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library

## Using Threads to Run Code Simultaneously

> In most current operating systems, an executed program’s code is run in a process, and the operating system will manage multiple processes at once. Within a program, you can also have independent parts that run simultaneously. The features that run these independent parts are called `threads`

- Thread can lead to problems, such as:
  - `Race conditions`, where threads are accessing data or resources in an inconsistent order
  - `Deadlocks`, where two threads are waiting for each other, preventing both threads from continuing
  - `Bugs that happen only in certain situations` and are hard to reproduce and fix reliably

### Creating a New Thread with spawn

- To create a new thread, we call the thread::spawn function and pass it a closure
- Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running
- The threads will probably take turns, but that isn’t guaranteed

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        // The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run.
        thread::sleep(Duration::from_millis(1));
    }
}
```

### Waiting for All Threads to Finish Using join Handles

- The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // The two threads continue alternating, but the main thread waits because of the call to handle.join() and does not end until the spawned thread is finished
    handle.join().unwrap();
}
```

### Using move Closures with Threads

- We'll often use the move keyword with closures passed to thread::spawn because the closure will then take ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another
- By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // Rust was being conservative and only borrowing v for the thread, which meant the main thread could theoretically invalidate the spawned thread’s reference
    drop(v); // oh no!
         ^ value used here after move
    handle.join().unwrap();
}
```

### Using Message Passing to Transfer Data Between Threads

- One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data
- To accomplish message-sending concurrency, Rust's standard library provides an implementation of `channels`. A `channel` is a general programming concept by which data is sent from one thread to another.
- A channel has two halves: a `transmitter` and a `receiver`
- One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // We create a new channel using the mpsc::channel function; mpsc stands for multiple producer,
    let (tx, rx) = mpsc::channel();

    // We’re using thread::spawn to create a new thread and then using move to move tx into the closure so the spawned thread owns tx
    thread::spawn(move || {
        let val = String::from("hi");
        // The transmitter has a send method that takes the value we want to send
        // The send method returns a Result<T, E> type, so if the receiver has already been dropped and there’s nowhere to send a value, the send operation will return an error
        tx.send(val).unwrap();
    });

    // The receiver has two useful methods: recv and try_recv. We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is sent down the channel
    // The try_recv method doesn’t block, but will instead return a Result<T, E> immediately
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

### Channels and Ownership Transference

- The ownership rules play a vital role in message sending because they help you write safe, concurrent code
- Preventing errors in concurrent programming is the advantage of thinking about ownership throughout your Rust programs
- Our concurrency mistake has caused a compile time error. The send function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
        tx.send(val).unwrap();
        println!("val is {}", val);
                              ^^^ value borrowed here after move
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

### Sending Multiple Values and Seeing the Receiver Waiting

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // We iterate over them, sending each individually, and pause between each by calling the thread::sleep function with a Duration value of 1 second
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // We’re not calling the recv function explicitly anymore: instead, we’re treating rx as an iterator
    for received in rx {
        println!("Got: {}", received);
    }
}
```

### Creating Multiple Producers by Cloning the Transmitter

- Earlier we mentioned that mpsc was an acronym for multiple producer, single consumer
- Create multiple threads that all send values to the same receiver

```rust
// --snip--

let (tx, rx) = mpsc::channel();

// Before we create the first spawned thread, we call clone on the transmitter
let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}

// --snip--
```

## Shared-State Concurrency

- Another method would be for multiple threads to access the same shared data
- Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time

### Using Mutexes to Allow Access to Data from One Thread at a Time

- Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time
- To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock
- The mutex is described as guarding the data it holds via the locking system
- Mutexes have a reputation for being difficult to use because you have to remember two rules:

  - You must attempt to acquire the lock before using the data.
  - When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

- Thanks to Rust’s type system and ownership rules, you can’t get locking and unlocking wrong.

#### The API of Mutex<T>

- We create a Mutex<T> using the associated function new
- To access the data inside the mutex, we use the lock method to acquire the lock. This call will block the current thread so it can’t do any work until it’s our turn to have the lock.
- After we’ve acquired the lock, we can treat the return value, named num in this case, as a mutable reference to the data inside

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

#### Sharing a Mutex<T> Between Multiple Threads

- We’ll spin up 10 threads and have them each increment a counter value by 1, so the counter goes from 0 to 10
- The error message states that the counter value was moved in the previous iteration of the loop. Rust is telling us that we can’t move the ownership of lock counter into multiple threads

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
        ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
                                   ^^^^^^^ value moved into closure here, in previous iteration of loop
            // We use thread::spawn and give all the threads the same closure
            let mut num = counter.lock().unwrap();
                          ------- use occurs due to use in closure
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

#### Multiple Ownership with Multiple Threads

- We gave a value multiple owners by using the smart pointer Rc<T> to create a reference counted value

```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        ^ `Rc<Mutex<i32>>` cannot be sent between threads safely
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

- The compiler is also telling us the reason why: the trait `Send` is not implemented for `Rc<Mutex<i32>>`
- Unfortunately, Rc<T> is not safe to share across threads

#### Atomic Reference Counting with Arc<T>

- Fortunately, Arc<T> is a type like Rc<T> that is safe to use in concurrent situations
- The a stands for atomic, meaning it’s an atomically reference counted type
- You might then wonder why all primitive types aren’t atomic and why standard library types aren’t implemented to use Arc<T> by default. The reason is that thread safety comes with a performance penalty that you only want to pay when you really need to
- Let’s return to our example: Arc<T> and Rc<T> have the same API

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

### Similarities Between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>

- You might have noticed that counter is immutable but we could get a mutable reference to the value inside it; this means Mutex<T> provides `interior mutability`
- Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use Mutex<T>. Similarly, Mutex<T> comes with the risk of creating deadlocks

## Extensible Concurrency with the Sync and Send Traits

- Interestingly, the Rust language has very few concurrency features
- Your options for handling concurrency are not limited to the language or the standard library; you can write your own concurrency features or use those written by others

### Allowing Transference of Ownership Between Threads with Send

- The Send marker trait indicates that ownership of values of the type implementing Send can be transferred between threads
- Almost every Rust type is Send, but there are `some exceptions`, including `Rc<T>`
- Rust’s type system and trait bounds ensure that you can never accidentally send an Rc<T> value across threads unsafely
- Any type composed entirely of Send types is automatically marked as Send as well. Almost all primitive types are Send

### Allowing Access from Multiple Threads with Sync

- The `Sync marker trait` indicates that it is safe for the type implementing Sync to be referenced from multiple threads
- In other words, any type `T is Sync if &T (an immutable reference to T) is Send`, meaning the reference can be sent safely to another thread
- Similar to Send, primitive types are Sync, and types composed entirely of types that are Sync are also Sync
- The smart pointer Rc<T> is also `not Sync for` the same reasons that it’s not Send. The RefCell<T> type and the family of related Cell<T> types are not Sync
- The smart pointer `Mutex<T> is Sync` and can be used to share access with multiple threads

### Implementing Send and Sync Manually Is Unsafe

- Because types that are made up of Send and Sync traits are automatically also Send and Sync, we don’t have to implement those traits manually.
- Manually implementing these traits involves implementing unsafe Rust code

# Object Oriented Programming Features of Rust

## Characteristics of Object-Oriented Languages

- Rust is influenced by many programming paradigms, including OOP

### Objects Contain Data and Behavior

> Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations

- Using this definition, Rust is object-oriented: structs and enums have data, and impl blocks provide methods on structs and enums
- Even though structs and enums with methods aren’t called objects

### Encapsulation that Hides Implementation Details

- We can use the `pub keyword` to decide which modules, types, functions, and methods in our code should be public, and by default everything else is private

### Inheritance as a Type System and as Code Sharing

- Rust takes the different approach of using `trait` objects instead of inheritance

## Using Trait Objects that Allow for Values of Different Types

- We’ll create a library crate called gui that contains the structure of a GUI library. This crate might include some types for people to use, such as Button or TextField

### Defining a Trait for Common Behavior

- `Trait` objects differ from traditional objects in that we can’t add data to a trait object
- `Trait` objects purpose is to allow abstraction across common behavior

```rust
pub trait Draw {
    fn draw(&self);
}
```

- `components`, this vector is of type Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

- This works differently from defining a struct that uses a generic type parameter with trait bounds.
- we could have defined the Screen struct using a generic type and a trait bound 

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

### Implementing the Trait

- We’ll add some types that implement the Draw trait

```rust
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

- If someone using our library decides to implement a SelectBox struct that has width, height, and options fields, they implement the Draw trait on the SelectBox type as well

```rust
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
```

- Our library’s user can now write their main function to create a Screen instance
- To the Screen instance, they can add a SelectBox and a Button by putting each in a Box<T> to become a trait object
- By specifying Box<dyn Draw> as the type of the values in the components vector, we’ve defined Screen to need values that we can call the draw method on

```rust
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

- The advantage of using trait objects and Rust’s type system to write code similar to code using duck typing is that we never have to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn’t implement a method but we call it anyway
- Rust won’t compile our code if the values don’t implement the traits that the trait objects need

```rust
use gui::Screen;

fn main() {
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
    };

    screen.run();
}
```

### Trait Objects Perform Dynamic Dispatch

- When we use trait objects, Rust must use dynamic dispatch
- The compiler doesn’t know all the types that might be used with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call
- Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call
- Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations
- However, we did get extra flexibility in the code

## Implementing an Object-Oriented Design Pattern

- The state pattern is an object-oriented design pattern
- The states are represented by a set of state objects, and the value’s behavior changes based on its state
- First, we’re going to implement the state pattern in a more traditional object-oriented way, then we’ll use an approach that’s a bit more natural in Rust
- The final functionality will look like this:
  - A blog post starts as an empty draft.
  - When the draft is done, a review of the post is requested.
  - When the post is approved, it gets published.
  - Only published blog posts return content to print, so unapproved posts can’t accidentally be published.
- This workflow in code form

```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

### Defining Post and Creating a New Instance in the Draft State

- We’ll start with the definition of the struct and an associated public new function to create an instance of Post
- We’ll also make a private State trait
- Post will hold a trait object of Box<dyn State> inside an Option<T> in a private field named state to hold the state object
-  The state objects are Draft,
- The state objects are Draft, PendingReview, and Published, and they will all implement the State trait

```rust
// src/lib.rs
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

### Storing the Text of the Post Content

- We implement add_text method, rather than exposing the content field as pub
- The add_text method takes a mutable reference to self, because we’re changing the Post instance that we’re calling add_text on
- We then call push_str on the String in content and pass the text argument to add to the saved content

```rust
impl Post {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

### Ensuring the Content of a Draft Post Is Empty

- We still want the content method to return an empty string slice because the post is still in the draft state
- Let’s implement the content method with the simplest thing that will fulfill this requirement: always returning an empty string slice
- We’ll change this later once we implement the ability to change a post’s state so it can be published

```rust
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }
}
```

### Requesting a Review of the Post Changes Its State

- We need to add functionality to request a review of a post, which should change its state from Draft to PendingReview
- We give Post a public method named request_review that will take a mutable reference to self
- Then we call an internal request_review method on the current state of Post, and this second request_review method consumes the current state and returns a new state
- We add the request_review method to the State trait; all types that implement the trait will now need to implement the request_review method
- Note that rather than having self, &self, or &mut self as the first parameter of the method, we have self: Box<Self>. This syntax means the method is only valid when called on a Box holding the type
- This syntax takes ownership of Box<Self>, invalidating the old state so the state value of the Post can transform into a new state

```rust
impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        // To consume the old state, the request_review method needs to take ownership of the state value. This is where the Option in the state field of Post comes in: we call the take method to take the Some value out of the state field and leave a None in its place
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // The request_review method on Draft returns a new, boxed instance of a new PendingReview struct, which represents the state when a post is waiting for a review
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
   // The PendingReview struct also implements the request_review method but doesn’t do any transformations. Rather, it returns itself, because when we request a review on a post already in the PendingReview state, it should stay in the PendingReview state.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

- Now we can start seeing the advantages of the state pattern: the request_review method on Post is the same no matter its state value. Each state is responsible for its own rules

### Adding approve to Change the Behavior of content

- The approve method will be similar to the request_review method: it will set state to the value that the current state says it should have when that state is approved
- We add the approve method to the State trait and add a new struct that implements State, the Published state.

```rust
impl Post {
    // --snip--
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // --snip--
    // Similar to the way request_review on PendingReview works, if we call the approve method on a Draft, it will have no effect because approve will return self. When we call approve on PendingReview, it returns a new, boxed instance of the Published struct
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

// Published struct implements the State trait, and for both the request_review method and the approve method, it returns itself, because the post should stay in the Published state in those cases
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

- Now we need to update the content method on Post
- We call a content method on the value in state and pass the post instance (that is, self) as an argument. Then we return the value that’s returned from using the content method on the state value
- We call the `as_ref` method on the Option because `we want a reference to the value inside the Option rather than ownership of the value`. Because state is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned. If we didn’t call as_ref, we would get an error because we can’t move state out of the borrowed &self of the function parameter
- We then call the unwrap method, which we know will never panic, because we know the methods on Post ensure that state will always contain a Some value when those methods are done

```rust
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // --snip--
}
```

- When we call content on the &Box<dyn State>, deref coercion will take effect on the & and the Box so the content method will ultimately be called on the type that implements the State trait
- We add a default implementation for the content method that returns an empty string slice. That means we don’t need to implement content on the Draft and PendingReview structs
- The Published struct will override the content method and return the value in post.content
- Note that we need lifetime annotations on this method. We’re taking a reference to a post as an argument and returning a reference to part of that post, so the lifetime of the returned reference is related to the lifetime of the post argument.

```rust
trait State {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

### Trade-offs of the State Pattern

- We’ve shown that Rust is capable of implementing the object-oriented state pattern to encapsulate the different kinds of behavior a post should have in each state
- The methods on Post know nothing about the various behaviors
- If we were to create an alternative implementation that didn’t use the state pattern, we might instead use match expressions in the methods on Post or even in the main code that checks the state of the post and changes behavior in those places
- The implementation using the state pattern is easy to extend to add more functionality
- One downside of the state pattern is that, because the states implement the transitions between states, some of the states are coupled to each other
- Another downside is that we’ve duplicated some logic. To eliminate some of the duplication, we might try to make default implementations for the request_review and approve methods on the State trait that return self;
- By implementing the state pattern exactly as it’s defined for object-oriented languages, we’re not taking as full advantage of Rust’s strengths as we could

### Encoding States and Behavior as Types

- We’ll show you how to rethink the state pattern to get a different set of trade-offs. Rather than encapsulating the states and transitions completely so outside code has no knowledge of them, we’ll encode the states into different types

```rust
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}
```

- We still enable the creation of new posts in the draft state using Post::new and the ability to add text to the post’s content. But instead of having a content method on a draft post that returns an empty string, we’ll make it so draft posts don’t have the content method at all
- Both the Post and DraftPost structs have a private content field that stores the blog post text. 
- The structs no longer have the state field because we’re moving the encoding of the state to the `types of the structs`
- The Post struct will represent a published post, and it has a content method that returns the content.

```rust
pub struct Post {
    content: String,
}

// DraftPost does not have a content method defined! So now the program ensures all posts start as draft posts
pub struct DraftPost {
    content: String,
}

impl Post {
    // We still have a Post::new function, but instead of returning an instance of Post, it returns an instance of DraftPost
    pub fn new() -> DraftPost {
        // Because content is private and there aren’t any functions that return Post, it’s not possible to create an instance of Post right now
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

### Implementing Transitions as Transformations into Different Types

- We want to enforce the rule that a draft post has to be reviewed and approved before it can be published

```rust
impl DraftPost {
    // --snip--
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    // The request_review and approve methods take ownership of self, thus consuming the DraftPost and PendingReviewPost instances and transforming them into a PendingReviewPost and a published Post
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

- We also have to make some small changes to main. The request_review and approve methods return new instances rather than modifying the struct they’re called on, so we need to add more let post = shadowing assignments to save the returned instances
- The changes we needed to make to main to reassign post mean that this implementation doesn’t quite follow the object-oriented state pattern anymore: the transformations between the states are no longer encapsulated entirely within the Post implementation

```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```

# Patterns and Matching

- Patterns are a special syntax in Rust for matching against the structure of types, both complex and simple
- Using patterns in conjunction with match expressions and other constructs gives you more control over a program’s control flow
- A pattern consists of some combination of the following:
  - Literals
  - Destructured arrays, enums, structs, or tuples
  - Variables
  - Wildcards
  - Placeholders

## All the Places Patterns Can Be Used

### match Arms

- We use patterns in the arms of match expressions
## All the Places Patterns Can be Used

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

- The patterns in this match expression are the None and Some(i) on the left of each arrow
- The particular pattern _ will match anything, but it never binds to a variable, so it’s often used in the last match arm

```rust
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

### Conditional `if let` Expressions

- Use `if let` expressions mainly as a shorter way to write the equivalent of a match that only matches one case
- Optionally, `if let` can have a corresponding else containing code to run if the pattern in the if let doesn’t match
- Possible to mix and match if let, else if, and else if let expressions
- Doing so gives us more flexibility than a match expression in which we can express only one value to compare with the patterns

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

### `while let` Conditional Loops

- `while let` conditional loop allows a while loop to run for as long as a pattern continues to match
- We code a while let loop that uses a vector as a stack and prints the values in the vector in the opposite order in which they were pushed

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### `for` Loops

- In a for loop, the value that directly follows the keyword for is a pattern. For example, in for x in y the x is the pattern

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

### `let` Statements

- Variable assignment with let

```rust
let x = 5;
```

- Every time you've used a let statement like this you've been using patterns, although you might not have realized it! More formally, a let statement looks like this
- Pattern that means “bind what matches here to the variable x.”

```rust
let PATTERN = EXPRESSION;
```

- Uses a pattern with let to destructure a tuple.
- We match a tuple against a pattern. Rust compares the value (1, 2, 3) to the pattern (x, y, z) and sees that the value matches the pattern
- If the number of elements in the pattern doesn’t match the number of elements in the tuple, the overall type won’t match and we’ll get a compiler error

```rust
let (x, y, z) = (1, 2, 3);
```

### Function Parameters

- Function parameters can also be patterns
- Which declares a function named foo that takes one parameter named x of type i32. The x part is a pattern! As we did with let, we could match a tuple in a function’s arguments to the pattern

```rust
fn foo(x: i32) {
    // code goes here
}
```

- The values &(3, 5) match the pattern &(x, y), so x is the value 3 and y is the value 5
- We can also use patterns in closure parameter lists in the same way as in function parameter lists, because closures are similar to functions

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```
## Refutability: Whether a Pattern Might Fail to Match

- Patterns come in two forms: `refutable` and `irrefutable`
- Patterns that will match for any possible value passed are `irrefutable`. An example would be x in the statement let x = 5; because x matches anything and therefore cannot fail to match
- Patterns that can fail to match for some possible value are `refutable`. An example would be Some(x) in the expression if let Some(x) = a_value because if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match
- Function parameters, let statements, and for loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match.
- The if let and while let expressions accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns because by definition they’re intended to handle possible failure
- If we have a refutable pattern where an irrefutable pattern is needed, we can fix it by changing the code that uses the pattern: instead of using let, we can use if let

```rust
let Some(x) = some_option_value;
    ^^^^^^^ pattern `None` not covered

if let Some(x) = some_option_value {
    println!("{}", x);
}

if let x = 5 {
   ^^^^^^^^^ this pattern will always match, so the `if let` is useles
    println!("{}", x);
};
```

## Pattern Syntax

- In this section, we gather all the syntax valid in patterns and discuss why and when you might want to use each one.

### Matching Literals

- You can match patterns against literals directly. The following code gives some examples:

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

### Matching Named Variables

- Named variables are irrefutable patterns that match any value
- There is a complication when you use named variables in match expressions. Because match starts a new scope, variables declared as part of a pattern inside the match expression will shadow those with the same name outside the match construct, as is the case with all variables

```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    // The pattern in the second match arm introduces a new variable named y that will match 
    // any value inside a Some value. Because we’re in a new scope inside the match expression, 
    // this is a new y variable, not the y we declared at the beginning with the value 10. 
    // This new y binding will match any value inside a Some
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
}

println!("at the end: x = {:?}, y = {y}", x, y);

// > Matched, y = 5
// > at the end: x = Some(5), y = 10
```

### Multiple Patterns

- In match expressions, you can match multiple patterns using the | syntax, which is the pattern or operator

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

### Matching Ranges of Values with ..=

- The ..= syntax allows us to match to an inclusive range of values
- If x is 1, 2, 3, 4, or 5, the first arm will match. This syntax is more convenient for multiple match values

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

- The compiler checks that the range isn’t empty at compile time, and because the only types for which Rust can tell if a range is empty or not are char and numeric values, ranges are only allowed with numeric or char values.

```rust
let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

### Destructuring to Break Apart Values

- We can also use patterns to destructure structs, enums, and tuples to use different parts of these values. Let’s walk through each value

#### Destructuring Structs

- Point struct with two fields, x and y, that we can break apart using a pattern with a let statement.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    // This code creates the variables a and b that match the values of the 
    // x and y fields of the p struct.
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    // This code creates the variables x and y that match the x and y fields of the p variable
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

- We have a match expression that separates Point values into three cases: points that lie directly on the x axis (which is true when y = 0), on the y axis (x = 0), or neither

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        // The second arm matches any point on the y axis by specifying that 
        // the x field matches if its value is 0 and creates a variable y for 
        // the value of the y field
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

// > On the y axis at 7
```

#### Destructuring Enums

- We've destructured enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}

// > Change the color to red 0, green 160, and blue 255
```

#### Destructuring Nested Structs and Enums

- Matching can work on nested items too
- The pattern of the first arm in the match expression matches a Message::ChangeColor enum variant that contains a Color::Rgb variant

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}
```

#### Destructuring Structs and Tuples

- We can mix, match, and nest destructuring patterns in even more complex ways. The following example shows a complicated destructure where we nest structs and tuples inside a tuple and destructure all the primitive values out

```rust
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

### Ignoring Values in a Pattern

- There are a few ways to ignore entire values or parts of values in a pattern: using the _ pattern (which you’ve seen), using the _ pattern within another pattern, using a name that starts with an underscore, or using .. to ignore remaining parts of a value

#### Ignoring an Entire Value with _

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

#### Ignoring Parts of a Value with a Nested _

- We can also use _ inside another pattern to ignore just part of a value

```rust
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);

// > Can't overwrite an existing customized value
// > setting is Some(5)
```

- We can also use underscores in multiple places within one pattern to ignore particular values

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {first}, {third}, {fifth}")
    }
}

// > Some numbers: 2, 8, 32
```

#### Ignoring an Unused Variable by Starting Its Name with _

- You can tell Rust not to warn you about the unused variable by starting the name of the variable with an underscore

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

- Using the underscore by itself doesn’t ever bind to the value. this code will compile without any errors because s doesn’t get moved into _

```rust
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
  ```

#### Ignoring Remaining Parts of a Value with ..

- With values that have many parts, we can use the .. syntax to use specific parts and ignore the rest, avoiding the need to list underscores for each ignored value

```rust
 struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

- The syntax .. will expand to as many values as it needs to be. The code shows how to use .. with a tuple.
- In this code, the first and last value are matched with first and last. The .. will match and ignore everything in the middle

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
```

- However, using .. must be unambiguous. If it is unclear which values are intended for matching and which should be ignored, Rust will give us an error

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
         |           ^^ can only be used once per tuple pattern
         | previously used here
            println!("Some numbers: {}", second)
        },
    }
}
```

### Extra Conditionals with Match Guards

- `A match guard is an additional` if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen
- Match guards are useful for expressing more complex ideas than a pattern alone allows

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

// > Default case, x = Some(5)
```

- You can also use the or operator | in a match guard to specify multiple patterns
- The match condition states that the arm only matches `if the value of x is equal to 4, 5, or 6 and if y is true`
  - The match guard in relation to a pattern behaves like this: `(4 | 5 | 6) if y => ...`

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

### @ Bindings

- The at operator @ lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello {
        // By specifying id_variable @ before the range 3..=7, we’re capturing whatever 
        // value matched the range while also testing that the value matched the range pattern
        id: id_variable @ 3..=7,
    } => {
        // We also want to bind the value to the variable id_variable so we can use it 
        // in the code associated with the arm
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    }
    Message::Hello { id } => println!("Found some other id: {}", id),
}

// > Found an id in range: 5
```

# Advanced Features

- In this chapter, we’ll cover:
  - `Unsafe Rust`: how to opt out of some of Rust’s guarantees and take responsibility for manually upholding those guarantees
  - `Advanced traits`: associated types, default type parameters, fully qualified syntax, supertraits, and the newtype pattern in relation to traits
  - `Advanced types`: more about the newtype pattern, type aliases, the never type, and dynamically sized types
  - `Advanced functions and closures`: function pointers and returning closures
  - `Macros`: ways to define code that defines more code at compile time

## Unsafe Rust

- Unsafe Rust exists because, by nature, static analysis is conservative. When the compiler tries to determine whether or not code upholds the guarantees, it’s better for it to reject some valid programs than to accept some invalid programs
- Rust has an unsafe alter ego is that the underlying computer hardware is inherently unsafe

### Unsafe Superpowers

- use the `unsafe` keyword and then start a new block that holds the unsafe code
- Those superpowers include the ability to:
  - Dereference a raw pointer
  - Call an unsafe function or method
  - Access or modify a mutable static variable
  - Implement an unsafe trait
  - Access fields of unions
- It’s important to understand that unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety checks: if you use a reference in unsafe code, it will still be checked
- The unsafe keyword only gives you access to these five features that are then not checked by the compiler for memory safety
- To isolate unsafe code as much as possible, it’s best to enclose unsafe code within a safe abstraction and provide a safe API

### Dereferencing a Raw Pointer

- Unsafe Rust has two new types called raw pointers that are similar to references
- As with references, raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively
- The asterisk isn’t the dereference operator; it’s part of the type name
- In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced
- Different from references and smart pointers, raw pointers:
  - `Are allowed to ignore the borrowing rules` by having both immutable and mutable pointers or multiple mutable pointers to the same location
  - `Aren’t guaranteed` to point to valid memory
  - `Are allowed to be null`
  - `Don’t implement any automatic cleanup`
- We’ve created raw pointers by using `as` to cast an immutable and a mutable reference into their corresponding raw pointer types

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

- We’ll create a raw pointer whose validity we can’t be so certain of
- Trying to use arbitrary memory is undefined: there might be data at that address or there might not, the compiler might optimize the code so there is no memory access, or the program might error with a segmentation fault

```rust
let address = 0x012345usize;
let r = address as *const i32;
```

- We can create raw pointers in safe code, but we can’t dereference raw pointers and read the data being pointed to. In the code below, we use the dereference operator * on a raw pointer that requires an unsafe block.
- With all of these dangers, why would you ever use raw pointers? One major use case is when interfacing with C code

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

### Calling an Unsafe Function or Method

- Unsafe functions and methods look exactly like regular functions and methods, but they have an extra unsafe before the rest of the definition
- The unsafe keyword in this context indicates the function has requirements we need to uphold when we call this function, because Rust can’t guarantee we’ve met these requirements
- We must call `the dangerous function within a separate unsafe block`. If we try to call dangerous `without the unsafe block`, we’ll get an error
- Bodies of unsafe functions are `effectively unsafe blocks`, so to perform other unsafe operations within an unsafe function, `we don’t need to add another unsafe block`

```rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

### Creating a Safe Abstraction over Unsafe Code

- Fu\nction contains unsafe code doesn’t mean we need to mark the entire function as unsafe. In fact, wrapping unsafe code in a safe function is a common abstraction
- We can’t implement this function using only safe Rust

```rust
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..])
    --------------------------^^^^^^--------
    |     |     |                   |
    |     |     |                   second mutable borrow occurs here
    |     |     first mutable borrow occurs here
    |     returning this value requires that `*values` is borrowed for `'1`
}
```

- Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice; it only knows that we’re borrowing from the same slice twice
- Borrowing different parts of a slice is fundamentally okay because the two slices aren’t overlapping, but Rust isn’t smart enough to know this. When we know code is okay, but Rust doesn’t, `it’s time to reach for unsafe code`

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // The function `slice::from_raw_parts_mut is unsafe` because `it takes a raw pointer` and must `trust that this pointer is valid`
            slice::from_raw_parts_mut(ptr, mid),
            // The add method on raw pointers is also unsafe, because it must trust that the offset location is also a valid pointer
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

- In contrast, the use of slice::from_raw_parts_mut in the code below would likely crash when the slice is used. 

```rust
use std::slice;

let address = 0x01234usize;
let r = address as *mut i32;

let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
```

#### Using extern Functions to Call External Code

- Rust code might need to interact with code written in another language
- Rust has the keyword extern that facilitates the creation and use of a Foreign Function Interface (FFI)
- Functions declared within extern blocks are always unsafe to call from Rust code

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

- The "C" part defines which application binary interface (ABI) the external function uses: the ABI defines how to call the function at the assembly level. The "C" ABI is the most common and follows the C programming language’s ABI

#### Calling Rust Functions from Other Languages

- We can also use extern to create an interface that allows other languages to call Rust functions. Instead of creating a whole extern block, we add the extern keyword and specify the ABI to use just before the fn keyword for the relevant function

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

### Accessing or Modifying a Mutable Static Variable

- In Rust, global variables are called static variables
- Static variables can only store references with the 'static lifetime, which means the Rust compiler can figure out the lifetime and we aren’t required to annotate it explicitly
- Accessing an immutable static variable is safe
- A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory.
- Using the value will always access the same data. Constants, on the other hand, are allowed to duplicate their data whenever they’re used. Another difference is that static variables can be mutable

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

- Accessing and modifying `mutable static variables is unsafe`
- As with regular variables, we specify mutability using the mut keyword. Any code that reads or writes from COUNTER must be within an unsafe block

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

### Implementing an Unsafe Trait

- We can use unsafe to implement an unsafe trait
- We declare that a trait is unsafe by adding the unsafe keyword before trait and marking the implementation of the trait as unsafe too
- By using unsafe impl, we’re promising that we’ll uphold the invariants that the compiler can’t verify

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

### Accessing Fields of a Union

- The final action that works only with unsafe is accessing fields of a union.
- A union is similar to a struct, but only one declared field is used in a particular instance at one time
- Unions are primarily used to interface with unions in C code. Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance

### When to Use Unsafe Code

- Using unsafe to take one of the five actions (superpowers) just discussed isn’t wrong or even frowned upon
- But it is trickier to get unsafe code correct because the compiler can’t help uphold memory safety
- When you have a reason to use unsafe code, you can do so, and having the explicit unsafe annotation makes it easier to track down the source of problems when they occur.

## Advanced Traits

### Specifying Placeholder Types in Trait Definitions with Associated Types

- Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures
- The associated type is named Item and stands in for the type of the values the type implementing the Iterator trait is iterating over

```rust
pub trait Iterator {
    // The type Item is a placeholder
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    // implementation of the Iterator trait on a type named Counter that specifies the Item type is u32
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
```

- The difference is that when using `generics`, we must annotate the types in each implementation
- We can also implement Iterator<String> for Counter or any other type, we could have multiple implementations of Iterator for Counter
- In other words, when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time
- With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times
- Associated types also become part of the trait’s contract

### Default Generic Type Parameters and Operator Overloading

- Rust doesn’t allow you to create your own operators or overload arbitrary operators

```rust
// You can overload the operations and corresponding traits listed in std::ops by implementing the traits associated with the operator
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// We do this by implementing the Add trait on a Point struct:
impl Add for Point {
    type Output = Point;
    // Overload the + operator to add two Point instances together
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

- The default generic type in this code is within the Add trait. Here is its definition:

```rust
// Rhs=Self: this syntax is called default type parameters
trait Add<Rhs=Self> {
    type Output;

    // The Rhs generic type parameter (short for “right hand side”) defines the type of the rhs parameter in the add method
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

- We have two structs, Millimeters and Meters, holding values in different units. This thin wrapping of an existing type in another struct is known as the newtype pattern
- We want to add values in millimeters to values in meters and have the implementation of Add do the conversion correctly. We can implement Add for Millimeters with Meters as the Rhs

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

- The first purpose is similar to the second but in reverse: if you want to add a type parameter to an existing trait, you can give it a default to allow extension of the functionality of the trait without breaking the existing implementation code.

### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

- Nothing in Rust prevents a trait from having a method with the same name as another trait’s method, nor does Rust prevent you from implementing both traits on one type
-  It’s also possible to implement a method directly on the type with the same name as methods from traits

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

- When we call fly on an instance of Human, the compiler defaults to calling the method that is directly implemented on the type

```rust
fn main() {
    let person = Human;
    person.fly();
}
```

- To call the fly methods from either the Pilot trait or the Wizard trait, we need to use more explicit syntax to specify which fly method we mean
- Specifying the trait name before the method name clarifies to Rust which implementation of fly we want to call

```rust
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

- However, associated functions that are not methods don’t have a self parameter

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}

// > A baby dog is called a Spot
```

- Rust with a type annotation within the angle brackets, which indicates we want to call the baby_name method from the Animal trait as implemented on Dog by saying that we want to treat the Dog type as an Animal for this function call

```rust
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// > A baby dog is called a puppy
```

- In general, fully qualified syntax is defined as follows:

```rust
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

### Using Supertraits to Require One Trait’s Functionality Within Another Trait

- In the implementation of the outline_print method, we want to use the Display trait’s functionality
- We can do that in the trait definition by specifying OutlinePrint: Display. This technique is similar to adding a trait bound to the trait.

```rust
use std::fmt;

// We’ve specified that OutlinePrint requires the Display trait, we can use the to_string function that is automatically implemented for any type that implements Display
trait OutlinePrint: fmt::Display {
                    ^^^^^^^^^^^^ required by this bound in `OutlinePrint`
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
     ^^^^^^^^^^^^ `Point` cannot be formatted with the default formatter
```

- To fix this, we implement Display on Point and satisfy the constraint that OutlinePrint requires

```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

### Using the Newtype Pattern to Implement External Traits on External Types

- The orphan rule that states we’re only allowed to implement a trait on a type if either the trait or the type are local to our crate. It’s possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct
- Newtype is a term that originates from the Haskell programming language. There is no runtime performance penalty for using this pattern, and the wrapper type is elided at compile time
- We want to implement Display on Vec<T>, which the orphan rule prevents us from doing directly because the Display trait and the Vec<T> type are defined outside our crate. We can make a Wrapper struct that holds an instance of Vec<T>; then we can implement Display on Wrapper and use the Vec<T> value

```rust
use std::fmt;

// Wrapper is a new type
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // uses self.0 to access the inner Vec<T>, because Wrapper is a tuple struct and Vec<T> is the item at index 0 in the tuple
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

- This newtype pattern is also useful even when traits are not involved

## Advanced Types

### Using the Newtype Pattern for Type Safety and Abstraction

- The newtype pattern is also useful for tasks beyond those we’ve discussed so far, including statically enforcing that values are never confused and indicating the units of a value

- We can also use the newtype pattern to abstract away some implementation details of a type: the new type can expose a public API that is different from the API of the private inner type.

- Newtypes can also hide internal implementation

### Creating Type Synonyms with Type Aliases

- Rust provides the ability to declare a type alias to give an existing type another name. For this we use the `type` keyword

```rust
type Kilometers = i32;
```

- Kilometers is not a separate, new type. Values that have the type Kilometers will be treated the same as values of type i32:

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

- The main use case for type synonyms is to reduce repetition. For example, we might have a lengthy type like this:

```rust
Box<dyn Fn() + Send + 'static>
```

- A type alias makes this code more manageable by reducing the repetition

```rust
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
fn takes_long_type(f: Thunk) {
    // --snip--
}

// fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
fn returns_long_type() -> Thunk {
    // --snip--
}
```

- Type aliases are also commonly used with the Result<T, E> type for reducing repetition

```rust
// Because this declaration is in the std::io module, we can use the fully qualified alias std::io::Result<T>; that is, a Result<T, E> with the E filled in as std::io::Error
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    // fn flush(&mut self) -> Result<(), Error>;
    fn flush(&mut self) -> Result<()>;

    // fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    // fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```

### The Never Type that Never Returns

- Rust has a special type named `!` that’s known in type theory lingo as the empty type because it has no values
- We prefer to call it the never type because it stands in the place of the return type when a function will never return
- This code is read as “the function bar returns never.”

```rust
fn bar() -> ! {
    // --snip--
}
```

- continue has a `!` value. That is, when Rust computes the type of guess, it looks at both match arms, the former with a value of u32 and the latter with a ! value. Because ! can never have a value, `Rust decides that the type of guess is u32`

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

- The never type is useful with the panic! macro as well. Recall the unwrap function that we call on Option<T> values to produce a value or panic with this definition
- Rust sees that val has the type T and panic! has the type !, so the result of the overall match expression is T

```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            // This code works because panic! doesn’t produce a value; it ends the program
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

- One final expression that has the type ! is a loop
- The loop never ends, so ! is the value of the expression. However, this wouldn’t be true if we included a break, because the loop would terminate when it got to the break.
```rust
print!("forever ");

loop {
    print!("and ever ");
}
```

### Dynamically Sized Types(DST) and the Sized Trait

- To work with DSTs, Rust provides the Sized trait to determine whether or not a type’s size is known at compile time
- This trait is automatically implemented for everything whose size is known at compile time
- Rust implicitly adds a bound on Sized to every generic function

```rust
// is actually treated as though we had written this, fn generic<T: Sized>(t: T) {
fn generic<T>(t: T) {
    // --snip--
}
```

- By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction
- A trait bound on ?Sized means “T may or may not be Sized”

```rust
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

## Advanced Functions & Closures

### Function Pointers

- You can also pass regular functions to functions
- This technique is useful when you want to pass a function you’ve already defined rather than defining a new closure
- Functions coerce to the type `fn` (with a lowercase f),
- The `fn` type is called a function pointer
- Passing functions with function pointers will allow you to use functions as arguments to other functions

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

// > The answer is: 12
```

- Unlike closures, fn is a type rather than a trait, so we specify fn as the parameter type directly rather than declaring a generic type parameter with one of the Fn traits as a trait bound
- Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), meaning you can always pass a function pointer as an argument for a function that expects a closure
- As an example of where you could use either a closure defined inline or a named function

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> =
    // We could use a closure, like this:
    list_of_numbers.iter().map(|i| i.to_string()).collect();

let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> =
    // We could name a function as the argument to map instead of the closure, like this:
    list_of_numbers.iter().map(ToString::to_string).collect();
```

- We can use these initializer functions as function pointers that implement the closure traits, which means we can specify the initializer functions as arguments for methods that take closures

```rust
enum Status {
    Value(u32),
    Stop,
}

// We create Status::Value instances using each u32 value in the range that map is called on by using the initializer function of Status::Value
let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```

### Returning Closures

- Closures are represented by traits, which means `you can’t return closures directly`

```rust
fn returns_closure() -> dyn Fn(i32) -> i32 {
                        ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |x| x + 1
}
```

- In most cases where you might want to return a trait, `you can instead use the concrete type that implements the trait as the return value of the function`. However, you can’t do that with closures because `they don’t have a concrete type that is returnable`; `you’re not allowed to use the function pointer fn as a return type`

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

## Macros

- We’ve used macros like println!
- The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:

- `Custom #[derive] macros` that specify code added with the derive attribute used on structs and enums
- `Attribute-like macros` that define custom attributes usable on any item
- `Function-like macros` that look like function calls but operate on the tokens specified as their argument

### The Difference Between Macros and Functions

- Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming
- We’ve also used the `println! and vec!` macros throughout the book. All of these macros expand to `produce more code than the code you’ve written manually`
- Metaprogramming is `useful for reducing the amount of code` you have to write and maintain, which is also one of the roles of functions. However, macros have some additional powers that functions don’t.
- A function signature must declare the number and type of parameters the function has. `Macros, on the other hand, can take a variable number of parameters`
- The downside to implementing a macro instead of a function is that macro definitions are more complex than function definitions because you’re writing Rust code that writes Rust code. Due to this indirection, macro definitions are `generally more difficult to read, understand, and maintain than function definitions`

### Declarative Macros with macro_rules! for General Metaprogramming

- The most widely used form of macros in Rust is the declarative macro
- These are also sometimes referred to as “macros by example,” “macro_rules! macros,” or just plain “macros.”
- To define a macro, you use the macro_rules! construct

```rust
// The #[macro_export] annotation indicates that this macro
#[macro_export]
// We then start the macro definition with macro_rules! and the name of the macro we’re defining without the exclamation mark
macro_rules! vec {
    // We have one arm with the pattern ( $( $x:expr ),* ), followed by => and the block of code associated with this pattern. If the pattern matches, the associated block of code will be emitted
    // First, we use a set of parentheses to encompass the whole pattern. We use a dollar sign ($) to declare a variable in the macro system that will contain the Rust code matching the pattern
    ( $( 
      // Matches any Rust expression and gives the expression the name $x
      $x:expr 
      // Literal comma separator character could optionally appear after the code that matches the code in $(). The * specifies that the pattern matches zero or more of whatever precedes the *
    ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// When we call this macro with vec![1, 2, 3];, the $x pattern matches three times with the three expressions 1, 2, and 3.
let v: Vec<u32> = vec![1, 2, 3];
```

### Procedural Macros for Generating Code from Attributes

- The second form of macros is the procedural macro, which acts more like a function (and is a type of procedure)
-  Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do
- The three kinds of procedural macros are custom derive, attribute-like, and function-like, and all work in a similar fashion
- When creating procedural macros, the definitions must reside in their own crate with a special crate type

```rust
use proc_macro;

// we show how to define a procedural macro, where some_attribute is a placeholder for using a specific macro variety
#[some_attribute]
// The TokenStream type is defined by the proc_macro crate
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

### How to Write a Custom derive Macro

- The first step is to make a new library crate

```sh
$ cargo new hello_macro --lib
```

- Next, we’ll define the HelloMacro trait and its associated function:

```rust
// src/lib.rs
pub trait HelloMacro {
    fn hello_macro();
}
```

- The next step is to define the procedural macro. Let’s start a new crate called hello_macro_derive inside our hello_macro project

```sh
$ cargo new hello_macro_derive --lib
```

- Our two crates are tightly related, so we create the procedural macro crate within the directory of our hello_macro crate.  If we change the trait definition in hello_macro, we’ll have to change the implementation of the procedural macro in hello_macro_derive as well
- The two crates will need to be published separately, and programmers using these crates will need to add both as dependencies and bring them both into scope
- We need to declare the hello_macro_derive crate as a procedural macro crate. We’ll also need functionality from the syn and quote crates, as you’ll see in a moment, so we need to add them as dependencies

```toml
# hello_macro_derive/Cargo.toml
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```

- To start defining the procedural macro
- Split the code into the hello_macro_derive function, which is responsible for parsing the TokenStream, and the impl_hello_macro function, which is responsible for transforming the syntax tree: this makes writing a procedural macro more convenient
- The code in the outer function (hello_macro_derive in this case) will be the same for almost every procedural macro crate you see or create. The code you specify in the body of the inner function (impl_hello_macro in this case) will be different depending on your procedural macro’s purpose

```rust
// hello_macro_derive/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    // Calling unwrap to cause the hello_macro_derive function to panic if the call to the syn::parse function fails here
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}
```

- We’ve introduced three new crates: proc_macro, syn, and quote. The proc_macro crate comes with Rust
  - `syn` crate parses Rust code from a string into a data structure that we can perform operations on
  - `quote` crate turns syn data structures back into Rust code
- The hello_macro_derive function first converts the input from a TokenStream to a data structure that we can then interpret and perform operations on
- This is where syn comes into play. The parse function in syn takes a TokenStream and returns a DeriveInput struct representing the parsed Rust code
- Shows the relevant parts of the DeriveInput struct we get from parsing the struct Pancakes; string:

```
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

- Define the impl_hello_macro function, which is where we’ll build the new Rust code we want to include

```rust
// hello_macro_derive/src/lib.rs
// The returned TokenStream is added to the code that our crate users write, so when they compile their crate, they’ll get the extra functionality that we provide in the modified TokenStream
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // Ident struct instance containing the name (identifier) of the annotated type using ast.ident
    // ident field with a value of "Pancakes"
    let name = &ast.ident;
    // quote! macro lets us define the Rust code that we want to return
    let gen = quote! {
        // quote! macro also provides some very cool templating mechanics: we can enter #name, and quote! will replace it with the value in the variable name
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! macro used here is built into Rust. It takes a Rust expression, such as 1 + 2, and at compile time turns the expression into a string literal, such as "1 + 2"
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

- We need to add hello_macro and hello_macro_derive as dependencies in the pancakes crate’s Cargo.toml

```toml
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

### Attribute-like macros

- Attribute-like macros are similar to custom derive macros, but instead of generating code for the derive attribute, they allow you to create new attributes
- They’re also more flexible: derive only works for structs and enums; attributes can be applied to other items as well, such as functions
- #[route] attribute would be defined by the framework as a procedural macro

```rust
#[route(GET, "/")]
fn index() {
```

- The signature of the macro definition function would look like this
- we have two parameters of type TokenStream. The first is for the contents of the attribute: the GET, "/" part. The second is the body of the item the attribute is attached to: in this case, fn index() {} and the rest of the function’s body

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

- Other than that, attribute-like macros work the same way as custom derive macros: you create a crate with the proc-macro crate type and implement a function that generates the code you want

### Function-like macros

- Function-like macros define macros that look like function calls. Similarly to `macro_rules!` macros
- Can take an `unknown number of arguments`
- Function-like macros take a TokenStream parameter and their definition manipulates that TokenStream using Rust code as the other two types of procedural macros do

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

- The sql! macro would be defined like this:
- This definition is similar to the custom derive macro’s signature: we receive the tokens that are inside the parentheses and return the code we wanted to generate.

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

# Final Project: Building a Multithreaded Web Server

1. Learn a bit about TCP and HTTP.
2. Listen for TCP connections on a socket.
3. Parse a small number of HTTP requests.
4. Create a proper HTTP response.
5. Improve the throughput of our server with a thread pool.

## Building a Single-Threaded Web Server

### Listening to the TCP Connection

- Let’s make a new project in the usual fashion

```sh
cargo new hello
```

- Our web server needs to listen to a TCP connection, so that’s the first part we’ll work on

```rust
// src/main.rs
// The standard library offers a std::net module 
use std::net::TcpListener;

fn main() {
    // Using TcpListener, we can listen for TCP connections at the address 127.0.0.1:7878
    // The bind function in this scenario works like the new function in that it will return a new TcpListener instance
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // The incoming method on TcpListener returns an iterator that gives us a sequence of streams (more specifically, streams of type TcpStream)
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
```

### Reading the Request

- Let’s implement the functionality to read the request from the browser! 

```rust
use std:: {
    // Bring std::io::prelude and std::io::BufReader into scope to get access to traits and types that let us read from and write to the stream
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // we create a new BufReader instance that wraps a 
    // mutable reference to the stream
    let buf_reader = BufReader::new(&stream);
    // We create a variable named http_request to collect the lines of the request the browser sends to our server. We indicate that we want to collect these lines in a vector by adding the Vec<_> type annotation
    let http_reques: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    print!("Request: {:#?}", http_reques);
}
```

### A Closer Look at an HTTP Request

- HTTP is a text-based protocol, and a request takes this format:
- The first part of the request line indicates the method being used, such as GET or POST, which describes how the client is making this request. Our client used a GET request, which means it is asking for information.
- The next part of the request line is /, which indicates the Uniform Resource Identifier (URI) the client is requesting
- The last part is the HTTP version the client uses, and then the request line ends in a CRLF sequence

```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

### Writing a Response

- We’re going to implement sending data in response to a client request. Responses have the following format:

```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

- Here is an example response that uses HTTP version 1.1

```
HTTP/1.1 200 OK\r\n\r\n
```

- Printing the request data and replace it with the code 

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
```

### Returning Real HTML

- Let’s implement the functionality for returning more than a blank page
- reate the new file hello.html in the root of your project directory, not in the src directory

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
```

- we’ll modify handle_connection to read the HTML file

```rust
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
```

### Validating the Request and Selectively Responding

- Let’s add functionality to check that the browser is requesting / before returning the HTML file and return an error if the browser requests anything else. For this we need to modify handle_connection

```rust
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // The first unwrap takes care of the Option and stops the program if the iterator has no items. The second unwrap handles the Result and has the same effect as the unwrap that was in the map added in the next line
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        // some other request
    }
}
```

- Now let’s add the code in Listing 20-7 to the else block to return a response with the status code 404

```rust
// --snip--
} else {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = fs::read_to_string("404.html").unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
```

## A Touch of Refactoring

- Shows the resulting code after replacing the large if and else blocks
  - Start the server using cargo run
  - Then open two browser windows: one for http://127.0.0.1:7878/ and the other for http://127.0.0.1:7878/sleep
  - If you enter the / URI a few times, as before, you’ll see it respond quickly
  - But if you enter /sleep and then load /, you’ll see that / waits until sleep has slept for its full 5 seconds before loading.

```rust
// --snip--

fn handle_connection(mut stream: TcpStream) {
    // --snip--

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
```

## Turning Our Single-Threaded Server into a Multithreaded Server

### Simulating a Slow Request in the Current Server Implementation

- Implements handling a request to /sleep with a simulated slow response that will cause the server to sleep for 5 seconds before responding

```rust
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
// --snip--

fn handle_connection(mut stream: TcpStream) {
    // --snip--

    // We switched from if to match now that we have three cases
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // --snip--
}
```

### Improving Throughput with a Thread Pool

- `thread pool` is a group of spawned threads that are waiting and ready to handle a task
- When the program receives a new task, it assigns one of the threads in the pool to the task, and that thread will process the task
- Rather than spawning unlimited threads, then, we’ll have a fixed number of threads waiting in the pool
- This technique is just one of many ways to improve the throughput of a web server. Other options you might explore are the fork/join model, the single-threaded async I/O model, or the multi-threaded async I/O model

#### Spawning a Thread for Each Request

- First, let’s explore how our code might look if it did create a new thread for every connection

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // thread::spawn will create a new thread and then run the code in the closure in the new thread
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
```

#### Creating a Finite Number of Threads

- Shows the hypothetical interface for a ThreadPool struct we want to use instead of thread::spawn

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // ThreadPool::new to create a new thread pool with a configurable number of threads, in this case four
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // pool.execute has a similar interface as thread::spawn in that it takes a closure the pool should run for each stream
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
```

#### Building ThreadPool Using Compiler Driven Development

- Great! This error tells us we need a ThreadPool type or module, so we’ll build one now
- Our ThreadPool implementation will be independent of the kind of work our web server is doing
- Create a src/lib.rs that contains the following, which is the simplest definition of a ThreadPool struct that we can have for now:

```rust
// src/lib.rs
pub struct ThreadPool;
```

- Then edit main.rs file to bring ThreadPool into scope from the library crate by adding the following code to the top of src/main.rs:

```rust
// src/main.rs
use hello::ThreadPool;
```

- This error indicates that next we need to create an associated function named new for ThreadPool

```rust
// src/lib.rs
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }
}
```

- We’ll define the execute method on ThreadPool to take a closure as a parameter
- We can take closures as parameters with three different traits: Fn, FnMut, and FnOnce. We need to decide which kind of closure to use here. We know we’ll end up doing something similar to the standard library thread::spawn implementation
- Let’s create an execute method on ThreadPool that will take a generic parameter of type F with these bounds:

```rust
// src/lib.rs
impl ThreadPool {
    // --snip--
    pub fn execute<F>(&self, f: F)
    where
        // The F type parameter also has the trait bound Send and the lifetime bound 'static, which are useful in our situation: we need Send to transfer the closure from one thread to another and 'static because we don’t know how long the thread will take to execute
        F: FnOnce() + Send + 'static,
    {
    }
}
```

#### Validating the Number of Threads in new

- Let’s implement the bodies of these functions with the behavior we want. To start, , let’s think about new

```rust
impl ThreadPool {
    // We’ve also added some documentation for our ThreadPool with doc comments.
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // However, a pool with zero threads also makes no sense, yet zero is a perfectly valid usize
        assert!(size > 0);

        ThreadPool
    }

    // --snip--
}
```

#### Creating Space to Store the Threads

- The spawn function returns a JoinHandle<T>, where T is the type that the closure returns. Let’s try using JoinHandle too and see what happens

```rust
// We’ve brought std::thread into scope in the library crate, because we’re using thread::JoinHandle
use std::thread;

pub struct ThreadPool {
    // We’ve changed the definition of ThreadPool to hold a vector of thread::JoinHandle<()> instances, initialized the vector with a capacity of size, set up a for loop that will run some code to create the threads, and returned a ThreadPool instance containing them.
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool { threads }
    }
    // --snip--
}
```

#### A Worker Struct Responsible for Sending Code from the ThreadPool to a Thread

- We’ll implement this behavior by introducing a new data structure between the ThreadPool and the threads that will manage this new behavior. We’ll call this data structure `Worker`, `which is a common term in pooling implementations`
- The Worker picks up code that needs to be run and runs the code in the Worker’s thread
- Instead of storing a vector of JoinHandle<()> instances in the thread pool, we’ll store instances of the Worker struct. Each Worker will store a single JoinHandle<()> instance. Then we’ll implement a method on Worker that will take a closure of code to run and send it to the already running thread for execution

```rust
// src/lib.rs
use std::thread;

pub struct ThreadPool {
    // Change ThreadPool to hold a vector of Worker instances.
    workers: Vec<Worker>,
}

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        // In ThreadPool::new, use the for loop counter to generate an id, create a new Worker with that id, and store the worker in the vector
        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }
    // --snip--
}

// Define a Worker struct that holds an id and a JoinHandle<()>
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    // Define a Worker::new function that takes an id number and returns a Worker instance that holds the id and a thread spawned with an empty closure
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
```

#### Sending Requests to Threads via Channels

- We’ll use a channel to function as the queue of jobs, and execute will send a job from the ThreadPool to the Worker instances, which will send the job to its thread

```rust
// src/lib.rs
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// We’ll create a new Job struct that will hold the closures we want to send down the channel.
struct Job;

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // The code is trying to pass receiver to multiple Worker instances
        // The channel implementation that Rust provides is multiple producer, single consumer
        // This means we can’t just clone the consuming end of the channel
        // Share ownership across multiple threads and allow the threads to mutate the value, we need to use Arc<Mutex<T>>. The Arc type will let multiple workers own the receiver, and Mutex will ensure that only one worker gets a job from the receiver at a time
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    // --snip--
}

// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id, thread }
    }
}
```

#### Implementing the execute Method

- Let’s finally implement the execute method on ThreadPool. We’ll also change Job from a struct to a type alias for a trait object that holds the type of closure that execute receives

```rust
// src/lib.rs
// --snip--

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // --snip--

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // reating a new Job instance using the closure we get in execute
        let job = Box::new(f);
        // We send that job down the sending end of the channel.
        // We’re calling unwrap on send for the case that sending fails. This might happen if, for example, we stop all our threads from executing, meaning the receiving end has stopped receiving new messages. At the moment, we can’t stop our threads from executing: our threads continue executing as long as the pool exists
        self.sender.send(job).unwrap();
    }
}

/ --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Call lock on the receiver to acquire the mutex, and then we call unwrap to panic on any errors
            let job = receiver
              // Acquiring a lock might fail if the mutex is in a poisoned state, which can happen if some other thread panicked while holding the lock rather than releasing the lock. In this situation, calling unwrap to have this thread panic is the correct action to take
              .lock().unwrap()
              // If we get the lock on the mutex, we call recv to receive a Job from the channel. A final unwrap moves past any errors here as well, which might occur if the thread holding the sender has shut down, similar to how the send method returns Err if the receiver shuts down.
              .recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}
```

- This code compiles and runs but doesn’t result in the desired threading behavior: a slow request will still cause other requests to wait to be processed. The reason is somewhat subtle: the Mutex struct has no public unlock method because the ownership of the lock is based on the lifetime of the MutexGuard<T> within the LockResult<MutexGuard<T>> that the lock method returns

```rust
// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // our closure being passed to thread::spawn still only references the receiving end of the channel. Instead, we need the closure to loop forever, asking the receiving end of the channel for a job and running the job when it gets one
        let thread = thread::spawn(move || loop {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker { id, thread }
    }
}
```

## Graceful Shutdown and Cleanup

We’ll implement the Drop trait to call join on each of the threads in the pool so they can finish the requests they’re working on before closing. Then we’ll implement a way to tell the threads they should stop accepting new requests and shut down

### Implementing the Drop Trait on ThreadPool

- Let’s start with implementing Drop on our thread pool. `When the pool is dropped, our threads should all join to make sure they finish their work`

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
                          ^^^^ method not found in `Option<JoinHandle<()>>`
        }
    }
}
```

- We can’t call join because we only have a mutable borrow of each worker and join
- To solve this issue, we need to move the thread out of the Worker instance that owns thread so join can consume the thread

```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --snip--

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

- We mentioned earlier that we intended to call take on the Option value to move thread out of worker

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

### Signaling to the Threads to Stop Listening for Jobs

- However, the bad news is this code doesn’t function the way we want it to yet. The key is the logic in the closures run by the threads of the Worker instances: at the moment, we call join, but that won’t shut down the threads because they loop forever looking for jobs
- We’ll need a change in the ThreadPool drop implementation and then a change in the Worker loop

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    // We use the same Option and take technique as we did with the thread to be able to move sender out of ThreadPool
    // Dropping sender closes the channel, which indicates no more messages will be sent
    sender: Option<mpsc::Sender<Job>>,
}
// --snip--
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // --snip--

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

- We change the Worker loop to gracefully exit the loop in that case, which means the threads will finish when the ThreadPool drop implementation calls join on them

```rust
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

- Test graceful shutdown by modifying main to accept only two requests before gracefully shutting down the server

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // To see this code in action, let’s modify main to accept only two requests before gracefully shutting down the server
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
```





