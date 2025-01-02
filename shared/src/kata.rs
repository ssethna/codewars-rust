//!
//! Kata Struct, Level enum & Tag enum<br>
//! Used for categorizing the Kata the same way as in Codewars
//!

pub struct Kata {
    pub level: Level,
    pub tags: Vec<Tag>,
    pub description: String,
}

impl Kata {
    pub fn level(&self) -> &Level{
        &self.level
    }
}

#[derive(Debug)]
pub enum Level {
    L8kyu, L7kyu, L6kyu, L5kyu, L4kyu, L3kyu, L2kyu, L1kyu, 
}

#[derive(Debug)]
pub enum Tag {
    AsciiArt, Algebra, Algorithms, Arrays, BigIntegers, Binary, BinarySearchTrees,
    BinaryTrees, Bits, CellularAutomata, Ciphers, Combinatorics, Compilers,
    Cryptography, DataScience, DataStructures, Databases, DateTime, Debugging, 
    DesignPatterns, DiscreteMathematics, DynamicProgramming, EsotericLanguages,
    FunctionalProgramming, Fundamentals, GameSolvers, Games, Geometry, GraphTheory, 
    Interpreters, Iterators, LanguageFeatures, LinearAlgebra, Lists, Logic, 
    MachineLearning, Mathematics, Matrix, Memorization, Metaprogramming, Networks,
    NumberTheory, ObjectOrientedProgramming, Parsing, Performance, Permutations,
    Probibility, Puzzles, Queues, Recursion, Refactoring, Regex, Restricted,
    ReverseEngineering, Riddles, Scheduling, Searching, Security, Sets, Sorting,
    Stacks, Statistics, Strings, Trees, Tutorials,
}